use serde::Deserialize;
use std::fmt;

use log::{info, warn};
use tokio::io::{BufReader, BufWriter};
use tokio::net::TcpStream;

#[derive(Deserialize)]
pub struct Addr {
    pub ip: String,
    pub port: u16,
}

pub struct C2 {
    stream: TcpStream,
}

struct Request {
    version: [u8; 6],     // WDv1.0
    request_type: String, // Type Alert, etc.
    length: u32,          // number of bytes to read from request
    content: [u8],        // Alert details
}

enum Alert<'a> {
    Warning(&'a str),
    Error(&'a str),
}

pub enum Caller {
    Process,
    Network,
    SelfDiagnosis,
    Shell,
}

impl fmt::Display for Addr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

impl C2 {
    pub async fn connect(c2_addr: &Addr) -> Option<C2> {
        let c2_addr = format!("{}", c2_addr);

        match TcpStream::connect(&c2_addr).await {
            Ok(stream) => {
                info!("Connection successful");
                Some(C2 { stream })
            }
            Err(error) => {
                warn!("Connection refused. Offline mode active. Error: {}", error);
                None
            }
        }
    }
    async fn send_active(&mut self) { // Send message that indicates everything is active.
                                      //self.stream.write
    }

    fn send_alert(&mut self, caller: Caller, alert: Alert) {}
    fn send_log(&mut self, caller: Caller) {}
}
