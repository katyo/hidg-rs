#![forbid(future_incompatible)]
#![deny(bad_style, missing_docs)]
#![doc = include_str!("../README.md")]

use core::marker::PhantomData;
use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
};

use hidg_core::{check_read, check_write, AsDevicePath};

pub use hidg_core::{Class, Result, StateChange, ValueChange};

#[cfg(feature = "keyboard")]
pub use hidg_core::{
    Key, KeyStateChanges, Keyboard, KeyboardInput, KeyboardOutput, Led, LedStateChanges, Leds,
    Modifiers,
};

#[cfg(feature = "mouse")]
pub use hidg_core::{
    Button, Buttons, Mouse, MouseInput, MouseInputChange, MouseInputChanges, MouseOutput,
};

/// HID Gadget Device
pub struct Device<C: Class> {
    file: File,
    _class: PhantomData<C>,
}

impl<C: Class> Device<C> {
    /// Open device by path or name or number
    pub fn open(device: impl AsDevicePath) -> Result<Self> {
        let path = device.as_device_path();

        let file = OpenOptions::new().read(true).write(true).open(path)?;

        Ok(Self {
            file,
            _class: PhantomData,
        })
    }

    /// Send input report
    pub fn input(&mut self, input: &C::Input) -> Result<()>
    where
        C::Input: AsRef<[u8]>,
    {
        let raw = input.as_ref();
        let len = self.file.write(raw)?;

        check_write(len, raw.len())
    }

    /// Receive output report
    pub fn output(&mut self, output: &mut C::Output) -> Result<()>
    where
        C::Output: AsMut<[u8]>,
    {
        let raw = output.as_mut();
        let len = self.file.read(raw)?;

        check_read(len, raw.len())?;

        Ok(())
    }

    /// Try clone device
    pub fn try_clone(&self) -> Result<Self> {
        let file = self.file.try_clone()?;

        Ok(Self {
            file,
            _class: PhantomData,
        })
    }
}
