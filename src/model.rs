use serde::Serialize;
use crate::vdo::{CableVdo, IdHeader};

#[derive(Debug, Serialize)]
pub struct Snapshot {
    pub ports: Vec<Port>,
}

#[derive(Debug, Serialize)]
pub struct Port {
    pub name: String,                // "port0"
    pub data_role: Option<String>,
    pub power_role: Option<String>,
    pub power_operation_mode: Option<String>,
    pub usb_typec_revision: Option<String>,
    pub usb_pd_revision: Option<String>,
    pub usb_capability: Option<String>,
    pub partner: Option<Partner>,
    pub cable: Option<Cable>,
    pub live: Option<LiveCharge>,
}

#[derive(Debug, Serialize)]
pub struct Partner {
    pub supports_usb_pd: Option<bool>,
    pub number_of_alternate_modes: Option<u32>,
    pub identity_id_header: Option<IdHeader>,
}

#[derive(Debug, Serialize)]
pub struct Cable {
    pub kind: Option<String>,        // "active" / "passive"
    pub plug_type: Option<String>,
    pub identity_id_header: Option<IdHeader>,
    pub cable_vdo: Option<CableVdo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LiveCharge {
    pub supply: String,              // power_supply name
    pub online: bool,
    pub voltage_now_uv: Option<i64>,
    pub current_now_ua: Option<i64>,
    pub voltage_max_uv: Option<i64>,
    pub current_max_ua: Option<i64>,
}
