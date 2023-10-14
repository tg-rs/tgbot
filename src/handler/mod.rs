use std::future::Future;

use crate::types::Update;

pub use self::{longpoll::*, webhook::*};

mod longpoll;
mod webhook;

/// An update handler
pub trait UpdateHandler {
    /// A future returned by `handle` method
    type Future: Future<Output = ()>;

    /// Handles an update
    ///
    /// # Arguments
    ///
    /// * update - A received update
    fn handle(&self, update: Update) -> Self::Future;
}
