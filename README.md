# Floor daemon
**Floord** is a lightweight background daemon written in Rust that automatically manages and organizes incoming files.

## File Organization

**Floord** automatically organizes incoming files based on their file extensions, sorting them into the appropriate system directories according to their file type.

It uses the operating system's standard directories whenever available:

- **Documents**: .pdf, .docx, .txt, .odt, etc.
- **Music**: .mp3, .wav, .flac, .ogg, etc.
- **Videos**: .mp4, .mkv, .avi, .mov, etc.
- **Pictures**: .jpg, .jpeg, .png, .gif, .webp, etc.

**Unknown files**: If a file has an unsupported or unrecognized extension, Floord automatically creates an Unknown folder inside the designated organization directory (if it doesn't already exist) and moves the file there.

## Building and Installation
Clone the repository and run the installation script:
```bash
git clone https://github.com/Kure2323/floord.git
cd floord
sh install.sh
```
Alternatively, install both binaries using Cargo:
```bash
cargo install --git https://github.com/Kure2323/floord.git floord
cargo install --git https://github.com/Kure2323/floord.git floord_setup
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
## Architecture
- **Daemon** (floord): Handles the core service logic and file organization.
- **Setup** (floord_setup): Installs, uninstalls, and starts the daemon.
