use crate::file_groups::save;
use dirs::download_dir;
use std::thread::sleep;
use std::{
    fs::ReadDir,
    path::PathBuf,
    time::Duration,
};

mod floord_args;
pub mod database;
pub mod file_groups;

fn main() {
    let arguments: floord_args::Args = floord_args::parse_args();
    let inbox_dir: PathBuf = arguments.inbox.unwrap_or_else(|| {
        download_dir().expect("[ERROR] Could not find default inbox path (Downloads)")
    });

    let seconds = Duration::new(arguments.delay, 0);

    loop {
        service(&inbox_dir);
        sleep(seconds);
    }
}


fn service(inbox_dir: &PathBuf) {
    let read_dir: ReadDir = inbox_dir
        .read_dir()
        .expect("[ERROR] Could not read inbox directory");

    for entry in read_dir {
        // Read the file's metadata
        let Ok(dir_entry) = entry else { continue };

        save(dir_entry, inbox_dir)
    }
}