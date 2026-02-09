use std::future::Future;
use std::sync::Arc;

use crate::types::Update;

pub use self::longpoll::*;
#[cfg(feature = "webhook")]
pub use self::webhook::*;

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

impl<T> UpdateHandler for Arc<T>
where T: UpdateHandler + Send + Sync {
    async fn handle(&self, update: Update) {
        self.as_ref().handle(update).await
    }
}
