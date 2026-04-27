//! nexus-cli — thin text-shuttle client for the nexus daemon.
//!
//! Stateless: every invocation opens a new connection, writes
//! the input text, reads the reply text, exits. The daemon
//! parses + forwards to criome + renders the reply per
//! nexus/ARCH; this crate just shuttles the bytes.

pub mod client;
pub mod error;

pub use client::Client;
pub use error::{Error, Result};
