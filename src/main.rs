mod config;
mod connection;
mod engine;
mod monitor;
mod analyzer;

use config::BASE_DIR;
use log4rs;

use engine::Engine;
use std::path::Path;

#[tokio::main]
async fn main() {
    log4rs::init_file(
        Path::new(BASE_DIR).join("config/log4rs.yaml"),
        Default::default(),
    )
    .unwrap();

    Engine::configure().init().await;
}
