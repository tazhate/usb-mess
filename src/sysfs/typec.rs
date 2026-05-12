use super::{read_trim, SysfsRoot};
use crate::model::Port;

pub fn enumerate(root: &SysfsRoot) -> anyhow::Result<Vec<Port>> {
    let typec_dir = root.class("typec");
    let Ok(entries) = std::fs::read_dir(&typec_dir) else {
        return Ok(vec![]);
    };
    let mut ports: Vec<Port> = entries
        .filter_map(Result::ok)
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            if is_port_name(&name) { Some((name, e.path())) } else { None }
        })
        .map(|(name, path)| Port {
            data_role: read_trim(&path.join("data_role")),
            power_role: read_trim(&path.join("power_role")),
            power_operation_mode: read_trim(&path.join("power_operation_mode")),
            usb_typec_revision: read_trim(&path.join("usb_typec_revision")),
            usb_pd_revision: read_trim(&path.join("usb_power_delivery_revision")),
            usb_capability: read_trim(&path.join("usb_capability")),
            partner: None,
            cable: None,
            live: None,
            name,
        })
        .collect();
    ports.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(ports)
}

fn is_port_name(s: &str) -> bool {
    // Match exactly "portN" — exclude "portN-partner", "portN-cable", "portN.M".
    if let Some(rest) = s.strip_prefix("port") {
        !rest.is_empty() && rest.chars().all(|c| c.is_ascii_digit())
    } else {
        false
    }
}
