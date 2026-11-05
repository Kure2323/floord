use clap::Parser;
use std::path::PathBuf;

/// Floor daemon launcher
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Directory to monitor for incoming files [default: ~/Downloads]
    #[arg(short, long)]
    pub inbox: Option<PathBuf>,

    /// Minimum time in seconds a file must remain unmodified before being organized
    #[arg(short, long, default_value_t = 300)]
    pub delay: u64,

    /// Uninstall Floord from the system service manager
    #[arg(long)]
    pub uninstall: bool,
}

pub fn parse_args() -> Args {
    Args::parse()
}
