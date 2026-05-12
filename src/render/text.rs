use crate::model::{Cable, LiveCharge, Partner, Port, Snapshot};
use crate::vdo::{CableCurrent, CableSpeed};
use std::fmt::Write;

pub fn to_text(snap: &Snapshot, _color: bool) -> String {
    let mut out = String::new();
    if snap.ports.is_empty() {
        out.push_str("No USB-C ports found.\n");
        return out;
    }
    for p in &snap.ports {
        render_port(&mut out, p);
        out.push('\n');
    }
    out
}

fn render_port(out: &mut String, p: &Port) {
    let _ = writeln!(out, "[{}]", p.name);
    if p.partner.is_none() {
        let _ = writeln!(out, "  (nothing connected)");
        return;
    }
    let _ = writeln!(
        out,
        "  role:       {} / {}",
        p.power_role.as_deref().unwrap_or("—"),
        p.data_role.as_deref().unwrap_or("—")
    );
    if let Some(rev) = &p.usb_pd_revision {
        let _ = writeln!(out, "  pd rev:     {rev}");
    }
    if let Some(partner) = &p.partner {
        render_partner(out, partner);
    }
    match &p.cable {
        None => {
            let _ = writeln!(out, "  cable:      (no e-marker / not detected)");
        }
        Some(c) => render_cable(out, c),
    }
    if let Some(live) = &p.live {
        render_live(out, live);
    }
}

fn render_partner(out: &mut String, p: &Partner) {
    let pd = match p.supports_usb_pd {
        Some(true) => "yes",
        Some(false) => "no",
        None => "—",
    };
    let _ = writeln!(
        out,
        "  partner:    PD={pd}, altmodes={}",
        p.number_of_alternate_modes
            .map(|n| n.to_string())
            .unwrap_or_else(|| "—".into())
    );
    if let Some(idh) = &p.identity_id_header {
        let _ = writeln!(out, "              VID=0x{:04x}", idh.usb_vid);
    }
}

fn render_cable(out: &mut String, c: &Cable) {
    let kind = c.kind.as_deref().unwrap_or("—");
    let speed = match c.cable_vdo.as_ref().map(|v| &v.speed) {
        Some(CableSpeed::Usb20) => "USB 2.0",
        Some(CableSpeed::Usb32Gen1) => "USB 3.2 Gen 1 (5 Gbps)",
        Some(CableSpeed::Usb32Gen2) => "USB 3.2 Gen 2 (10 Gbps)",
        Some(CableSpeed::Usb4) => "USB4 (≥20 Gbps)",
        Some(CableSpeed::Unknown(b)) => {
            let _ = writeln!(out, "  cable:      {kind}, speed=unknown({b})");
            return;
        }
        None => "—",
    };
    let current = match c.cable_vdo.as_ref().map(|v| &v.max_current) {
        Some(CableCurrent::A3) => "3 A (≤60 W @ 20 V)",
        Some(CableCurrent::A5) => "5 A (≤100 W @ 20 V / ≤240 W @ 48 V EPR)",
        Some(CableCurrent::Reserved) => "reserved",
        Some(CableCurrent::Unknown(b)) => {
            let _ = writeln!(out, "  cable:      {kind}, current=unknown({b})");
            return;
        }
        None => "—",
    };
    let _ = writeln!(out, "  cable:      {kind}, {speed}, {current}");
    if let Some(idh) = &c.identity_id_header {
        let _ = writeln!(out, "              cable VID=0x{:04x}", idh.usb_vid);
    }
}

fn render_live(out: &mut String, l: &LiveCharge) {
    let v = l.voltage_now_uv.map(|x| x as f64 / 1_000_000.0);
    let i = l.current_now_ua.map(|x| x as f64 / 1_000_000.0);
    let w = match (v, i) {
        (Some(v), Some(i)) => format!("{:.1} W", v * i),
        _ => "—".into(),
    };
    let _ = writeln!(
        out,
        "  charging:   {w}  ({}, {})",
        v.map(|x| format!("{x:.2} V"))
            .unwrap_or_else(|| "— V".into()),
        i.map(|x| format!("{x:.3} A"))
            .unwrap_or_else(|| "— A".into())
    );
}
