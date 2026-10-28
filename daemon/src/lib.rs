pub mod service;
pub mod daemon_args;
pub mod savings;

pub fn daemon_run() -> anyhow::Result<()> {
    service::run(daemon_args::parse_args())?;
    Ok(())
}