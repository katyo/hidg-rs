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
pub use keyboard::{
    Key, KeyStateChanges, Keyboard, KeyboardInput, KeyboardOutput, Led, LedStateChanges, Leds,
    Modifiers,
};

#[cfg(feature = "mouse")]
pub use mouse::{
    Button, Buttons, Mouse, MouseInput, MouseInputChange, MouseInputChanges, MouseOutput,
};

use std::{
    io::ErrorKind,
    path::{Path, PathBuf},
};

pub use std::io::{Error, Result};

/// Unknown error
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Unknown;

impl std::error::Error for Unknown {}

impl core::fmt::Display for Unknown {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str("Unknown")
    }
}

/// Key/button/LED state change event
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    /// Create new on event
    pub fn on(data: T) -> Self {
        Self::new(data, true)
    }

    /// Create new release event
    pub fn release(data: T) -> Self {
        Self::new(data, false)
    }

    /// Create new off event
    pub fn off(data: T) -> Self {
        Self::new(data, false)
    }

    /// Get data
    pub fn data(&self) -> T
    where
        T: Copy,
    {
        self.data
    }

    /// Get state
    pub fn state(&self) -> bool {
        self.state
    }

    /// Is key/button press event
    pub fn is_press(&self) -> bool {
        self.state
    }

    /// Is LED on event
    pub fn is_on(&self) -> bool {
        self.state
    }

    /// Is key/button release event
    pub fn is_release(&self) -> bool {
        !self.state
    }

    /// Is LED off event
    pub fn is_off(&self) -> bool {
        !self.state
    }
}

impl<T> From<(T, bool)> for StateChange<T> {
    fn from((data, state): (T, bool)) -> Self {
        Self { data, state }
    }
}

impl<T> From<StateChange<T>> for (T, bool) {
    fn from(StateChange { data, state }: StateChange<T>) -> Self {
        (data, state)
    }
}

/// Pointer/cursor position change event
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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

    /// Get underlying data
    pub fn data(&self) -> T
    where
        T: Copy,
    {
        self.data
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

impl<T> From<(T, bool)> for ValueChange<T> {
    fn from((data, relative): (T, bool)) -> Self {
        Self { data, relative }
    }
}

impl<T> From<ValueChange<T>> for (T, bool) {
    fn from(ValueChange { data, relative }: ValueChange<T>) -> Self {
        (data, relative)
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

/// Device path trait
pub trait AsDevicePath {
    /// Get absolute device path
    fn as_device_path(&self) -> PathBuf;
}

impl AsDevicePath for Path {
    fn as_device_path(&self) -> PathBuf {
        if self.is_absolute() {
            self.to_path_buf()
        } else {
            Path::new("/dev").join(self)
        }
    }
}

impl AsDevicePath for &Path {
    fn as_device_path(&self) -> PathBuf {
        if self.is_absolute() {
            self.to_path_buf()
        } else {
            Path::new("/dev").join(self)
        }
    }
}

impl AsDevicePath for PathBuf {
    fn as_device_path(&self) -> PathBuf {
        let path: &Path = self;
        path.as_device_path()
    }
}

impl AsDevicePath for &PathBuf {
    fn as_device_path(&self) -> PathBuf {
        let path: &Path = self;
        path.as_device_path()
    }
}

impl AsDevicePath for &str {
    fn as_device_path(&self) -> PathBuf {
        Path::new(self).as_device_path()
    }
}

impl AsDevicePath for String {
    fn as_device_path(&self) -> PathBuf {
        let s: &str = self;
        s.as_device_path()
    }
}

impl AsDevicePath for &String {
    fn as_device_path(&self) -> PathBuf {
        let s: &str = self;
        s.as_device_path()
    }
}

impl AsDevicePath for usize {
    fn as_device_path(&self) -> PathBuf {
        format!("/dev/hidg{self}").as_device_path()
    }
}

macro_rules! as_device_path {
    ($($type:ty),*) => {
        $(
            impl AsDevicePath for $type {
                fn as_device_path(&self) -> PathBuf {
                    (*self as usize).as_device_path()
                }
            }
        )*
    };
}

as_device_path!(u8, u16, u32, u64, i8, i16, i32, i64, isize);

/// Wrapper to hide internals
#[derive(Clone, Copy, Default)]
pub struct Internal<T>(T);

impl<T> core::ops::Deref for Internal<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> core::ops::DerefMut for Internal<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
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
