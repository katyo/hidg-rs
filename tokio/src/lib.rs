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
    io,
    os::unix::io::{AsRawFd, RawFd},
    path::Path,
    pin::Pin,
    task::{Context, Poll},
};

use tokio::{
    io::{unix::AsyncFd, AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf},
    task::spawn_blocking,
};

async fn asyncify<F, T>(f: F) -> Result<T>
where
    F: FnOnce() -> Result<T> + Send + 'static,
    T: Send + 'static,
{
    match spawn_blocking(f).await {
        Ok(res) => res,
        Err(_) => Err(Error::new(io::ErrorKind::Other, "background task failed")),
    }
}

#[doc(hidden)]
pub struct File {
    // use file to call close when drop
    inner: AsyncFd<std::fs::File>,
}

impl File {
    pub fn from_file(file: std::fs::File) -> Result<Self> {
        Ok(Self {
            inner: AsyncFd::new(file)?,
        })
    }
}

impl AsRawFd for File {
    fn as_raw_fd(&self) -> RawFd {
        self.inner.as_raw_fd()
    }
}

impl AsyncRead for File {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<Result<()>> {
        loop {
            use std::io::Read;

            let mut guard = match self.inner.poll_read_ready(cx) {
                Poll::Ready(x) => x,
                Poll::Pending => return Poll::Pending,
            }?;

            match guard.try_io(|inner| inner.get_ref().read(buf.initialize_unfilled())) {
                Ok(Ok(bytes_read)) => {
                    buf.advance(bytes_read);
                    return Poll::Ready(Ok(()));
                }
                Ok(Err(err)) => {
                    return Poll::Ready(Err(err));
                }
                Err(_would_block) => continue,
            }
        }
    }
}

impl AsyncWrite for File {
    fn poll_write(self: Pin<&mut Self>, cx: &mut Context<'_>, buf: &[u8]) -> Poll<Result<usize>> {
        use std::io::Write;

        loop {
            let mut guard = match self.inner.poll_write_ready(cx) {
                Poll::Ready(x) => x,
                Poll::Pending => return Poll::Pending,
            }?;

            match guard.try_io(|inner| inner.get_ref().write(buf)) {
                Ok(result) => return Poll::Ready(result),
                Err(_would_block) => continue,
            }
        }
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<()>> {
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
