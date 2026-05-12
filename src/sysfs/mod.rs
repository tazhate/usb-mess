use std::path::{Path, PathBuf};
use crate::model::Snapshot;

pub mod power_supply;
pub mod typec;

pub struct SysfsRoot {
    root: PathBuf,
}

impl SysfsRoot {
    pub fn new(root: impl Into<PathBuf>) -> Self { Self { root: root.into() } }
    pub fn system() -> Self { Self::new("/sys") }
    pub fn class(&self, name: &str) -> PathBuf { self.root.join("class").join(name) }

    pub fn snapshot(&self) -> anyhow::Result<Snapshot> {
        let mut ports = typec::enumerate(self)?;
        let live = power_supply::find_for_sinking_port(self);
        for p in &mut ports {
            if p.power_role.as_deref() == Some("sink") {
                p.live = live.clone();
            }
        }
        Ok(ports_into_snapshot(ports))
    }
}

fn ports_into_snapshot(ports: Vec<crate::model::Port>) -> Snapshot {
    Snapshot { ports }
}

pub(crate) fn read_trim(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}
