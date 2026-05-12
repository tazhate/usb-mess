use clap::Parser;
use usb_mess::cli::Args;
use usb_mess::render::{json::to_json, text::to_text};
use usb_mess::sysfs::SysfsRoot;

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let root = match &args.sysfs_root {
        Some(p) => SysfsRoot::new(p),
        None => SysfsRoot::system(),
    };

    if !root_typec_exists(&root) {
        eprintln!(
            "kernel typec class not available; ensure CONFIG_TYPEC + UCSI/tcpm driver is loaded"
        );
        std::process::exit(2);
    }

    if args.watch {
        usb_mess::watch::run(&root, &args)?;
        return Ok(());
    }

    let mut snap = root.snapshot()?;
    if let Some(p) = args.port {
        let target = format!("port{p}");
        snap.ports.retain(|x| x.name == target);
    }

    if args.json {
        println!("{}", to_json(&snap)?);
    } else {
        print!("{}", to_text(&snap, atty::is(atty::Stream::Stdout)));
    }
    Ok(())
}

fn root_typec_exists(r: &SysfsRoot) -> bool {
    r.class("typec").exists()
}
