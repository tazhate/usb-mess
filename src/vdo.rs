use serde::Serialize;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum ProductType {
    PassiveCable,
    ActiveCable,
    AlternateModeAdapter,
    Hub,
    Peripheral,
    Other(u8),
}

#[derive(Debug, Clone, Serialize)]
pub struct IdHeader {
    pub raw: u32,
    pub usb_vid: u16,
    pub product_type_cable: Option<ProductType>,
    pub modal_operation: bool,
    pub usb_host: bool,
    pub usb_device: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum CableCurrent {
    Reserved,
    A3,
    A5,
    Unknown(u8),
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub enum CableSpeed {
    Usb20,
    Usb32Gen1,
    Usb32Gen2,
    Usb4,
    Unknown(u8),
}

#[derive(Debug, Clone, Serialize)]
pub struct CableVdo {
    pub raw: u32,
    pub speed: CableSpeed,
    pub max_current: CableCurrent,
}

pub fn decode_cable_vdo(raw: u32) -> anyhow::Result<CableVdo> {
    let speed = match raw & 0b111 {
        0 => CableSpeed::Usb20,
        1 => CableSpeed::Usb32Gen1,
        3 => CableSpeed::Usb32Gen2,
        4 => CableSpeed::Usb4,
        other => CableSpeed::Unknown(other as u8),
    };
    let max_current = match (raw >> 5) & 0b11 {
        0 => CableCurrent::Reserved,
        1 => CableCurrent::A3,
        2 => CableCurrent::A5,
        other => CableCurrent::Unknown(other as u8),
    };
    Ok(CableVdo { raw, speed, max_current })
}

pub fn parse_vdo_hex(s: &str) -> anyhow::Result<u32> {
    let s = s.trim();
    let stripped = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")).unwrap_or(s);
    u32::from_str_radix(stripped, 16)
        .map_err(|e| anyhow::anyhow!("invalid VDO hex {s:?}: {e}"))
}

pub fn decode_id_header(raw: u32) -> anyhow::Result<IdHeader> {
    let usb_vid = (raw & 0xFFFF) as u16;
    let product_type_cable_bits = ((raw >> 27) & 0b111) as u8;
    let product_type_cable = match product_type_cable_bits {
        0 => None,
        3 => Some(ProductType::PassiveCable),
        4 => Some(ProductType::ActiveCable),
        other => Some(ProductType::Other(other)),
    };
    Ok(IdHeader {
        raw,
        usb_vid,
        product_type_cable,
        modal_operation: (raw >> 26) & 1 == 1,
        usb_host: (raw >> 31) & 1 == 1,
        usb_device: (raw >> 30) & 1 == 1,
    })
}
