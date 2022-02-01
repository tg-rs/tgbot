use crate::{
    config::Config,
    methods::Method,
    request::{FormError, Request, RequestBody, RequestMethod},
    types::{Response, ResponseError},
};
use bytes::Bytes;
use futures_util::stream::Stream;
use log::debug;
use reqwest::{
    Client as HttpClient, ClientBuilder as HttpClientBuilder, Error as HttpError, RequestBuilder as HttpRequestBuilder,
};
use serde::de::DeserializeOwned;
use serde_json::Error as JsonError;
use std::{error::Error as StdError, fmt, time::Duration};
use tokio::time::sleep;

/// Telegram Bot API client
#[derive(Clone)]
pub struct Api {
    client: HttpClient,
    host: String,
    token: String,
}

impl Api {
    /// Creates a new API instance with given configuration
    pub fn new<C: Into<Config>>(config: C) -> Result<Self, ApiError> {
        let config = config.into();

        let mut builder = HttpClientBuilder::new();
        builder = if let Some(proxy) = config.get_proxy() {
            builder.proxy(proxy.clone())
        } else {
            builder.no_proxy()
        };
        let client = builder.use_rustls_tls().build().map_err(ApiError::BuildClient)?;

        Ok(Api {
            client,
            host: config.get_host().to_string(),
            token: config.get_token().to_string(),
        })
    }

    /// Downloads a file
    ///
    /// Use `getFile` method in order to get a value for `file_path` argument
    ///
    /// # Example
    ///
    /// ```
    /// # async fn download_file() {
    /// use tgbot::Api;
    /// use futures_util::stream::StreamExt;
    /// let api = Api::new("token").unwrap();
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
        let req = Request::empty(file_path.as_ref());
        let url = req.build_url(&format!("{}/file", &self.host), &self.token);
        debug!("Downloading file from {}", url);
        let rep = self.client.get(&url).send().await?;
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
        let request = method.into_request();
        let builder = self.get_http_request_builder(request).await?;
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

    async fn get_http_request_builder(&self, request: Request) -> Result<HttpRequestBuilder, ExecuteError> {
        let url = request.build_url(&self.host, &self.token);
        let builder = match request.get_method() {
            RequestMethod::Get => {
                debug!("Execute GET {}", url);
                self.client.get(&url)
            }
            RequestMethod::Post => {
                debug!("Execute POST: {}", url);
                self.client.post(&url)
            }
        };
        Ok(match request.into_body() {
            RequestBody::Form(form) => {
                let form = form.into_multipart().await?;
                debug!("Sending multipart body: {:?}", form);
                builder.multipart(form)
            }
            RequestBody::Json(data) => {
                let data = data?;
                debug!("Sending JSON body: {:?}", data);
                builder.header("Content-Type", "application/json").body(data)
            }
            RequestBody::Empty => {
                debug!("Sending empty body");
                builder
            }
        })
    }

    async fn send_request<T>(&self, http_request: HttpRequestBuilder) -> Result<Response<T>, ExecuteError>
    where
        T: DeserializeOwned,
    {
        let response = http_request.send().await?;
        Ok(response.json::<Response<T>>().await?)
    }
}

impl fmt::Debug for Api {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Api")
            .field("client", &self.client)
            .field("host", &self.host)
            .field("token", &format_args!("..."))
            .finish()
    }
}

/// A general API error
#[derive(Debug)]
pub enum ApiError {
    /// Can not build HTTP client
    BuildClient(HttpError),
}

impl StdError for ApiError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(match self {
            ApiError::BuildClient(err) => err,
        })
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApiError::BuildClient(err) => write!(out, "can not build HTTP client: {}", err),
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

impl StdError for DownloadFileError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
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
    /// Can not build multipart form
    Form(FormError),
    /// Can not serialize JSON
    Json(JsonError),
    /// Telegram error got in response
    Response(ResponseError),
    /// Too many requests
    TooManyRequests,
}

impl StdError for ExecuteError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        use self::ExecuteError::*;
        Some(match self {
            Http(err) => err,
            Form(err) => err,
            Json(err) => err,
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
                Form(err) => err.to_string(),
                Json(err) => err.to_string(),
                Response(err) => err.to_string(),
                TooManyRequests => "too many requests".to_string(),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DEFAULT_HOST;

    #[test]
    fn api() {
        let config = Config::new("token")
            .host("https://example.com")
            .proxy("http://user:password@127.0.0.1:1234")
            .unwrap();
        let api = Api::new(config).unwrap();
        assert_eq!(api.host, "https://example.com");
        assert_eq!(api.token, "token");

        let config = Config::new("token");
        let api = Api::new(config).unwrap();
        assert_eq!(api.host, DEFAULT_HOST);
        assert_eq!(api.token, "token");
    }
}
