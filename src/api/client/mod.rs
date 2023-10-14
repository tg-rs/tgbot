use std::{error::Error, fmt, time::Duration};

use bytes::Bytes;
use futures_util::stream::Stream;
use log::debug;
use reqwest::{
    Client as HttpClient,
    ClientBuilder as HttpClientBuilder,
    Error as HttpError,
    RequestBuilder as HttpRequestBuilder,
};
use serde::de::DeserializeOwned;
use tokio::time::sleep;

use crate::types::{Response, ResponseError};

use super::payload::{Payload, PayloadError};

#[cfg(test)]
mod tests;

const DEFAULT_HOST: &str = "https://api.telegram.org";

/// Telegram Bot API client
#[derive(Clone)]
pub struct Client {
    host: String,
    http_client: HttpClient,
    token: String,
}

impl Client {
    /// Creates a new API instance with given token
    ///
    /// # Arguments
    ///
    /// * token - A bot token
    pub fn new<T>(token: T) -> Result<Self, ClientError>
    where
        T: Into<String>,
    {
        let client = HttpClientBuilder::new()
            .use_rustls_tls()
            .build()
            .map_err(ClientError::BuildClient)?;
        Ok(Self::with_client(client, token))
    }

    /// Creates a new API instance with given HTTP client and token
    ///
    /// # Arguments
    ///
    /// * client - An HTTP client
    /// * token - A bot token
    ///
    pub fn with_client<T>(http_client: HttpClient, token: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            http_client,
            host: String::from(DEFAULT_HOST),
            token: token.into(),
        }
    }

    /// Overrides API host
    ///
    /// # Arguments
    ///
    /// * host - A new API host
    pub fn with_host<T>(mut self, host: T) -> Self
    where
        T: Into<String>,
    {
        self.host = host.into();
        self
    }

    /// Downloads a file
    ///
    /// Use `getFile` method in order to get a value for `file_path` argument
    ///
    /// # Example
    ///
    /// ```
    /// # async fn download_file() {
    /// use tgbot::api::Client;
    /// use futures_util::stream::StreamExt;
    /// let api = Client::new("token").unwrap();
    /// let mut stream = api.download_file("path").await.unwrap();
    /// while let Some(chunk) = stream.next().await {
    ///     let chunk = chunk.unwrap();
    ///     // write chunk to something...
    /// }
    /// # }
    /// ```
    pub async fn download_file<P>(
        &self,
        file_path: P,
    ) -> Result<impl Stream<Item = Result<Bytes, HttpError>>, DownloadFileError>
    where
        P: AsRef<str>,
    {
        let payload = Payload::empty(file_path.as_ref());
        let url = payload.build_url(&format!("{}/file", &self.host), &self.token);
        debug!("Downloading file from {}", url);
        let rep = self.http_client.get(&url).send().await?;
        let status = rep.status();
        if !status.is_success() {
            Err(DownloadFileError::Response {
                status: status.as_u16(),
                text: rep.text().await?,
            })
        } else {
            Ok(rep.bytes_stream())
        }
    }

    /// Executes a method
    ///
    /// Note that client will not retry a request on timeout error
    /// if the request is not cloneable
    /// (e.g. contains a stream).
    pub async fn execute<M>(&self, method: M) -> Result<M::Response, ExecuteError>
    where
        M: Method,
        M::Response: DeserializeOwned + Send + 'static,
    {
        let builder = method
            .into_payload()
            .into_http_request_builder(&self.http_client, &self.host, &self.token)?;
        for i in 0..2 {
            if i != 0 {
                debug!("Retrying request after timeout error");
            }
            match builder.try_clone() {
                Some(builder) => {
                    let response = self.send_request(builder).await?;
                    match response.retry_after() {
                        Some(retry_after) => {
                            debug!("Got a timeout error (retry_after={retry_after})");
                            sleep(Duration::from_secs(retry_after)).await
                        }
                        None => return Ok(response.into_result()?),
                    }
                }
                None => {
                    debug!("Could not clone builder, sending request without retry");
                    return Ok(self.send_request(builder).await?.into_result()?);
                }
            }
        }
        Err(ExecuteError::TooManyRequests)
    }

    async fn send_request<T>(&self, http_request: HttpRequestBuilder) -> Result<Response<T>, ExecuteError>
    where
        T: DeserializeOwned,
    {
        let response = http_request.send().await?;
        Ok(response.json::<Response<T>>().await?)
    }
}

impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Client")
            .field("http_client", &self.http_client)
            .field("host", &self.host)
            .field("token", &format_args!("..."))
            .finish()
    }
}

/// Represents an API method
pub trait Method {
    /// Type of successful result in API response
    type Response;

    /// Returns information about HTTP request
    fn into_payload(self) -> Payload;
}

/// A general Client error
#[derive(Debug)]
pub enum ClientError {
    /// Can not build HTTP client
    BuildClient(HttpError),
}

impl Error for ClientError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            ClientError::BuildClient(err) => err,
        })
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ClientError::BuildClient(err) => write!(out, "can not build HTTP client: {}", err),
        }
    }
}

/// An error when downloading file
#[derive(Debug)]
pub enum DownloadFileError {
    /// Error when sending request
    Http(HttpError),
    /// Server replied with an error
    Response {
        /// HTTP status code
        status: u16,
        /// Response body
        text: String,
    },
}

impl From<HttpError> for DownloadFileError {
    fn from(err: HttpError) -> Self {
        Self::Http(err)
    }
}

impl Error for DownloadFileError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            DownloadFileError::Http(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for DownloadFileError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DownloadFileError::Http(err) => write!(out, "failed to download file: {}", err),
            DownloadFileError::Response { status, text } => {
                write!(out, "failed to download file: status={} text={}", status, text)
            }
        }
    }
}

/// An error when executing method
#[derive(Debug, derive_more::From)]
pub enum ExecuteError {
    /// Error when sending request
    Http(HttpError),
    /// Can not build an HTTP request
    Payload(PayloadError),
    /// Telegram error got in response
    Response(ResponseError),
    /// Too many requests
    TooManyRequests,
}

impl Error for ExecuteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::ExecuteError::*;
        Some(match self {
            Http(err) => err,
            Payload(err) => err,
            Response(err) => err,
            TooManyRequests => return None,
        })
    }
}

impl fmt::Display for ExecuteError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::ExecuteError::*;
        write!(
            out,
            "failed to execute method: {}",
            match self {
                Http(err) => err.to_string(),
                Payload(err) => err.to_string(),
                Response(err) => err.to_string(),
                TooManyRequests => "too many requests".to_string(),
            }
        )
    }
}
