use serde::{Deserialize, Serialize};

/// Represents a [Web App](https://core.telegram.org/bots/webapps).
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct WebAppInfo {
    /// An HTTPS URL of a Web App to be opened
    /// with additional data as specified in
    /// [Initializing Web Apps](https://core.telegram.org/bots/webapps#initializing-mini-apps).
    pub url: String,
}

impl<T> From<T> for WebAppInfo
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self { url: value.into() }
    }
}

/// Represents a data sent from a Web App to the bot.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct WebAppData {
    /// The data.
    ///
    /// Be aware that a bad client can send arbitrary data in this field.
    pub data: String,
    /// Text of the web_app keyboard button from which the Web App was opened.
    ///
    /// Be aware that a bad client can send arbitrary data in this field.
    pub button_text: String,
}
