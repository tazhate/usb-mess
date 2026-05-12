use std::path::{Path, PathBuf};
use crate::model::Snapshot;

pub mod typec;

pub struct SysfsRoot {
    root: PathBuf,
}

impl SysfsRoot {
    pub fn new(root: impl Into<PathBuf>) -> Self { Self { root: root.into() } }
    pub fn system() -> Self { Self::new("/sys") }
    pub fn class(&self, name: &str) -> PathBuf { self.root.join("class").join(name) }

    pub fn snapshot(&self) -> anyhow::Result<Snapshot> {
        let ports = typec::enumerate(self)?;
        Ok(Snapshot { ports })
    }
}

pub(crate) fn read_trim(path: &Path) -> Option<String> {
    std::fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}
