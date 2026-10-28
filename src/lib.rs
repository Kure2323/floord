use crate::service_handler::service_handler;
use anyhow::Result;
pub mod service;
pub mod floord_args;
pub mod file_groups;
pub mod service_handler;
pub fn floord_run() -> Result<()> {
    let arguments = floord_args::parse_args();

    if arguments.run {
        service::run(arguments)?;
    } else {
        // TODO: Service start on Windows devices (currently broken)
        service_handler(arguments)?;
    }
    Ok(())
}