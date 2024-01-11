#![forbid(future_incompatible)]
#![deny(bad_style, missing_docs)]
#![doc = include_str!("../README.md")]

use hidg_core::{check_read, check_write, AsDevicePath};

pub use hidg_core::{Class, Error, Result, StateChange, ValueChange};

#[cfg(feature = "keyboard")]
pub use hidg_core::{
    Key, KeyStateChanges, Keyboard, KeyboardInput, KeyboardOutput, Led, LedStateChanges, Leds,
    Modifiers,
};

#[cfg(feature = "mouse")]
pub use hidg_core::{
    Button, Buttons, Mouse, MouseInput, MouseInputChange, MouseInputChanges, MouseOutput,
};

use core::marker::PhantomData;
use std::{
    fs::File,
    io::{Read, Write},
};

use tokio::{fs::OpenOptions, io::unix::AsyncFd};

/// HID Gadget Device
pub struct Device<C: Class> {
    file: AsyncFd<File>,
    _class: PhantomData<C>,
}

impl<C: Class> Device<C> {
    /// Open device by path or name or number
    pub async fn open(device: impl AsDevicePath) -> Result<Self> {
        let path = device.as_device_path();
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(path)
            .await?
            .into_std()
            .await;
        let file = AsyncFd::new(file)?;
        Ok(Self {
            file,
            _class: PhantomData,
        })
    }

    /// Send input report
    pub async fn input(&mut self, input: &C::Input) -> Result<()>
    where
        C::Input: AsRef<[u8]>,
    {
        let _ = self.file.writable().await?;
        let raw = input.as_ref();
        let len = self.file.get_ref().write(raw)?;

        check_write(len, raw.len())
    }

    /// Receive output report
    pub async fn output(&mut self, output: &mut C::Output) -> Result<()>
    where
        C::Output: AsMut<[u8]>,
    {
        let _ = self.file.readable().await?;
        let raw = output.as_mut();
        let len = self.file.get_ref().read(raw)?;

        check_read(len, raw.len())?;

        Ok(())
    }
}
