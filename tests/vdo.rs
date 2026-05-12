use usb_mess::vdo::{decode_cable_vdo, decode_id_header, CableCurrent, CableSpeed, ProductType};

#[test]
fn id_header_apple_passive_cable() {
    // Real Apple 240W USB4 cable ID header VDO (vendor 0x05ac, passive cable plug)
    let v = decode_id_header(0x1c0005ac).unwrap();
    assert_eq!(v.usb_vid, 0x05ac);
    assert!(matches!(v.product_type_cable, Some(ProductType::PassiveCable)));
}

#[test]
fn id_header_unknown_bits_preserved() {
    let v = decode_id_header(0xdeadbeef).unwrap();
    assert_eq!(v.raw, 0xdeadbeef);
}

#[test]
fn cable_vdo_passive_usb32_gen2_3a() {
    // Synthetic: USB 3.2 Gen 2 (speed=3), 3A (current=1), passive (vdo_version=0)
    // Layout per USB PD 3.x Passive Cable VDO
    let raw = (0b011u32 << 0)        // USB Signaling = USB 3.2 Gen2
           | (0b01u32 << 5);         // VBUS Current = 3A
    let v = decode_cable_vdo(raw).unwrap();
    assert_eq!(v.speed, CableSpeed::Usb32Gen2);
    assert_eq!(v.max_current, CableCurrent::A3);
}

#[test]
fn cable_vdo_passive_usb4_5a() {
    let raw = (0b100u32 << 0)        // USB4
           | (0b10u32 << 5);         // 5A
    let v = decode_cable_vdo(raw).unwrap();
    assert_eq!(v.speed, CableSpeed::Usb4);
    assert_eq!(v.max_current, CableCurrent::A5);
}
