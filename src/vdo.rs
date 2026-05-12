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
