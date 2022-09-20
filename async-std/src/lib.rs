#![forbid(future_incompatible)]
#![deny(bad_style, missing_docs)]
#![doc = include_str!("../README.md")]

use hidg_core::{check_read, check_write, open};

pub use hidg_core::{Class, Error, Result, StateChange, ValueChange};

#[cfg(feature = "keyboard")]
pub use hidg_core::{Key, Keyboard, KeyboardInput, KeyboardOutput, Led, Leds, Modifiers};

#[cfg(feature = "mouse")]
pub use hidg_core::{Button, Buttons, Mouse, MouseInput, MouseOutput};

use std::{
    os::unix::io::{AsRawFd, RawFd},
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use async_io::Async;
use async_std::{
    io::{Read, ReadExt, Write, WriteExt},
    path::Path,
    sync::Mutex,
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
    pub async fn open(class: C, path: impl AsRef<Path>) -> Result<Self>
    where
        C: Copy,
    {
        let path = path.as_ref().to_owned();
        let file = asyncify(move || open(path, false)).await?;
        let file = File::from_file(file)?;
        let state = Arc::new(State {
            class,
            input: Mutex::new(class.input()),
            output: Mutex::new(class.output()),
        });
        Ok(Self { state, file })
    }

    /// Get reference to actual input report
    pub async fn input(&self) -> C::Input
    where
        C::Input: Copy,
    {
        *self.state.input.lock().await
    }

    /// Get reference to actual output report
    pub async fn output(&self) -> C::Output
    where
        C::Output: Copy,
    {
        *self.state.output.lock().await
    }

    /// Update input report
    pub async fn update<T>(&mut self, input: T) -> Result<()>
    where
        C::Input: AsRef<[u8]> + Extend<T>,
    {
        self.updates(core::iter::once(input)).await
    }

    /// Update input report
    pub async fn updates<T>(&mut self, input: impl IntoIterator<Item = T>) -> Result<()>
    where
        C::Input: AsRef<[u8]> + Extend<T>,
    {
        let report = {
            let mut report = self.state.input.lock().await;
            report.extend(input.into_iter());
            report
        };

        let raw = report.as_ref();
        let len = self.file.write(raw).await?;

        check_write(len, raw.len())
    }

    /// Await output report
    pub async fn updated(&mut self) -> Result<()>
    where
        C::Output: AsMut<[u8]>,
    {
        let mut report = self.state.class.output();
        let raw = report.as_mut();
        let len = self.file.read(raw).await?;

        check_read(len, raw.len())?;

        *self.state.output.lock().await = report;

        Ok(())
    }
}
