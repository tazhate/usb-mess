use super::{read_trim, SysfsRoot};
use crate::model::LiveCharge;
use std::path::Path;

pub fn find_for_sinking_port(root: &SysfsRoot) -> Option<LiveCharge> {
    let dir = root.class("power_supply");
    let entries = std::fs::read_dir(&dir).ok()?;
    for e in entries.flatten() {
        let p = e.path();
        if !is_usb_pd_supply(&p) {
            continue;
        }
        if read_int(&p.join("online")) != Some(1) {
            continue;
        }
        return Some(LiveCharge {
            supply: e.file_name().to_string_lossy().into_owned(),
            online: true,
            voltage_now_uv: read_int(&p.join("voltage_now")),
            current_now_ua: read_int(&p.join("current_now")),
            voltage_max_uv: read_int(&p.join("voltage_max")),
            current_max_ua: read_int(&p.join("current_max")),
        });
    }
    None
}

/// True if this power_supply entry represents a USB-C / USB-PD port.
/// The kernel reports `type` as `USB`, `USB_PD`, or `USB_PD_DRP` for type-C
/// power inputs. UCSI drivers use `USB` and put the negotiated contract
/// (PD / PD_PPS) into `usb_type` as a bracketed-active list.
fn is_usb_pd_supply(p: &Path) -> bool {
    matches!(
        read_trim(&p.join("type")).as_deref(),
        Some("USB" | "USB_PD" | "USB_PD_DRP" | "USB_C")
    )
}

fn read_int(p: &Path) -> Option<i64> {
    read_trim(p)?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::super::parse_active;

    #[test]
    fn usb_type_active_value() {
        assert_eq!(parse_active("C [PD] PD_PPS"), "PD");
    }
}
