pub mod proc_mon {
    use tokio::sync::Mutex;
    use tokio::time::{sleep, Duration};

    use log::{error, info, warn};

    use std::collections::HashSet;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::Arc;

    use serde::Deserialize;
    use toml;

    use crate::config::BASE_DIR;
    use crate::connection::C2;
    use crate::engine::Modules;

    #[derive(Debug, Deserialize, Hash, Eq, PartialEq)]
    struct Proc {
        pid: u32,
        cmd: String,
        tty: String,
        username: String,
        start: String,
    }

    #[derive(Debug, Deserialize)]
    struct ProcList {
        proc: Vec<Proc>,
    }

    pub async fn init(c2: Option<Arc<Mutex<C2>>>) -> Modules {
        match setup() {
            Ok(procs) => {
                info!("Monitoring processes...");
                loop {
                    match load_script() {
                        Ok(_) => {
                            // get_process
                            // if snapshot != current_get_process
                            // if let Some(ioc) = ProcAnalyzer
                            let ioc = false; 
                            if ioc {
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
                        },
                        Err(e) => {
                            error!("Error running script: {}", e);
                            break;
                        }
                    }
                    sleep(Duration::from_secs(1)).await;
                }
            }
            Err(_) => {
                error!("Setup failed");
            }
        }
        Modules::ProcMon
    }

    fn get_processes() -> Result<HashSet<Proc>, ()> {
        match fs::read_to_string(Path::new("/tmp/procs.toml")) {
            Ok(output) => {
                let output = output.replace("\\", "\\\\");
                match toml::from_str::<ProcList>(&output) {
                    Ok(procs) => Ok(procs.proc.into_iter().collect::<HashSet<Proc>>()),
                    Err(e) => {
                        error!("Could not parse processes: {}", e);
                        Err(())
                    }
                }
            }
            Err(e) => {
                error!("Could not read processes: {}", e);
                Err(())
            }
        }
    }

    fn retrieve_snapshot() -> Option<HashSet<Proc>> {
        // check DB
        None
    }

    fn setup() -> Result<HashSet<Proc>, ()> {
        info!("Checking for existing snapshots...");
        match retrieve_snapshot() {
            Some(snapshot) => {
                info!("Snapshot exists");
                Ok(snapshot)
            }
            None => {
                info!("Snapshot not found. Creating one...");
                let _ = load_script();
                match get_processes() {
                    Ok(procs) => {
                        info!("Snapshot created");
                        Ok(procs)
                    }
                    Err(_) => Err(()),
                }
            }
        }
    }

    pub fn load_script() -> Result<(), std::io::Error> {
        let path: PathBuf = Path::new(BASE_DIR).join("scripts/proc_mon.sh");
        match Command::new(path)
            .env("IDS_HOME", Path::new(BASE_DIR))
            .output()
        {
            Ok(_) => {
                // TODO: better handling of the script output
                Ok(())
            }
            Err(error) => Err(error),
        }
    }
}

#[cfg(test)]
    mod tests {
        use crate::Path;
        use super::proc_mon::load_script;
        use std::fs::exists;

        #[test]
        fn test_load_script() {
            let _ = std::fs::remove_file(Path::new("/tmp/procs.toml"));

            let _ = load_script();

            assert!(exists(Path::new("/tmp/procs.toml")).unwrap());

            let _ = std::fs::remove_file(Path::new("/tmp/procs.toml"));
        }
    }
