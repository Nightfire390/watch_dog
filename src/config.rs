use serde::Deserialize;
use std::fs;
use std::path::Path;
use toml;

use crate::connection::Addr;

pub const BASE_DIR: &str = env!("CARGO_MANIFEST_DIR");

#[derive(Deserialize)]
pub struct Monitor {
    pub process: bool,         // monitor processes
    pub network_traffic: bool, // analyse network traffic
    pub shell_access: bool,    // monitor any login attempts
}

#[derive(Deserialize)]
pub struct Config {
    pub c2_addr: Addr, // c2 server address
    // pub ips: bool,        // for when ips is ready
    pub monitor: Monitor, // what to monitor
}

impl Config {
    pub fn load_config() -> Option<Self> {
        // Convert it to result
        let config = match fs::read_to_string(Path::new(BASE_DIR).join("config/config.toml")) {
            Ok(file) => match toml::from_str(&file) {
                Ok(config) => config,
                Err(error) => None,
            },
            Err(error) => None,
        };
        config
    }
}

// To be updated
#[cfg(test)]
mod tests {
    use super::{fs, toml, Config, BASE_DIR};
    use std::path::Path;

    #[test]
    fn test_load_config() {
        // Create a temporary config file
        let path = Path::new(BASE_DIR).join("test/config.toml");

        // Read the configuration from the temp file
        let file = fs::read_to_string(path).expect("Failed to read config file");

        // Parse the TOML configuration
        let config: Config = toml::from_str(&file).expect("Failed to parse TOML");

        // Assertions on the parsed config
        assert_eq!(config.c2_addr.ip, "192.168.0.1");
        // assert_eq!(config.ips, false);

        assert_eq!(config.monitor.process, true);
        assert_eq!(config.monitor.network_traffic, false);
        assert_eq!(config.monitor.shell_access, true);
    }
}
