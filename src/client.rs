//! `Client` — synchronous text shuttle to the nexus daemon.
//!
//! Opens a Unix-socket connection to the daemon, writes the
//! input text, half-closes the write side so the daemon sees
//! EOF and renders its reply, reads the reply back, returns
//! it. One connection per [`Client::shuttle`] call — the CLI is
//! one-shot per invocation per nexus-cli/ARCH §"Invariants".

use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::path::PathBuf;

use crate::error::Result;

pub struct Client {
    socket_path: PathBuf,
}

impl Client {
    pub fn new(socket_path: PathBuf) -> Self {
        Self { socket_path }
    }

    /// Send `input` text to the daemon, return the reply text.
    pub fn shuttle(&self, input: &str) -> Result<String> {
        let mut stream = UnixStream::connect(&self.socket_path)?;
        stream.write_all(input.as_bytes())?;
        stream.shutdown(std::net::Shutdown::Write)?;
        let mut reply = String::new();
        stream.read_to_string(&mut reply)?;
        Ok(reply)
    }
}
