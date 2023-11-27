#![cfg_attr(nightly, feature(doc_cfg))]
#![doc = include_str!("../README.md")]

/// An HTTP client implementation
pub mod api;

/// Update handlers
pub mod handler;

/// Telegram Bot API types
pub mod types;
