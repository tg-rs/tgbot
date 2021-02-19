use crate::types::Update;
use futures_util::future::BoxFuture;
use std::{future::Future, sync::Arc};
use tokio::sync::Mutex;

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

/// Wrapper for [`UpdateHandler`] that doesn't implement [`Sync`]
///
/// Uses [`Arc`] and [tokio Mutex] internally.
/// Useful for [`webhook::run_server`] that require handler is sync
///
/// [`UpdateHandler`]: UpdateHandler
/// [`Sync`]: Sync
/// [`webhook::run_server`]: crate::webhook::run_server
/// [`Arc`]: Arc
/// [tokio Mutex]: tokio::sync::Mutex
pub struct SyncedUpdateHandler<T> {
    handler: Arc<Mutex<T>>,
}

impl<T> SyncedUpdateHandler<T> {
    /// Creates a new SyncedUpdateHandler
    pub fn new(handler: T) -> Self {
        Self {
            handler: Arc::new(Mutex::new(handler)),
        }
    }
}

impl<T> UpdateHandler for SyncedUpdateHandler<T>
where
    T: UpdateHandler + Send + 'static,
{
    type Future = BoxFuture<'static, ()>;

    fn handle(&self, update: Update) -> Self::Future {
        let handler = Arc::clone(&self.handler);
        Box::pin(async move {
            let handler = handler.lock().await;
            handler.handle(update);
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_send<T: Send>() {}

    fn assert_sync<T: Sync>() {}

    #[test]
    fn synced_is_sync() {
        // pointer is neither Send nor Sync
        struct NotSync(*mut ());

        unsafe impl Send for NotSync {}

        impl UpdateHandler for NotSync {
            type Future = BoxFuture<'static, ()>;

            fn handle(&self, _update: Update) -> Self::Future {
                Box::pin(async {})
            }
        }

        assert_send::<NotSync>();
        assert_send::<SyncedUpdateHandler<NotSync>>();
        assert_sync::<SyncedUpdateHandler<NotSync>>();
    }
}
