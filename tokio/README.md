# HID Gadget Emulation in Rust

[![github](https://img.shields.io/badge/github-katyo/hidg--rs-8da0cb.svg?style=for-the-badge&logo=github)](https://github.com/katyo/hidg-rs)
[![crate](https://img.shields.io/crates/v/tokio-hidg.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/tokio-hidg)
[![docs](https://img.shields.io/badge/docs.rs-tokio--hidg-66c2a5?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/tokio-hidg)
[![MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![CI](https://img.shields.io/github/workflow/status/katyo/hidg-rs/Rust?style=for-the-badge&logo=github-actions&logoColor=white)](https://github.com/katyo/hidg-rs/actions?query=workflow%3ARust)

Rust crate for interfacing with Linux HID Gadget devices (/dev/hidgX).
This crate supports [tokio](https://tokio.rs/) async runtime.

Since all functionality is dependent on Linux function calls, this crate only compiles for Linux systems.

## Crates

- [hidg-core](https://crates.io/crates/hidg-core) - core abstractions and low level interface (not for end users)
- [hidg](https://crates.io/crates/hidg) - std interface which supports synchronous operation only
- **[tokio-hidg](https://crates.io/crates/tokio-hidg)** - async interface for [tokio](https://tokio.rs/) adepts
- [async-std-hidg](https://crates.io/crates/async-std-hidg) - async interface for [async-std](https://async.rs/) adepts

## Features

- *fromstr* - implements [core::str::FromStr] implementation for some types
- *display* - implements [std::fmt::Display] implementation for some types
- *phf* - use [phf](https://crates.io/crates/phf) in [core::str::FromStr] trait implementations
- *serde* - enables [serde](https://crates.io/crates/serde) support for some types
- *keyboard* - enables keyboard class support
- *mouse* - enables mouse class support

## Usage examples

Keyboard input simulation:

```no_run
use tokio_hidg::{Device, Keyboard, StateChange, Key, Led};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut dev = Device::open(Keyboard, "hidg0").await?; // open device

    // Press left ctrl modifier
    dev.update(StateChange::press(Key::LeftCtrl)).await?;

    // Press key 'A'
    dev.update(StateChange::press(Key::A)).await?;

    // Get pressed keys
    println!("Keys: {:?}", dev.input().await.pressed().collect::<Vec<Key>>());

    // Release left ctrl modifier
    dev.update(StateChange::release(Key::LeftCtrl)).await?;

    // Release key 'A'
    dev.update(StateChange::release(Key::A)).await?;

    // Wait for LEDs updates
    dev.updated().await?;

    // Get LEDs
    println!("LEDs: {:?}", dev.output().await.lit().collect::<Vec<Led>>());

    Ok(())
}
```

Mouse input simulation:

```no_run
use tokio_hidg::{Device, Mouse, StateChange, ValueChange, Button};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let mut dev = Device::open(Mouse, "hidg0").await?; // open device

    // Press primary button
    dev.update(StateChange::press(Button::Primary)).await?;

    // Update pointer coordinates
    dev.update(ValueChange::absolute((150, 50))).await?;

    // Move pointer relatively
    dev.update(ValueChange::relative((70, -30))).await?;

    // Get pressed buttons
    println!("Buttons: {:?}", dev.input().await.pressed().collect::<Vec<Button>>());

    // Release primary button
    dev.update(StateChange::release(Button::Primary)).await?;

    Ok(())
}
```