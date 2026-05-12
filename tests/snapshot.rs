use std::path::PathBuf;
use usb_mess::render::json::to_json;
use usb_mess::render::text::to_text;
use usb_mess::sysfs::SysfsRoot;

fn fixture(name: &str) -> SysfsRoot {
    let p: PathBuf = [
        env!("CARGO_MANIFEST_DIR"),
        "tests",
        "fixtures",
        "sysfs",
        name,
    ]
    .iter()
    .collect();
    SysfsRoot::new(p)
}

#[test]
fn empty_returns_no_ports() {
    let snap = fixture("empty").snapshot().unwrap();
    assert!(snap.ports.is_empty());
}

#[test]
fn one_port_no_partner_reads_roles() {
    let snap = fixture("one_port_no_partner").snapshot().unwrap();
    assert_eq!(snap.ports.len(), 1);
    let p = &snap.ports[0];
    assert_eq!(p.name, "port0");
    assert_eq!(p.data_role.as_deref(), Some("host"));
    assert_eq!(p.power_role.as_deref(), Some("source"));
    assert!(p.partner.is_none());
}

#[test]
fn port_with_partner_and_cable() {
    let snap = fixture("port_partner_cable").snapshot().unwrap();
    let p = &snap.ports[0];
    let partner = p.partner.as_ref().expect("partner present");
    assert_eq!(partner.supports_usb_pd, Some(true));
    assert_eq!(partner.number_of_alternate_modes, Some(1));
    assert_eq!(partner.identity_id_header.as_ref().unwrap().usb_vid, 0x05ac);

    let cable = p.cable.as_ref().expect("cable present");
    assert_eq!(cable.kind.as_deref(), Some("passive"));
    let cv = cable.cable_vdo.as_ref().unwrap();
    assert!(matches!(cv.speed, usb_mess::vdo::CableSpeed::Usb32Gen2));
    assert!(matches!(cv.max_current, usb_mess::vdo::CableCurrent::A3));
}

#[test]
fn json_output_for_one_port() {
    let snap = fixture("one_port_no_partner").snapshot().unwrap();
    let s = to_json(&snap).unwrap();
    insta::assert_snapshot!("one_port_no_partner_json", s);
}

#[test]
fn port_sinking_reports_live_charge() {
    let snap = fixture("port_charging").snapshot().unwrap();
    let live = snap.ports[0]
        .live
        .as_ref()
        .expect("live present when sinking");
    assert!(live.online);
    assert_eq!(live.current_now_ua, Some(2_890_000));
    assert_eq!(live.voltage_now_uv, Some(19_500_000));
}

#[test]
fn text_output_port_partner_cable() {
    let snap = fixture("port_partner_cable").snapshot().unwrap();
    let s = to_text(&snap, false); // no color
    insta::assert_snapshot!("port_partner_cable_text", s);
}

#[test]
fn text_output_empty_says_no_ports() {
    let snap = fixture("empty").snapshot().unwrap();
    let s = to_text(&snap, false);
    assert!(s.contains("No USB-C ports"));
}
