use std::{
    convert::Infallible,
    error::Error,
    future::Future,
    net::SocketAddr,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};

use bytes::Buf;
use futures_util::future::{ok, BoxFuture, Ready};
use http::Error as HttpError;
pub use hyper::Error as HyperError;
use hyper::{body, service::Service, Body, Method, Request, Response, Server, StatusCode};
use log::error;
use tokio::sync::Mutex;

use crate::{handler::UpdateHandler, types::Update};

#[doc(hidden)]
pub struct WebhookServiceFactory<H> {
    path: String,
    handler: Arc<H>,
}

impl<H> WebhookServiceFactory<H> {
    #[doc(hidden)]
    pub fn new<P>(path: P, update_handler: H) -> Self
    where
        P: Into<String>,
    {
        WebhookServiceFactory {
            path: path.into(),
            handler: Arc::new(update_handler),
        }
    }
}

impl<H, T> Service<T> for WebhookServiceFactory<H> {
    type Response = WebhookService<H>;
    type Error = Infallible;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, _: T) -> Self::Future {
        let path = self.path.clone();
        ok(WebhookService {
            path,
            handler: self.handler.clone(),
        })
    }
}

#[doc(hidden)]
pub struct WebhookService<H> {
    path: String,
    handler: Arc<H>,
}

impl<H> Clone for WebhookService<H> {
    fn clone(&self) -> Self {
        Self {
            path: self.path.clone(),
            handler: self.handler.clone(),
        }
    }
}

async fn handle_request<H>(
    handler: Arc<H>,
    path: String,
    request: Request<Body>,
) -> Result<Response<Body>, WebhookError>
where
    H: UpdateHandler,
    H::Future: Send,
{
    Ok(if let Method::POST = *request.method() {
        if request.uri().path() == path {
            let data = body::aggregate(request).await?;
            match serde_json::from_reader(data.reader()) {
                Ok(update) => {
                    handler.handle(update).await;
                    Response::new(Body::empty())
                }
                Err(err) => Response::builder()
                    .header("Content-Type", "text/plain")
                    .status(StatusCode::BAD_REQUEST)
                    .body(Body::from(format!("Failed to parse update: {}\n", err)))?,
            }
        } else {
            Response::builder().status(StatusCode::NOT_FOUND).body(Body::empty())?
        }
    } else {
        Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .header("Allow", "POST")
            .body(Body::empty())?
    })
}

type ServiceFuture = Pin<Box<dyn Future<Output = Result<Response<Body>, WebhookError>> + Send>>;

impl<H> Service<Request<Body>> for WebhookService<H>
where
    H: UpdateHandler + Send + Sync + 'static,
    H::Future: Send,
{
    type Response = Response<Body>;
    type Error = WebhookError;
    type Future = ServiceFuture;

    fn poll_ready(&mut self, _: &mut Context) -> Poll<Result<(), Self::Error>> {
        Ok(()).into()
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let this = self.clone();
        Box::pin(async move {
            let result = handle_request(this.handler, this.path, request).await;
            match result {
                Ok(rep) => Ok(rep),
                Err(err) => {
                    error!("Webhook error: {}", err);
                    Response::builder()
                        .status(StatusCode::INTERNAL_SERVER_ERROR)
                        .body(Body::empty())
                        .map_err(WebhookError::from)
                }
            }
        })
    }
}

#[doc(hidden)]
#[derive(Debug, derive_more::Display, derive_more::From)]
pub enum WebhookError {
    Hyper(HyperError),
    Http(HttpError),
}

impl Error for WebhookError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::WebhookError::*;
        Some(match self {
            Hyper(err) => err,
            Http(err) => err,
        })
    }
}

/// A wrapper for non-sync [`UpdateHandler`].
///
/// Useful for [`run_server`] which requires a sync handler.
pub struct SyncedUpdateHandler<T> {
    handler: Arc<Mutex<T>>,
}

impl<T> From<T> for SyncedUpdateHandler<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> SyncedUpdateHandler<T> {
    /// Creates a new SyncedUpdateHandler.
    ///
    /// # Arguments
    ///
    /// * `handler` - A non-sync handler.
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

/// Starts a server for handling incoming updates via webhook.
///
/// # Arguments
///
/// * `address` - The bind address for the server.
/// * `path` - The URL path for the webhook endpoint.
/// * `handler` - The updates handler for processing incoming updates.
pub async fn run_server<A, P, H>(address: A, path: P, handler: H) -> Result<(), HyperError>
where
    A: Into<SocketAddr>,
    P: Into<String>,
    H: UpdateHandler + Send + Sync + 'static,
    H::Future: Send,
{
    let address = address.into();
    let path = path.into();
    let server = Server::bind(&address).serve(WebhookServiceFactory::new(path, handler));
    server.await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_send<T: Send>() {}

    fn assert_sync<T: Sync>() {}

    #[test]
    fn synced_is_sync() {
        // pointer is neither Send nor Sync
        struct NonSync(*mut ());

        unsafe impl Send for NonSync {}

        impl UpdateHandler for NonSync {
            type Future = BoxFuture<'static, ()>;

            fn handle(&self, _update: Update) -> Self::Future {
                Box::pin(async {})
            }
        }

        assert_send::<NonSync>();
        assert_send::<SyncedUpdateHandler<NonSync>>();
        assert_sync::<SyncedUpdateHandler<NonSync>>();
    }
}
