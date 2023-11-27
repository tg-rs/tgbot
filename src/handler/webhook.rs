use std::{io::Error as IoError, net::SocketAddr};

use axum::Router;
use tokio::net::TcpListener;

use crate::{handler::UpdateHandler, types::Update};

/// Represents a simple webhook server for handling incoming updates from the Telegram Bot API.
#[cfg_attr(nightly, doc(cfg(feature = "webhook")))]
pub struct WebhookServer {
    router: Router,
}

impl WebhookServer {
    /// Creates a new `WebhookServer`.
    ///
    /// # Arguments
    ///
    /// * `path` - The path where the webhook server will receive incoming updates.
    /// * `handler` - The handler for processing updates.
    pub fn new<A, B>(path: A, handler: B) -> Self
    where
        A: AsRef<str>,
        B: UpdateHandler + Clone + Send + Sync + 'static,
        B::Future: Send,
    {
        let router = Router::new()
            .route(path.as_ref(), axum::routing::post(handle_update::<B>))
            .layer(axum::Extension(handler));
        Self { router }
    }

    /// Runs the server
    ///
    /// Returns the local address that the server is bound to.
    ///
    /// # Arguments
    ///
    /// * `address` - The address to bind the server to.
    pub async fn run<T>(self, address: T) -> Result<SocketAddr, IoError>
    where
        T: Into<SocketAddr>,
    {
        let listener = TcpListener::bind(address.into()).await?;
        let result = listener.local_addr();
        axum::serve(listener, self.router).await?;
        result
    }
}

impl From<WebhookServer> for Router {
    fn from(value: WebhookServer) -> Self {
        value.router
    }
}

async fn handle_update<H>(handler: axum::Extension<H>, axum::extract::Json(update): axum::extract::Json<Update>)
where
    H: UpdateHandler,
{
    handler.handle(update).await
}
