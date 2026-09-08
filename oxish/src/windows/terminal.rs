//! PTY-backed terminal on Windows
//!
//! TODO: Replace stub for real implementation

use core::{
    convert::Infallible,
    task::{Context, Poll},
};
use std::io;

use proto::channels::{PtyReq, WindowChange};

pub(crate) struct Terminal(Infallible);

impl Terminal {
    pub(crate) fn spawn(_: &PtyReq<'_>, _: &[(String, String)]) -> io::Result<Self> {
        Err(io::Error::new(
            io::ErrorKind::Unsupported,
            "PTY support is not implemented on Windows",
        ))
    }

    /// Resize the PTY window (in response to a window-change request)
    pub(crate) fn resize(&self, _: &WindowChange) -> io::Result<()> {
        match self.0 {}
    }

    /// Write data to the PTY (sends input to the shell)
    pub(crate) async fn write(&self, _: &[u8]) -> io::Result<()> {
        match self.0 {}
    }

    /// Read data from the PTY (receives output from the shell)
    pub(crate) fn poll_read(
        &mut self,
        _: &mut [u8],
        _: &mut Context<'_>,
    ) -> Poll<io::Result<usize>> {
        match self.0 {}
    }

    pub(crate) fn poll_kill(self, _: &mut Context<'_>) -> Poll<Result<(), io::Error>> {
        match self.0 {}
    }
}
