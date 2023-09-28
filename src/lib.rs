//! A Telegram Bot API client library
#![doc = include_str!("../README.md")]
#![recursion_limit = "256"]
#![warn(missing_docs)]

pub use mime;

pub use self::{
    api::{Api, ApiError, DownloadFileError, ExecuteError},
    handler::{SyncedUpdateHandler, UpdateHandler},
    method::Method,
};

mod api;
mod handler;
mod method;
mod request;

pub(crate) mod form;

/// Utilities to receive updates using long poll
pub mod longpoll;

/// Types and methods available in the Bot API
pub mod types;

/// Services to receive updates via webhook
pub mod webhook;
