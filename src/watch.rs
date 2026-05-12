use crate::cli::Args;
use crate::render::{json::to_json, text::to_text};
use crate::sysfs::SysfsRoot;
use std::io::Write;
use std::time::{Duration, Instant};

pub fn run(root: &SysfsRoot, args: &Args) -> anyhow::Result<()> {
    let socket = udev::MonitorBuilder::new()?
        .match_subsystem("typec")?
        .match_subsystem("power_supply")?
        .listen()?;

    render_once(root, args)?;
    let debounce = Duration::from_millis(200);
    let mut pending: Option<Instant> = None;

    loop {
        if let Some(_event) = socket.iter().next() {
            pending = Some(Instant::now());
        }
        if let Some(t) = pending {
            if t.elapsed() >= debounce {
                pending = None;
                render_once(root, args)?;
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn render_once(root: &SysfsRoot, args: &Args) -> anyhow::Result<()> {
    // Clear screen, home cursor.
    print!("\x1b[2J\x1b[H");
    let mut snap = root.snapshot()?;
    if let Some(p) = args.port {
        let target = format!("port{p}");
        snap.ports.retain(|x| x.name == target);
    }
    if args.json {
        println!("{}", to_json(&snap)?);
    } else {
        print!("{}", to_text(&snap, true));
    }
    std::io::stdout().flush()?;
    Ok(())
}
