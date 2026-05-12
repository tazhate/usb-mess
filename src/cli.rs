use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about = "Explain your USB-C cables on Linux", long_about = None)]
pub struct Args {
    /// Output JSON instead of human text
    #[arg(long)]
    pub json: bool,

    /// Re-render on udev events (typec / power_supply)
    #[arg(long)]
    pub watch: bool,

    /// Restrict output to one port (e.g. 0 or 1)
    #[arg(long)]
    pub port: Option<u32>,

    /// Alternate sysfs root (for testing); defaults to /sys
    #[arg(long, hide = true)]
    pub sysfs_root: Option<PathBuf>,
}
