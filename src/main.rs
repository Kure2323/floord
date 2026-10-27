use anyhow::Result;
pub mod service;
pub mod floord_args;
pub mod file_groups;

fn main() -> Result<()> {
    let arguments = floord_args::parse_args();
    service::run(arguments)?;
    Ok(())
}