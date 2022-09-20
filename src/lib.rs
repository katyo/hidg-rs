#![forbid(future_incompatible)]
#![deny(bad_style, missing_docs)]
#![doc = include_str!("../README.md")]

use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
    sync::{Arc, Mutex},
};

use hidg_core::{check_read, check_write, open};

pub use hidg_core::{Class, Result, StateChange, ValueChange};

#[cfg(feature = "keyboard")]
pub use hidg_core::{Key, Keyboard, KeyboardInput, KeyboardOutput, Led, Leds, Modifiers};

#[cfg(feature = "mouse")]
pub use hidg_core::{Button, Buttons, Mouse, MouseInput, MouseOutput};

struct State<C: Class> {
    input: Mutex<C::Input>,
    output: Mutex<C::Output>,
    class: C,
}

/// HID Gadget Device
pub struct Device<C: Class> {
    state: Arc<State<C>>,
    file: File,
}

impl<C: Class> Device<C> {
    /// Open device by path or name
    pub fn open(class: C, path: impl AsRef<Path>) -> Result<Self>
    where
        C: Copy,
    {
        let file = open(path, false)?;
        let state = Arc::new(State {
            class,
            input: Mutex::new(class.input()),
            output: Mutex::new(class.output()),
        });
        Ok(Self { state, file })
    }

    /// Get reference to actual input report
    pub fn input(&self) -> C::Input
    where
        C::Input: Copy,
    {
        *self.state.input.lock().unwrap()
    }

    /// Get reference to actual output report
    pub fn output(&self) -> C::Output
    where
        C::Output: Copy,
    {
        *self.state.output.lock().unwrap()
    }

    /// Update input report
    pub fn update<T>(&mut self, input: T) -> Result<()>
    where
        C::Input: AsRef<[u8]> + Extend<T>,
    {
        self.updates(core::iter::once(input))
    }

    /// Update input report
    pub fn updates<T>(&mut self, input: impl IntoIterator<Item = T>) -> Result<()>
    where
        C::Input: AsRef<[u8]> + Extend<T>,
    {
        let report = {
            let mut report = self.state.input.lock().unwrap();
            report.extend(input.into_iter());
            report
        };

        let raw = report.as_ref();
        let len = self.file.write(raw)?;

        check_write(len, raw.len())
    }

    /// Await output report
    pub fn updated(&mut self) -> Result<()>
    where
        C::Output: AsMut<[u8]>,
    {
        let mut report = self.state.class.output();
        let raw = report.as_mut();
        let len = self.file.read(raw)?;

        check_read(len, raw.len())?;

        *self.state.output.lock().unwrap() = report;

        Ok(())
    }

    /// Try clone device
    pub fn try_clone(&self) -> Result<Self> {
        let file = self.file.try_clone()?;
        let state = self.state.clone();

        Ok(Self { state, file })
    }
}
