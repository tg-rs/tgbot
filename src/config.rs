use reqwest::{Error as ReqwestError, Proxy};
use std::{error::Error, fmt};
use url::{ParseError as UrlParseError, Url};

pub(super) const DEFAULT_HOST: &str = "https://api.telegram.org";

/// An API config
#[derive(Debug, Clone)]
pub struct Config {
    host: String,
    token: String,
    proxy: Option<Proxy>,
}

impl Config {
    /// Creates a new configuration with a given token
    pub fn new<S: Into<String>>(token: S) -> Self {
        Self {
            token: token.into(),
            host: String::from(DEFAULT_HOST),
            proxy: None,
        }
    }

    /// Returns the token
    pub fn get_token(&self) -> &str {
        &self.token
    }

    /// Sets an API host
    ///
    /// `https://api.telegram.org` is used by default
    pub fn host<S: Into<String>>(mut self, host: S) -> Self {
        self.host = host.into();
        self
    }

    /// Returns the API host
    pub fn get_host(&self) -> &str {
        &self.host
    }

    /// Sets a proxy to config
    ///
    /// Proxy format:
    /// * http://\[user:password\]@host:port
    /// * https://\[user:password\]@host:port
    /// * socks5://\[user:password\]@host:port
    pub fn proxy<U: AsRef<str>>(mut self, url: U) -> Result<Self, ParseProxyError> {
        let raw_url = url.as_ref();
        let url = Url::parse(raw_url)?;
        let username = url.username();
        if !username.is_empty() {
            let mut base_url = format!("{}://{}", url.scheme(), url.host_str().expect("Can not get host"));
            if let Some(port) = url.port() {
                base_url = format!("{}:{}", base_url, port);
            }
            self.proxy = Some(Proxy::all(base_url.as_str())?.basic_auth(username, url.password().unwrap_or("")));
        } else {
            self.proxy = Some(Proxy::all(raw_url)?);
        }
        Ok(self)
    }

    /// Returns the proxy
    pub fn get_proxy(&self) -> Option<&Proxy> {
        self.proxy.as_ref()
    }
}

impl<S> From<S> for Config
where
    S: Into<String>,
{
    fn from(token: S) -> Self {
        Config::new(token.into())
    }
}

/// An error when parsing proxy
#[derive(Debug, derive_more::From)]
pub enum ParseProxyError {
    /// Can not parse given URL
    UrlParse(UrlParseError),
    /// Can not create reqwest Proxy
    Reqwest(ReqwestError),
}

impl Error for ParseProxyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            ParseProxyError::UrlParse(err) => err,
            ParseProxyError::Reqwest(err) => err,
        })
    }
}

impl fmt::Display for ParseProxyError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(
            out,
            "can not parse proxy URL: {}",
            match self {
                ParseProxyError::UrlParse(err) => err.to_string(),
                ParseProxyError::Reqwest(err) => err.to_string(),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config() {
        let config = Config::new("token")
            .proxy("socks5://user:password@127.0.0.1:1234")
            .unwrap();
        assert_eq!(config.token, "token");
        assert_eq!(config.host, DEFAULT_HOST);
        assert!(config.proxy.is_some());

        let config = Config::new("token")
            .host("https://example.com")
            .proxy("http://127.0.0.1:1234")
            .unwrap();
        assert_eq!(config.token, "token");
        assert_eq!(config.host, "https://example.com");
        assert!(config.proxy.is_some());

        let config = Config::new("token").proxy("https://127.0.0.1:1234").unwrap();
        assert_eq!(config.token, "token");
        assert_eq!(config.host, DEFAULT_HOST);
        assert!(config.proxy.is_some());

        let config = Config::new("token");
        assert_eq!(config.token, "token");
        assert_eq!(config.host, DEFAULT_HOST);
        assert!(config.proxy.is_none());
    }
}
