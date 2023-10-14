//! A Telegram Bot API client library
#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

/// An HTTP client implementation
pub mod api;

/// Update handlers
pub mod handler;

/// Types and methods available in the Bot API
pub mod types;
