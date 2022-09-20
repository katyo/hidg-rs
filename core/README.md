# HID Gadget Emulation in Rust

[![github](https://img.shields.io/badge/github-katyo/hidg--rs-8da0cb.svg?style=for-the-badge&logo=github)](https://github.com/katyo/hidg-rs)
[![crate](https://img.shields.io/crates/v/hidg-core.svg?style=for-the-badge&color=fc8d62&logo=rust)](https://crates.io/crates/hidg-core)
[![docs](https://img.shields.io/badge/docs.rs-hidg--core-66c2a5?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyByb2xlPSJpbWciIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgdmlld0JveD0iMCAwIDUxMiA1MTIiPjxwYXRoIGZpbGw9IiNmNWY1ZjUiIGQ9Ik00ODguNiAyNTAuMkwzOTIgMjE0VjEwNS41YzAtMTUtOS4zLTI4LjQtMjMuNC0zMy43bC0xMDAtMzcuNWMtOC4xLTMuMS0xNy4xLTMuMS0yNS4zIDBsLTEwMCAzNy41Yy0xNC4xIDUuMy0yMy40IDE4LjctMjMuNCAzMy43VjIxNGwtOTYuNiAzNi4yQzkuMyAyNTUuNSAwIDI2OC45IDAgMjgzLjlWMzk0YzAgMTMuNiA3LjcgMjYuMSAxOS45IDMyLjJsMTAwIDUwYzEwLjEgNS4xIDIyLjEgNS4xIDMyLjIgMGwxMDMuOS01MiAxMDMuOSA1MmMxMC4xIDUuMSAyMi4xIDUuMSAzMi4yIDBsMTAwLTUwYzEyLjItNi4xIDE5LjktMTguNiAxOS45LTMyLjJWMjgzLjljMC0xNS05LjMtMjguNC0yMy40LTMzLjd6TTM1OCAyMTQuOGwtODUgMzEuOXYtNjguMmw4NS0zN3Y3My4zek0xNTQgMTA0LjFsMTAyLTM4LjIgMTAyIDM4LjJ2LjZsLTEwMiA0MS40LTEwMi00MS40di0uNnptODQgMjkxLjFsLTg1IDQyLjV2LTc5LjFsODUtMzguOHY3NS40em0wLTExMmwtMTAyIDQxLjQtMTAyLTQxLjR2LS42bDEwMi0zOC4yIDEwMiAzOC4ydi42em0yNDAgMTEybC04NSA0Mi41di03OS4xbDg1LTM4Ljh2NzUuNHptMC0xMTJsLTEwMiA0MS40LTEwMi00MS40di0uNmwxMDItMzguMiAxMDIgMzguMnYuNnoiPjwvcGF0aD48L3N2Zz4K)](https://docs.rs/hidg-core)
[![MIT](https://img.shields.io/badge/License-MIT-brightgreen.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![CI](https://img.shields.io/github/workflow/status/katyo/hidg-rs/Rust?style=for-the-badge&logo=github-actions&logoColor=white)](https://github.com/katyo/hidg-rs/actions?query=workflow%3ARust)

Rust crate for interfacing with Linux HID Gadget devices (/dev/hidgX).

Since all functionality is dependent on Linux function calls, this crate only compiles for Linux systems.

## Crates

- **[hidg-core](https://crates.io/crates/hidg-core)** - core abstractions and low level interface (not for end users)
- [hidg](https://crates.io/crates/hidg) - std interface which supports synchronous operation only
- [tokio-hidg](https://crates.io/crates/tokio-hidg) - async interface for [tokio](https://tokio.rs/) adepts
- [async-std-hidg](https://crates.io/crates/async-std-hidg) - async interface for [async-std](https://async.rs/) adepts

## Features

- *fromstr* - implements [core::str::FromStr] implementation for some types
- *display* - implements [std::fmt::Display] implementation for some types
- *phf* - use [phf](https://crates.io/crates/phf) in [core::str::FromStr] trait implementations
- *serde* - enables [serde](https://crates.io/crates/serde) support for some types
- *keyboard* - enables keyboard class support
- *mouse* - enables mouse class support
