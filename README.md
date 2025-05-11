# Watch_Dog

Watch_Dog is a simple HIDS to keep your network safe by monitoring connections, processes, and network traffic. When suspicious activity is detected, Watch_Dog alerts you through a centralized control panel. Currently its in the development phase. It will integrate the use ML algorithms with signature detection to identify both known threats and unusual behavior that might indicate compromise. Once the daemon is done, a centralized control panel will be provided.

## How it works
### Monitor Modules

Watch_Dog's monitor modules collect data from varous sources:

- **Connection Monitor**: Tracks SSH logins and unusual connections (like reverse shells)
- **Process Monitor**: Observes process currently running processes
- **Network Traffic Monitor**: Captures network traffic on available interfaces

### Analyzer Modules (In Development)

Each monitor will feed data to their variant of ML powered analyzer that:

- Analyze each of the data sources
- Identify indicators of compromise
- Rate severity of the attack
- Report findings to the control panel

## Quick Start
To compile the project, you will need to install rust. Follow the installation steps from rust's documentation page. Once done, you can proceed with the following:

```
# Clone the repository
git clone https://github.com/Nightfire390/watch_dog.git
cd watch_dog

# Build the program
cargo build --release

# Run the program
sudo ./target/release/watch_dog
```

## Current Status

Watch_Dog is under active development:

- Core monitoring framework: ✅
- Connection, process, and network monitoring: ✅
- Analyzer modules: 🚧 (In development)
- AI/ML pipeline: 🚧 (In development)
- Control panel integration: 🚧 (In development)

This project is not production-ready. Use in production environments at your own risk.
