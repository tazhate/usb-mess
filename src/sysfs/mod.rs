use crate::model::Snapshot;
use std::path::{Path, PathBuf};

pub mod power_supply;
pub mod typec;

pub struct SysfsRoot {
    root: PathBuf,
}

impl SysfsRoot {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }
    pub fn system() -> Self {
        Self::new("/sys")
    }
    pub fn class(&self, name: &str) -> PathBuf {
        self.root.join("class").join(name)
    }

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
    std::fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
}

/// Extract the bracketed (currently active) value from a sysfs string like
/// `source [sink]` -> `sink`. Strings without brackets are returned trimmed
/// (single-value fields).
pub(crate) fn parse_active(s: &str) -> String {
    if let (Some(start), Some(end)) = (s.find('['), s.find(']')) {
        if end > start + 1 {
            return s[start + 1..end].trim().to_string();
        }
    }
    s.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::parse_active;

    #[test]
    fn parse_active_picks_bracketed() {
        assert_eq!(parse_active("source [sink]"), "sink");
        assert_eq!(parse_active("[source] sink"), "source");
        assert_eq!(parse_active("host [device]"), "device");
    }

    #[test]
    fn parse_active_passes_through_plain() {
        assert_eq!(parse_active("sink"), "sink");
        assert_eq!(parse_active("  host  "), "host");
    }

    #[test]
    fn parse_active_handles_multi_with_brackets() {
        // kernel usb_type: "C [PD] PD_PPS" → active is "PD"
        assert_eq!(parse_active("C [PD] PD_PPS"), "PD");
    }
}
