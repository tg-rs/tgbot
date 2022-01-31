//! A Telegram Bot API client library
#![doc = include_str!("../README.md")]
#![recursion_limit = "256"]
#![warn(missing_docs)]

mod api;
mod config;
mod handler;
mod request;

/// Utilities to receive updates using long poll
pub mod longpoll;

/// Methods available in the Bot API
pub mod methods;

/// Types available in the Bot API
pub mod types;

/// Services to receive updates via webhook
pub mod webhook;

pub use self::{
    api::{Api, ApiError, DownloadFileError, ExecuteError},
    config::{Config, ParseProxyError},
    handler::{SyncedUpdateHandler, UpdateHandler},
};

pub use mime;
pub use vec1::Vec1;
