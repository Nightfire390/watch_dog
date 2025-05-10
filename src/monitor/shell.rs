pub mod shell_mon {
    use serde::Deserialize;
    use tokio::sync::Mutex;
    use tokio::time::{sleep, Duration};

    use log::{error, info, warn};
    use std::collections::HashSet;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::Arc;

    use crate::config::BASE_DIR;
    use crate::connection::C2;
    use crate::engine::Modules;

    
    #[derive(Debug, Deserialize, Hash, Eq, PartialEq)]
    struct ShellAccess {
        username: String,
        tty: String,
        from: String,
        process: String
    }

    #[derive(Debug, Deserialize)]
    struct ShellAccessList {
        shell: Vec<ShellAccess>
    }

    pub async fn init(c2: Option<Arc<Mutex<C2>>>) -> Modules {
        info!("Monitoring shells...");
        loop {
            match load_script() {
                Ok(_) => {
                    let _ = get_processes();
                    // if let Some(ioc) = ShellAnalyzer
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
            sleep(Duration::from_secs(3)).await;
        }
        Modules::ShellMon
    }

    fn get_processes() -> Result<HashSet<ShellAccess>, ()> {
        match fs::read_to_string(Path::new("/tmp/shells.toml")) {
            Ok(output) => {
                let output = output.replace("\\", "\\\\");
                match toml::from_str::<ShellAccessList>(&output) {
                    Ok(shells) => Ok(shells.shell.into_iter().collect::<HashSet<ShellAccess>>()),
                    Err(e) => {
                        error!("Could not parse shells: {}", e);
                        Err(())
                    }
                }
            }
            Err(e) => {
                error!("Could not read shells: {}", e);
                Err(())
            }
        }
    }

    pub fn load_script() -> Result<(), std::io::Error> {
        let path: PathBuf = Path::new(BASE_DIR).join("scripts/shell_mon.sh");
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
        use super::shell_mon::load_script;
        use std::fs::exists;

        #[test]
        fn test_load_script() {
            let _ = std::fs::remove_file(Path::new("/tmp/shells.toml"));

            let _ = load_script();

            assert!(exists(Path::new("/tmp/shells.toml")).unwrap());

            let _ = std::fs::remove_file(Path::new("/tmp/shells.toml"));
        }
    }
