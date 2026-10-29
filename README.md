# Floor daemon
Ritten in Rust, Floord is a lightweight tool that works in the background that manages the incoming files and organize them.

## Organization
- **Daemon** (floord): The service logic and execution within files
- **Set-up** (floord_setup): Service installer/uninstaller and starter

## Compiling and Installation process
```bash
git clone https://github.com/Kure2323/floord.git
cd floord
sh install.sh
```

## Start service
```bash
floord_setup
```

