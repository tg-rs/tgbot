//! A Telegram Bot API client library
#![doc = include_str!("../README.md")]
#![recursion_limit = "256"]
#![warn(missing_docs)]

pub use mime;

/// An HTTP client implementation
pub mod api;

/// Update handlers
pub mod handler;

/// Types and methods available in the Bot API
pub mod types;
