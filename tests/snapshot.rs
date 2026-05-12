use std::path::PathBuf;
use usb_mess::sysfs::SysfsRoot;

fn fixture(name: &str) -> SysfsRoot {
    let p: PathBuf = [env!("CARGO_MANIFEST_DIR"), "tests", "fixtures", "sysfs", name].iter().collect();
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
