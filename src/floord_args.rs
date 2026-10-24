use std::path::PathBuf;

use clap::Parser;

/// Future daemon to organize files
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Directory used as inbox (Default: Downloads)
    #[arg(short, long)]
    pub inbox: Option<PathBuf>,

    /// Seconds to wait after a file apears at inbox path
    #[arg(short, long, default_value_t = 300)]
    pub delay: u64,
}

pub fn parse_args() -> Args {
    return Args::parse();
}
