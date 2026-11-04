use crate::daemon_args::Args;
use crate::savings::save;
use anyhow::Result;
use dirs::download_dir;
use std::fs::{DirEntry, ReadDir};
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::{Duration, SystemTime};

const DELAY: Duration = Duration::from_millis(25);
pub fn run(arguments: Args) -> Result<()> {
    let inbox_dir: PathBuf = arguments.inbox.unwrap_or_else(|| { // No need to verify 
        download_dir().unwrap()
    });
    let seconds = Duration::from_secs(arguments.delay);

    if !arguments.daemon {
        service(&inbox_dir, &seconds)?;
        println!("All cleaned!");
        return Ok(())
    }

    loop {
        service(&inbox_dir, &seconds)?;
        sleep(DELAY);
    }
}

/// Initialize the service
pub fn service(inbox_dir: &Path, seconds: &Duration) -> Result<()> {
    let read_dir: ReadDir = inbox_dir
        .read_dir()
        .expect("[ERROR] Could not read inbox directory");

    for entry in read_dir {

        let Ok(dir_entry) = entry else { continue };
        let _ = manage_entry(dir_entry, inbox_dir, seconds);
    }
    Ok(())
}

/// Filters and manages the file-entry in the folder
fn manage_entry(dir_entry: DirEntry, inbox_dir: &Path, delay_in_seconds: &Duration) -> Result<()> {

    let metadata = dir_entry.metadata()?;
    let current_time = SystemTime::now();

    if metadata.accessed().ok().and_then(|t| current_time.duration_since(t).ok())
        .map(|d| d < *delay_in_seconds).unwrap_or(false) {
        return Ok(())
    }
    if metadata.created().ok().and_then(|t| current_time.duration_since(t).ok())
        .map(|d| d < *delay_in_seconds).unwrap_or(false) {
        return Ok(())
    }
    if metadata.modified().ok().and_then(|t| current_time.duration_since(t).ok())
        .map(|d| d < *delay_in_seconds).unwrap_or(false) {
        return Ok(())
    }

    save(dir_entry, inbox_dir);
    Ok(())
}