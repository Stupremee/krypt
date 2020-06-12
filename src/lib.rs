//! The krypt library contains all stuff that is required
//! for the krypt binary to work.
#![warn(rust_2018_idioms)]
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

pub mod chunk;
pub mod hash;

/// Common Result type that will match all errors
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
