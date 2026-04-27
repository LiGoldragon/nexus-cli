//! `nexus` — text-shuttle CLI for the nexus daemon.
//!
//! Usage:
//!   `nexus <file.nexus>` — read the file as nexus text.
//!   `nexus`              — read stdin until EOF.
//!
//! The shuttled reply text is written to stdout. The daemon
//! socket is `$NEXUS_SOCKET` or `/tmp/nexus.sock` by default.

use std::io::{Read, Write};
use std::path::PathBuf;

use nexus_cli::{Client, Result};

const DEFAULT_NEXUS_SOCKET: &str = "/tmp/nexus.sock";

fn main() -> Result<()> {
    let socket_path: PathBuf = std::env::var("NEXUS_SOCKET")
        .unwrap_or_else(|_| DEFAULT_NEXUS_SOCKET.to_string())
        .into();

    let mut input = String::new();
    match std::env::args().nth(1) {
        Some(path) => input = std::fs::read_to_string(&path)?,
        None => {
            std::io::stdin().read_to_string(&mut input)?;
        }
    }

    let reply = Client::new(socket_path).shuttle(&input)?;
    std::io::stdout().write_all(reply.as_bytes())?;
    Ok(())
}
