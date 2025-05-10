pub mod net_mon {
    use pnet::datalink::Channel::Ethernet;
    use pnet::datalink::{self, NetworkInterface};
    use pnet::packet::ethernet::EthernetPacket;
    use pnet::packet::Packet;
    use tokio::sync::mpsc::{channel, Sender};
    use tokio::sync::Mutex;

    use log::{error, info, warn};
    use serde::Serialize;
    use std::sync::Arc;

    use crate::connection::C2;
    use crate::engine::Modules;

    #[derive(Serialize)]
    struct MalPacket {
        interface: String,
        packet: Vec<u8>,
        ioc: String,
    }

    #[derive(Debug, PartialEq, Eq)]
    enum NetMonError {
        ErrorCreatingDatalink,
        UnhandledChannel,
        ErrorReadingPacket,
    }

    pub async fn init(c2: Option<Arc<Mutex<C2>>>) -> Modules {
        let (tx, mut rx) = channel(10);

        for interface in datalink::interfaces() {
            info!("Capturing interface: {}", interface.name);

            let tx = tx.clone();
            tokio::spawn(async move { capture_packets(&interface, tx).await });
        }

        let mut counter = 0;
        let mut ud = 0;

        loop {
            let packet = match rx.recv().await {
                Some(packet) => {
                    if let Err(e) = packet {
                        match e {
                            NetMonError::ErrorCreatingDatalink => error!("Closing the thread due to an error creating datalink channel"),
                            NetMonError::ErrorReadingPacket => error!("Closing the thread due to too many errors reading packets from the datalink channel"),
                            NetMonError::UnhandledChannel => error!("Closing the thread due to unhandled channel")
                        }

                        ud += 1;
                        if ud >= datalink::interfaces().len() {
                            error!("All threads closed, shutting down...");
                            break;
                        } else {
                            continue;
                        }
                    } else {
                        packet.unwrap()
                    }
                }
                None => {
                    error!("Channel NetMon::packet_capture cannot receive.");

                    counter += 1;
                    if counter >= 5 {
                        error!("Too many errors while receiving from channel NetMon::packet_capture, shutting down...");
                        break;
                    }
                    continue;
                }
            };

            info!("Unusual activity discovered.");

            packet_serialize(packet);
            match &c2 {
                Some(stream) => {
                    info!("Sending alerts to the C2");
                    // grab existing logs + this incident, send and delete
                }
                None => {
                    warn!("Can't connect to C2");
                }
            }
        }
        Modules::NetMon
    }

    fn packet_serialize(packet: MalPacket) {
        // We can add wireshark comments in it. like save the packets in a pcap file and use ioc and
        // interface for comments
        // if works, else serialization error and shut down after trying once
    }

    async fn capture_packets(
        interface: &NetworkInterface,
        tx: Sender<Result<MalPacket, NetMonError>>,
    ) {
        match datalink::channel(&interface, Default::default()) {
            Ok(Ethernet(_, mut rx)) => {
                let mut counter = 0;
                loop {
                    match rx.next() {
                        Ok(packet) => {
                            let packet = EthernetPacket::new(packet).unwrap();

                            let ioc = false;
                            // if let Some(ioc) = NetAnalyzer::check_or_something {
                            if ioc {
                                let m_packet = MalPacket {
                                    interface: interface.name.clone(),
                                    packet: packet.packet().to_vec(),
                                    ioc: String::from("asd"),
                                };
                                let _ = tx.send(Ok(m_packet)).await;
                            }
                            counter = 0;
                        }
                        Err(e) => {
                            counter += 1;
                            error!(
                                "An error occurred while reading from {}: {}",
                                interface.name, e
                            );
                            if counter >= 5 {
                                error!(
                                    "Too many errors while reading from datalink channel for interface {}",
                                    interface.name
                                );

                                let _ = tx.send(Err(NetMonError::ErrorReadingPacket)).await;
                                break;
                            }
                        }
                    }
                }
            }
            Ok(_) => {
                error!("Unhandled channel type");
                let _ = tx.send(Err(NetMonError::UnhandledChannel)).await;
            }
            Err(e) => {
                error!(
                    "An error occurred when creating the datalink channel for {}: {}",
                    interface.name, e
                );
                let _ = tx.send(Err(NetMonError::ErrorCreatingDatalink)).await;
            }
        }
    }
}
