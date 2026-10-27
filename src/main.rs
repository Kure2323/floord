use anyhow::Result;
use crate::service_handler::service_handler;

pub mod service;
pub mod floord_args;
pub mod file_groups;
pub mod service_handler;

fn main() -> Result<()> {
    let arguments = floord_args::parse_args();

    if arguments.run {
        service::run(arguments)?;
    } else {
        // TODO: Service install process
        service_handler(arguments)?;
    }
    Ok(())
}