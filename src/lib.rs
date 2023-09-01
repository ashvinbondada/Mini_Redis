//! A minimal (i.e. very incomplete) implementation of a Redis server and
//! client.

pub mod clients;
pub use clients::{BlockingClient, BufferedClient, Client};

pub mod cmd;
pub use cmd::Command;

mod connection;
pub use connection::Connection;

pub mod frame;
pub use frame::Frame;

mod db;
use db::Db;
use db::DbDropGuard;

mod parse;
use parse::{Parse, ParseError};

pub mod server;

mod shutdown;
use shutdown::Shutdown;

/// Default port that a redis server listens on.
///
/// Used if no port is specified.
pub const DEFAULT_PORT: u16 = 6379;

/// Error returned by most functions.
///
/// For performance reasons, boxing is avoided in any hot path. For example, in
/// `parse`, a custom error `enum` is defined. This is because the error is hit
/// and handled during normal execution when a partial frame is received on a
/// socket. `std::error::Error` is implemented for `parse::Error` which allows
/// it to be converted to `Box<dyn std::error::Error>`.
pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// A specialized `Result` type for mini-redis operations.
pub type Result<T> = std::result::Result<T, Error>;
