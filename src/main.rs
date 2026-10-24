use dirs::download_dir;
use std::{
    fs::ReadDir,
    path::PathBuf,
    time::Duration,
};
use std::thread::sleep;

mod floord_args;
pub mod database;

fn main() {
    let arguments: floord_args::Args = floord_args::parse_args();
    let inbox_dir: PathBuf = arguments.inbox.unwrap_or_else(|| {
        download_dir().expect("[ERROR] Could not find default inbox path (Downloads)")
    });

    let seconds = Duration::new(arguments.delay, 0);

    loop {
        let read_dir: ReadDir = inbox_dir
            .read_dir()
            .expect("[ERROR] Could not read inbox directory");

        for entry in read_dir {
            // Read the file's metadata
            let metadata = match entry {
                Ok(i) => {
                    match i.metadata() {
                        Ok(_metadata_) => {
                            _metadata_
                        }
                        Err(_) => {continue}
                    }
                }
                Err(_) => {continue}
            };

            // Todo: Database and file filter by filetype
            println!("Todo bien: {:?}", metadata);

        }

        sleep(seconds);
    }
}
