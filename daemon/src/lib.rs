use std::path::Path;
use std::time::Duration;

pub mod service;
pub mod daemon_args;
pub mod savings;

pub fn daemon_run() -> anyhow::Result<()> {
    service::run(daemon_args::parse_args())?;
    Ok(())
}

pub fn service(path: &Path, duration: &Duration) -> anyhow::Result<()> {
    service::service(path, duration)?;
    Ok(())
}