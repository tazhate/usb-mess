use super::{parse_active, read_trim, SysfsRoot};
use crate::model::{Cable, Partner, Port};
use crate::vdo::{decode_cable_vdo, decode_id_header, parse_vdo_hex};
use std::path::Path;

pub fn enumerate(root: &SysfsRoot) -> anyhow::Result<Vec<Port>> {
    let typec_dir = root.class("typec");
    let Ok(entries) = std::fs::read_dir(&typec_dir) else {
        return Ok(vec![]);
    };
    let mut ports: Vec<Port> = entries
        .filter_map(Result::ok)
        .filter_map(|e| {
            let name = e.file_name().to_string_lossy().into_owned();
            if is_port_name(&name) {
                Some((name, e.path()))
            } else {
                None
            }
        })
        .map(|(name, path)| {
            let typec_dir = path.parent().unwrap().to_path_buf();
            let partner_path = typec_dir.join(format!("{name}-partner"));
            let cable_path = typec_dir.join(format!("{name}-cable"));
            Port {
                data_role: read_trim(&path.join("data_role"))
                    .as_deref()
                    .map(parse_active),
                power_role: read_trim(&path.join("power_role"))
                    .as_deref()
                    .map(parse_active),
                power_operation_mode: read_trim(&path.join("power_operation_mode"))
                    .as_deref()
                    .map(parse_active),
                usb_typec_revision: read_trim(&path.join("usb_typec_revision")),
                usb_pd_revision: read_trim(&path.join("usb_power_delivery_revision")),
                usb_capability: read_trim(&path.join("usb_capability")),
                partner: read_partner(&partner_path),
                cable: read_cable(&cable_path),
                live: None,
                name,
            }
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

fn read_yes_no(p: &Path) -> Option<bool> {
    read_trim(p).map(|s| s == "yes")
}

fn read_u32_dec(p: &Path) -> Option<u32> {
    read_trim(p).and_then(|s| s.parse().ok())
}

fn read_id_header(identity_dir: &Path) -> Option<crate::vdo::IdHeader> {
    let raw = parse_vdo_hex(&read_trim(&identity_dir.join("id_header"))?).ok()?;
    decode_id_header(raw).ok()
}

fn read_partner(path: &Path) -> Option<Partner> {
    if !path.exists() {
        return None;
    }
    Some(Partner {
        supports_usb_pd: read_yes_no(&path.join("supports_usb_power_delivery")),
        number_of_alternate_modes: read_u32_dec(&path.join("number_of_alternate_modes")),
        identity_id_header: read_id_header(&path.join("identity")),
    })
}

fn read_cable(path: &Path) -> Option<Cable> {
    if !path.exists() {
        return None;
    }
    let identity = path.join("identity");
    let cable_vdo = read_trim(&identity.join("product_type_vdo1"))
        .and_then(|s| parse_vdo_hex(&s).ok())
        .and_then(|raw| decode_cable_vdo(raw).ok());
    Some(Cable {
        kind: read_trim(&path.join("type")),
        plug_type: read_trim(&path.join("plug_type")),
        identity_id_header: read_id_header(&identity),
        cable_vdo,
    })
}
