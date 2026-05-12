# usb-mess

> Your USB-C cable is lying to you. Find out how.

`usb-mess` is a Linux CLI that reads `/sys/class/typec/` and
`/sys/class/power_supply/`, decodes the e-marker chip's VDOs, and tells
you in plain English what your cable can actually do.

Inspired by [WhatCable](https://github.com/darrylmorley/whatcable) (macOS).

## Quick start

```sh
$ usb-mess
[port0]
  role:       sink / device
  pd rev:     3.0
  partner:    PD=yes, altmodes=1
              VID=0x05ac
  cable:      passive, USB 3.2 Gen 2 (10 Gbps), 3 A (≤60 W @ 20 V)
              cable VID=0x05ac
  charging:   56.4 W  (19.50 V, 2.890 A)
```

## Install

### From release

```sh
curl -L https://github.com/tazhate/usb-mess/releases/latest/download/usb-mess-x86_64-unknown-linux-musl.tar.gz | tar xz
sudo install -m 755 usb-mess /usr/local/bin/
```

### From source

```sh
cargo install --git https://github.com/tazhate/usb-mess
```

## Usage

```
usb-mess              one-shot snapshot
usb-mess --json       machine-readable
usb-mess --watch      re-render on udev events
usb-mess --port 1     restrict to one port
```

## Requirements

- Linux kernel with `CONFIG_TYPEC` and a backend driver (UCSI on most
  modern laptops, tcpm on embedded boards).
- No root needed — `/sys/class/typec/` is world-readable.

## How it works

USB-C cables with an e-marker chip carry VDOs (Vendor Defined Objects)
that describe their capabilities. The kernel exposes them under
`/sys/class/typec/<port>-cable/identity/`. `usb-mess` decodes those
bitfields per the USB Power Delivery spec and prints them in human form.

## License

MIT
