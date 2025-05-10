use crate::config::Config;
use crate::connection::C2;
use crate::monitor::*;

use network::net_mon;
use process::proc_mon;
use shell::shell_mon;

use log::{error, info};
use std::sync::Arc;

use tokio;
use tokio::sync::Mutex;

pub struct Engine {
    config: Config,
    stream: Option<Arc<Mutex<C2>>>,
}

#[derive(Debug)]
pub enum Modules {
    NetMon,
    ProcMon,
    ShellMon,
}

impl Engine {
    pub fn configure() -> Self {
        info!("Loading configuration");
        let config = match Config::load_config() {
            Some(config) => config,
            None => {
                error!("Failed to load configuration");
                std::process::exit(-1);
            }
        };

        Self {
            config,
            stream: None,
        }
    }

    pub async fn init(&mut self) {
        info!("Initiating engine");

        info!("Initiating Connection...");
        let stream = C2::connect(&self.config.c2_addr).await;
        match stream {
            Some(c2) => {
                self.stream = Some(Arc::new(Mutex::new(c2)));
            }
            None => {
                self.stream = None;
            }
        }

        let mut handles: Vec<tokio::task::JoinHandle<Modules>> = vec![];
        info!("Initiating monitoring...");

        let engine_ref_clone = self.stream.clone();
        handles.push(tokio::spawn(async move {
            net_mon::init(engine_ref_clone).await
        }));


        let engine_ref_clone = self.stream.clone();
        handles.push(tokio::spawn(async move {
            proc_mon::init(engine_ref_clone).await
        }));


        let engine_ref_clone = self.stream.clone();
        handles.push(tokio::spawn(async move {
            shell_mon::init(engine_ref_clone).await
        }));

        for handle in handles {
            if let Ok(module) = handle.await {
                error!("Module unloaded: {:?}", module);
            }
        }
    }
}
