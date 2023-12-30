use std::future::Future;

pub use self::longpoll::*;
#[cfg(feature = "webhook")]
pub use self::webhook::*;
use crate::types::Update;

mod longpoll;

#[cfg(feature = "webhook")]
mod webhook;

/// Represents an update handler for processing updates received from the Telegram Bot API.
pub trait UpdateHandler {
    /// Handles a received update.
    ///
    /// # Arguments
    ///
    /// * `update` - The received update from the Telegram Bot API.
    fn handle(&self, update: Update) -> impl Future<Output = ()> + Send;
}
