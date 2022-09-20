#![forbid(future_incompatible)]
#![deny(bad_style, missing_docs)]
#![doc = include_str!("../README.md")]

#[cfg(not(target_os = "linux"))]
compile_error!("This crate support Linux only");

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[macro_use]
mod macros;

#[cfg(feature = "either")]
mod either_report;

#[cfg(feature = "keyboard")]
mod keyboard;

#[cfg(feature = "mouse")]
mod mouse;

#[cfg(feature = "keyboard")]
pub use keyboard::{Key, Keyboard, KeyboardInput, KeyboardOutput, Led, Leds, Modifiers};

#[cfg(feature = "mouse")]
pub use mouse::{Button, Buttons, Mouse, MouseInput, MouseOutput};

use std::{
    fs::{File, OpenOptions},
    io::ErrorKind,
    os::unix::fs::OpenOptionsExt,
    path::Path,
};

pub use std::io::{Error, Result};

/// Unknown error
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Unknown;

impl std::error::Error for Unknown {}

impl std::fmt::Display for Unknown {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str("Unknown")
    }
}

/// Key/button/LED state change event
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct StateChange<T> {
    data: T,
    state: bool,
}

impl<T> StateChange<T> {
    /// Create new state change event
    pub fn new(data: T, state: bool) -> Self {
        Self { data, state }
    }

    /// Create new press event
    pub fn press(data: T) -> Self {
        Self::new(data, true)
    }

    /// Create new release event
    pub fn release(data: T) -> Self {
        Self::new(data, false)
    }

    /// Is key/button press event
    pub fn is_press(&self) -> bool {
        self.state
    }

    /// Is key/button release event
    pub fn is_release(&self) -> bool {
        !self.state
    }
}

/// Pointer/cursor position change event
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ValueChange<T> {
    data: T,
    #[cfg_attr(feature = "serde", serde(rename = "rel"))]
    relative: bool,
}

impl<T> ValueChange<T> {
    /// Create new value change event
    pub fn new(data: T, relative: bool) -> Self {
        Self { data, relative }
    }

    /// Create new absolute value change event
    pub fn absolute(data: T) -> Self {
        Self::new(data, false)
    }

    /// Create new relative value change event
    pub fn relative(data: T) -> Self {
        Self::new(data, true)
    }

    /// Is value change relative
    pub fn is_relative(&self) -> bool {
        self.relative
    }

    /// Is value change absolute
    pub fn is_absolute(&self) -> bool {
        !self.relative
    }
}

deref_impl! {
    StateChange<T> => data: T,
    ValueChange<T> => data: T,
}

/// Device class trait
pub trait Class {
    /// Input report type
    type Input;

    /// Output report type
    type Output;

    /// Create input report
    fn input(&self) -> Self::Input;

    /// Create output report
    fn output(&self) -> Self::Output;
}

/// Open device by path or name
pub fn open(path: impl AsRef<Path>, nonblock: bool) -> Result<File> {
    let path = path.as_ref();

    #[allow(unused)]
    let mut full_path = None;

    let path = if path.is_absolute() {
        path
    } else {
        full_path = Some(Path::new("dev").join(path));
        full_path.as_ref().unwrap()
    };

    pub const O_NONBLOCK: i32 = 2048;

    OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(if nonblock { O_NONBLOCK } else { 0 })
        .open(path)
}

/// Check write report length
pub fn check_write(actual: usize, expected: usize) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "Error when writing report"))
    }
}

/// Check read report length
pub fn check_read(actual: usize, expected: usize) -> Result<()> {
    if actual == expected {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Other, "Error when reading report"))
    }
}
