use crate::types::Update;
use std::future::Future;

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
