use super::{read_trim, SysfsRoot};
use crate::model::LiveCharge;
use std::path::Path;

pub fn find_for_sinking_port(root: &SysfsRoot) -> Option<LiveCharge> {
    let dir = root.class("power_supply");
    let entries = std::fs::read_dir(&dir).ok()?;
    for e in entries.flatten() {
        let p = e.path();
        if matches!(
            read_trim(&p.join("type")).as_deref(),
            Some("USB_PD" | "USB_PD_DRP")
        ) && read_int(&p.join("online")) == Some(1)
        {
            return Some(LiveCharge {
                supply: e.file_name().to_string_lossy().into_owned(),
                online: true,
                voltage_now_uv: read_int(&p.join("voltage_now")),
                current_now_ua: read_int(&p.join("current_now")),
                voltage_max_uv: read_int(&p.join("voltage_max")),
                current_max_ua: read_int(&p.join("current_max")),
            });
        }
    }
    None
}

fn read_int(p: &Path) -> Option<i64> {
    read_trim(p)?.parse().ok()
}
