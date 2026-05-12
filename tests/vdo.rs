use usb_mess::vdo::{decode_id_header, ProductType};

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
