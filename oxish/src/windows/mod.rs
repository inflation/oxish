//! Windows implementations of what the server needs from the host OS
//!
//! Every entry point here is a stub: the platform is wired up far enough for the library and
//! its binaries to build, but nothing yet knows how to resolve a Windows account or hand an
//! authenticated session to a child process.

mod terminal;

use std::io;

use proto::{ServerHostKey, crypto::CryptoProvider};
use tokio::{net::TcpStream, process::Child};

pub(crate) use terminal::Terminal;

use crate::{Error, Server, Session, SessionState, User, authentication::UserStore};

/// Default [`UserStore`] implementation
pub struct DefaultStore(());

impl DefaultStore {
    /// Construct a new [`DefaultStore`] for the account this process runs as
    #[expect(clippy::new_ret_no_self)]
    pub fn new(_: &dyn CryptoProvider) -> Result<Box<dyn UserStore>, Error> {
        Err(unsupported("user lookup"))
    }
}

/// Resume an SSH session from the session state handed to this process
pub fn resume(_: &'static dyn CryptoProvider) -> Result<Session<TcpStream>, Error> {
    Err(unsupported("session handoff"))
}

/// Spawn a child process for the authenticated session
///
/// Unreachable while [`SPAWN_SESSIONS`] is false: the session keeps running in the server
/// process instead.
pub(crate) async fn spawn(
    _: SessionState<ServerHostKey<'_>>,
    _: TcpStream,
    _: User,
    _: &Server,
) -> Result<Child, Error> {
    Err(unsupported("session handoff"))
}

fn unsupported(what: &str) -> Error {
    Error::Io(io::Error::new(
        io::ErrorKind::Unsupported,
        format!("{what} is not implemented on Windows"),
    ))
}
