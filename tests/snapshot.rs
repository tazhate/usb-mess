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
