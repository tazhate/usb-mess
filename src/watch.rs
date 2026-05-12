use crate::cli::Args;
use crate::sysfs::SysfsRoot;

pub fn run(_root: &SysfsRoot, _args: &Args) -> anyhow::Result<()> {
    anyhow::bail!("--watch not yet implemented")
}
