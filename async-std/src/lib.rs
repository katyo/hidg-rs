#![forbid(future_incompatible)]
#![deny(bad_style, missing_docs)]
#![doc = include_str!("../README.md")]

use hidg_core::{check_read, check_write, open};

pub use hidg_core::{Class, Error, Result, StateChange, ValueChange};

#[cfg(feature = "keyboard")]
pub use hidg_core::{Key, Keyboard, KeyboardInput, KeyboardOutput, Led, Leds, Modifiers};

#[cfg(feature = "mouse")]
pub use hidg_core::{Button, Buttons, Mouse, MouseInput, MouseOutput};

use core::marker::PhantomData;
use std::{
    os::unix::io::{AsRawFd, RawFd},
    pin::Pin,
    task::{Context, Poll},
};

use async_io::Async;
use async_std::{
    io::{Read, ReadExt, Write, WriteExt},
    path::Path,
    task::spawn_blocking as asyncify,
};

#[doc(hidden)]
pub struct File {
    // use file to call close when drop
    inner: Async<std::fs::File>,
}

impl File {
    pub fn from_file(file: std::fs::File) -> Result<Self> {
        Ok(Self {
            inner: Async::new(file)?,
        })
    }
}

impl AsRawFd for File {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}

impl Read for File {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<Result<usize>> {
        use std::io::Read;

        match self.inner.poll_readable(cx) {
            Poll::Ready(x) => x,
            Poll::Pending => return Poll::Pending,
        }?;

        Poll::Ready(self.inner.get_ref().read(buf))
    }
}

impl Write for File {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
        use std::io::Write;

        match self.inner.poll_writable(cx) {
            Poll::Ready(x) => x,
            Poll::Pending => return Poll::Pending,
        }?;

        Poll::Ready(self.inner.get_ref().write(buf))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }
}

/// HID Gadget Device
pub struct Device<C: Class> {
    file: File,
    _class: PhantomData<C>,
}

impl<C: Class> Device<C> {
    /// Open device by path or name
    pub async fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_owned();
        let file = asyncify(move || open(path, false)).await?;
        let file = File::from_file(file)?;
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
        let raw = input.as_ref();
        let len = self.file.write(raw).await?;

        check_write(len, raw.len())
    }

    /// Receive output report
    pub async fn output(&mut self, output: &mut C::Output) -> Result<()>
    where
        C::Output: AsMut<[u8]>,
    {
        let raw = output.as_mut();
        let len = self.file.read(raw).await?;

        check_read(len, raw.len())?;

        Ok(())
    }
}
