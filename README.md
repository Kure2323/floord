# Floor daemon
**Floord** is a lightweight background daemon written in Rust that automatically manages and organizes incoming files.
## Architecture
- **Daemon** (floord): Handles the core service logic and file organization.
- **Set-up** (floord_setup): Installs, uninstalls, and starts the daemon.

## Building and Installation
Clone the repository and run the installation script:
```bash
git clone https://github.com/Kure2323/floord.git
cd floord
sh install.sh
```

## Starting the Service
Once installed, run:
```bash
floord_setup
```
## Service Guide
To see the available commands and options, run:
```bash
floord_setup --help
```
