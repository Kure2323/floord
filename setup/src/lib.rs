mod service_handler;
mod setup_args;

pub fn setup_run() -> anyhow::Result<()> {
    service_handler::run(setup_args::parse_args())?;
    Ok(())
}

