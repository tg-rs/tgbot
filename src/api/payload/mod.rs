use std::{error::Error, fmt};

use log::debug;
use reqwest::{Client as HttpClient, Method as HttpMethod, RequestBuilder as HttpRequestBuilder};
use serde::ser::Serialize;
use serde_json::Error as JsonError;

use super::form::{Form, FormError};

#[cfg(test)]
pub(crate) use self::tests::*;

#[cfg(test)]
mod tests;

#[derive(Debug)]
enum PayloadData {
    Form(Form),
    Json(Result<String, JsonError>),
    Empty,
}

#[doc(hidden)]
#[derive(Debug)]
pub struct Payload {
    http_method: HttpMethod,
    payload_data: PayloadData,
    url_path: String,
}

impl Payload {
    pub(crate) fn form<P: Into<String>>(path: P, form: Form) -> Self {
        Self {
            http_method: HttpMethod::POST,
            payload_data: PayloadData::Form(form),
            url_path: path.into(),
        }
    }

    pub(crate) fn json<P: Into<String>>(path: P, data: impl Serialize) -> Self {
        Self {
            http_method: HttpMethod::POST,
            payload_data: PayloadData::Json(serde_json::to_string(&data)),
            url_path: path.into(),
        }
    }

    pub(crate) fn empty<P: Into<String>>(path: P) -> Self {
        Self {
            http_method: HttpMethod::GET,
            payload_data: PayloadData::Empty,
            url_path: path.into(),
        }
    }

    pub(super) fn build_url(&self, base_url: &str, token: &str) -> String {
        format!("{}/bot{}/{}", base_url, token, self.url_path)
    }

    pub(super) fn into_http_request_builder(
        self,
        http_client: &HttpClient,
        base_url: &str,
        token: &str,
    ) -> Result<HttpRequestBuilder, PayloadError> {
        let url = self.build_url(base_url, token);
        let builder = http_client.request(self.http_method, url);
        Ok(match self.payload_data {
            PayloadData::Form(form) => {
                let form = form.into_multipart()?;
                debug!("Sending multipart body: {:?}", form);
                builder.multipart(form)
            }
            PayloadData::Json(data) => {
                let data = data?;
                debug!("Sending JSON body: {:?}", data);
                builder.header("Content-Type", "application/json").body(data)
            }
            PayloadData::Empty => {
                debug!("Sending empty body");
                builder
            }
        })
    }
}

/// An error when building an HTTP request
#[derive(Debug, derive_more::From)]
pub enum PayloadError {
    /// Can not build a form body
    Form(FormError),
    /// Can not build a JSON body
    Json(JsonError),
}

impl Error for PayloadError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::PayloadError::*;
        Some(match self {
            Form(err) => err,
            Json(err) => err,
        })
    }
}

impl fmt::Display for PayloadError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::PayloadError::*;
        write!(
            out,
            "could not build an HTTP request: {}",
            match self {
                Form(err) => err.to_string(),
                Json(err) => err.to_string(),
            }
        )
    }
}
