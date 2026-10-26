use crate::file_groups::save;
use crate::floord_args::Args;
use dirs::download_dir;
use std::fs::ReadDir;
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;

pub fn run(arguments: Args) {
    let inbox_dir: PathBuf = arguments.inbox.unwrap_or_else(|| {
        download_dir().unwrap()
    });

    loop {
        service(&inbox_dir);
        sleep(Duration::from_millis(100)); // 10 times per second to no overload
    }
}

fn service(inbox_dir: &Path) {
    let read_dir: ReadDir = inbox_dir
        .read_dir()
        .expect("[ERROR] Could not read inbox directory");

    for entry in read_dir {
        // Read the file's metadata
        let Ok(dir_entry) = entry else { continue };

        save(dir_entry, inbox_dir)
    }
}