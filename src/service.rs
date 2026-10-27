use crate::file_groups::save;
use crate::floord_args::Args;
use dirs::download_dir;
use std::fs::{DirEntry, ReadDir};
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::{Duration, SystemTime};
use anyhow::Result;

const DELAY: Duration = Duration::from_millis(100);
pub fn run(arguments: Args) -> Result<()> {
    let inbox_dir: PathBuf = arguments.inbox.unwrap_or_else(|| {
        download_dir().unwrap()
    });
    let seconds = Duration::from_secs(arguments.delay);

    loop {
        service(&inbox_dir, &seconds)?;
        sleep(DELAY);
    }
}

fn service(inbox_dir: &Path, seconds: &Duration) -> Result<()> {
    let read_dir: ReadDir = inbox_dir
        .read_dir()
        .expect("[ERROR] Could not read inbox directory");

    for entry in read_dir {
        // Read the file's metadata
        let Ok(dir_entry) = entry else { continue };
        manage_entry(dir_entry, inbox_dir, &seconds)?
    }
    Ok(())
}

fn manage_entry(dir_entry: DirEntry, inbox_dir: &Path, delay_in_seconds: &Duration) -> Result<()> {

    let metadata = dir_entry.metadata()?;
    let current_time = SystemTime::now();

    if current_time.duration_since(metadata.accessed()?)? < *delay_in_seconds ||
        current_time.duration_since(metadata.created()?)? < *delay_in_seconds ||
        current_time.duration_since(metadata.modified()?)? < *delay_in_seconds
    { return Ok(()) }

    save(dir_entry, inbox_dir);
    Ok(())
}