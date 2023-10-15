use std::{convert::TryFrom, error::Error, fmt};

use serde::{Deserialize, Serialize};
use serde_json::{Error as JsonError, Value as JsonValue};

use crate::types::WebAppInfo;

#[cfg(test)]
mod tests;

/// Inline keyboard that appears right next to the message it belongs to
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InlineKeyboardMarkup {
    inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardMarkup {
    /// Returns a KeyboardMarkup with given keyboard
    pub fn from_vec(inline_keyboard: Vec<Vec<InlineKeyboardButton>>) -> Self {
        InlineKeyboardMarkup { inline_keyboard }
    }

    /// Adds a row to keyboard
    pub fn row(mut self, row: Vec<InlineKeyboardButton>) -> Self {
        self.inline_keyboard.push(row);
        self
    }

    pub(crate) fn serialize(&self) -> Result<String, InlineKeyboardError> {
        serde_json::to_string(self).map_err(InlineKeyboardError::SerializeMarkup)
    }

    /// Converts markup into a vector of buttons
    pub fn into_vec(self) -> Vec<Vec<InlineKeyboardButton>> {
        self.inline_keyboard
    }
}

impl From<Vec<Vec<InlineKeyboardButton>>> for InlineKeyboardMarkup {
    fn from(keyboard: Vec<Vec<InlineKeyboardButton>>) -> InlineKeyboardMarkup {
        InlineKeyboardMarkup::from_vec(keyboard)
    }
}

impl From<InlineKeyboardMarkup> for Vec<Vec<InlineKeyboardButton>> {
    fn from(keyboard: InlineKeyboardMarkup) -> Self {
        keyboard.into_vec()
    }
}

/// Button of an inline keyboard
///
/// You must use exactly one of the optional fields
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineKeyboardButton {
    text: String,
    #[serde(flatten)]
    kind: InlineKeyboardButtonKindRaw,
}

impl InlineKeyboardButton {
    /// Creates a new InlineKeyboardButton
    ///
    /// # Arguments
    ///
    /// * text - Text of the button
    /// * kind - Data for the button
    pub fn new<S: Into<String>>(text: S, kind: InlineKeyboardButtonKind) -> Self {
        Self {
            text: text.into(),
            kind: InlineKeyboardButtonKindRaw::from(kind),
        }
    }

    /// HTTP or tg:// url to be opened when button is pressed
    pub fn with_url<T: Into<String>, D: Into<String>>(text: T, url: D) -> Self {
        Self::new(text, InlineKeyboardButtonKind::Url(url.into()))
    }

    /// Description of the Web App that will be launched when the user presses the button
    pub fn with_web_app<T: Into<String>>(text: T, web_app_info: WebAppInfo) -> Self {
        Self::new(text, InlineKeyboardButtonKind::WebApp(web_app_info))
    }

    /// Data to be sent in a callback query to the bot when button is pressed, 1-64 bytes
    pub fn with_callback_data<T: Into<String>, D: Into<String>>(text: T, data: D) -> Self {
        Self::new(text, InlineKeyboardButtonKind::CallbackData(data.into()))
    }

    /// Same as with_callback_data, but takes a serializable type
    ///
    /// Data will be serialized using serde_json
    pub fn with_callback_data_struct<T: Into<String>, D: Serialize>(
        text: T,
        data: &D,
    ) -> Result<Self, InlineKeyboardError> {
        let data = serde_json::to_string(data).map_err(InlineKeyboardError::SerializeCallbackData)?;
        Ok(Self::new(text, InlineKeyboardButtonKind::CallbackData(data)))
    }

    /// Pressing the button will prompt the user to select one of their chats,
    /// open that chat and insert the bot‘s username and
    /// the specified inline query in the input field
    ///
    /// Can be empty, in which case just the bot’s username will be inserted
    ///
    /// Note: This offers an easy way for users to start using your bot
    /// in inline mode when they are currently in a private chat with it
    ///
    /// Especially useful when combined with switch_pm… actions – in this case the user
    /// will be automatically returned to the chat they switched from,
    /// skipping the chat selection screen
    pub fn with_switch_inline_query<T: Into<String>, D: Into<String>>(text: T, data: D) -> Self {
        Self::new(text, InlineKeyboardButtonKind::SwitchInlineQuery(data.into()))
    }

    /// If set, pressing the button will insert the bot‘s username and
    /// the specified inline query in the current chat's input field
    ///
    /// Can be empty, in which case only the bot’s username will be inserted
    /// This offers a quick way for the user to open your bot in
    /// inline mode in the same chat – good for selecting something from multiple options
    pub fn with_switch_inline_query_current_chat<T: Into<String>, D: Into<String>>(text: T, data: D) -> Self {
        Self::new(
            text,
            InlineKeyboardButtonKind::SwitchInlineQueryCurrentChat(data.into()),
        )
    }

    /// Description of the game that will be launched when the user presses the button
    ///
    /// NOTE: This type of button must always be the first button in the first row
    pub fn with_callback_game<T: Into<String>>(text: T) -> Self {
        Self::new(text, InlineKeyboardButtonKind::CallbackGame)
    }

    /// Send a Pay button
    ///
    /// NOTE: This type of button must always be the first button in the first row
    pub fn with_pay<T: Into<String>>(text: T) -> Self {
        Self::new(text, InlineKeyboardButtonKind::Pay)
    }

    /// An HTTP URL used to automatically authorize the user
    ///
    /// Can be used as a replacement for the [Telegram Login Widget][1]
    ///
    /// [1]: https://core.telegram.org/widgets/login
    pub fn with_login_url<T: Into<String>, D: Into<LoginUrl>>(text: T, data: D) -> Self {
        Self::new(text, InlineKeyboardButtonKind::LoginUrl(data.into()))
    }

    /// Returns a text of the button
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns a data for the button
    pub fn kind(&self) -> Result<InlineKeyboardButtonKind, InlineKeyboardError> {
        InlineKeyboardButtonKind::try_from(self.kind.clone())
    }
}

/// Variant of inline keyboard button
#[derive(Clone, Debug)]
pub enum InlineKeyboardButtonKind {
    /// HTTP or tg:// url to be opened when button is pressed
    Url(String),
    /// Description of the Web App that will be launched when the user presses the button
    ///
    /// The Web App will be able to send an arbitrary message on behalf
    /// of the user using the method answerWebAppQuery.
    /// Available only in private chats between a user and the bot.
    WebApp(WebAppInfo),
    /// Data to be sent in a callback query to the bot when button is pressed, 1-64 bytes
    CallbackData(String),
    /// Pressing the button will prompt the user to select one of their chats,
    /// open that chat and insert the bot‘s username and
    /// the specified inline query in the input field
    ///
    /// Can be empty, in which case just the bot’s username will be inserted
    ///
    /// Note: This offers an easy way for users to start using your bot
    /// in inline mode when they are currently in a private chat with it
    ///
    /// Especially useful when combined with switch_pm… actions – in this case the user
    /// will be automatically returned to the chat they switched from,
    /// skipping the chat selection screen
    SwitchInlineQuery(String),
    /// If set, pressing the button will insert the bot‘s username and
    /// the specified inline query in the current chat's input field
    ///
    /// Can be empty, in which case only the bot’s username will be inserted
    /// This offers a quick way for the user to open your bot in
    /// inline mode in the same chat – good for selecting something from multiple options
    SwitchInlineQueryCurrentChat(String),
    /// Description of the game that will be launched when the user presses the button
    ///
    /// NOTE: This type of button must always be the first button in the first row
    CallbackGame,
    /// Send a Pay button
    ///
    /// NOTE: This type of button must always be the first button in the first row
    Pay,
    /// An HTTP URL used to automatically authorize the user
    ///
    /// Can be used as a replacement for the [Telegram Login Widget][1]
    ///
    /// [1]: https://core.telegram.org/widgets/login
    LoginUrl(LoginUrl),
}

#[derive(Default, Clone, Debug, Deserialize, PartialEq, Serialize)]
struct InlineKeyboardButtonKindRaw {
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    web_app: Option<WebAppInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    callback_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_inline_query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_inline_query_current_chat: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    callback_game: Option<JsonValue>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    login_url: Option<LoginUrl>,
}

impl From<InlineKeyboardButtonKind> for InlineKeyboardButtonKindRaw {
    fn from(kind: InlineKeyboardButtonKind) -> Self {
        use self::InlineKeyboardButtonKind::*;
        let mut raw = Self::default();
        match kind {
            Url(data) => raw.url = Some(data),
            WebApp(data) => raw.web_app = Some(data),
            CallbackData(data) => raw.callback_data = Some(data),
            SwitchInlineQuery(data) => raw.switch_inline_query = Some(data),
            SwitchInlineQueryCurrentChat(data) => raw.switch_inline_query_current_chat = Some(data),
            CallbackGame => raw.callback_game = Some(serde_json::json!({})),
            Pay => raw.pay = Some(true),
            LoginUrl(data) => raw.login_url = Some(data),
        }
        raw
    }
}

impl TryFrom<InlineKeyboardButtonKindRaw> for InlineKeyboardButtonKind {
    type Error = InlineKeyboardError;

    fn try_from(kind: InlineKeyboardButtonKindRaw) -> Result<Self, Self::Error> {
        Ok(if let Some(data) = kind.url {
            Self::Url(data)
        } else if let Some(data) = kind.web_app {
            Self::WebApp(data)
        } else if let Some(data) = kind.callback_data {
            Self::CallbackData(data)
        } else if let Some(data) = kind.switch_inline_query {
            Self::SwitchInlineQuery(data)
        } else if let Some(data) = kind.switch_inline_query_current_chat {
            Self::SwitchInlineQueryCurrentChat(data)
        } else if kind.callback_game.is_some() {
            Self::CallbackGame
        } else if kind.pay.is_some() {
            Self::Pay
        } else if let Some(data) = kind.login_url {
            Self::LoginUrl(data)
        } else {
            return Err(InlineKeyboardError::UnexpectedButtonKind);
        })
    }
}

/// An error occurred with inline keyboard
#[derive(Debug)]
pub enum InlineKeyboardError {
    /// Can not serialize callback data
    SerializeCallbackData(JsonError),
    /// Can not serialize markup
    SerializeMarkup(JsonError),
    /// Can not detect button kind when deserializing keyboard
    UnexpectedButtonKind,
}

impl Error for InlineKeyboardError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::InlineKeyboardError::*;
        match self {
            SerializeCallbackData(err) => Some(err),
            SerializeMarkup(err) => Some(err),
            UnexpectedButtonKind => None,
        }
    }
}

impl fmt::Display for InlineKeyboardError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::InlineKeyboardError::*;
        match self {
            SerializeCallbackData(err) => write!(out, "failed to serialize callback data: {}", err),
            SerializeMarkup(err) => write!(out, "failed to serialize markup: {}", err),
            UnexpectedButtonKind => write!(out, "can not detect button kind"),
        }
    }
}

/// Represents a parameter of the inline keyboard button used to automatically authorize a user
///
/// Serves as a great replacement for the Telegram Login Widget when the user is coming from Telegram
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct LoginUrl {
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    forward_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bot_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    request_write_access: Option<bool>,
}

impl LoginUrl {
    /// Creates a new LoginUrl with given URL
    ///
    /// An HTTP URL will be opened with user authorization data added to the query string when the button is pressed
    ///
    /// If the user refuses to provide authorization data, the original URL without information about the user will be opened
    ///
    /// The data added is the same as described in [Receiving authorization data][1]
    ///
    /// NOTE: You **must** always check the hash of the received data to verify the authentication
    /// and the integrity of the data as described in [Checking authorization][2]
    ///
    /// [1]: https://core.telegram.org/widgets/login#receiving-authorization-data
    /// [2]: https://core.telegram.org/widgets/login#checking-authorization
    pub fn new<S>(url: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            url: url.into(),
            forward_text: None,
            bot_username: None,
            request_write_access: None,
        }
    }

    /// New text of the button in forwarded messages
    pub fn forward_text<S>(mut self, forward_text: S) -> Self
    where
        S: Into<String>,
    {
        self.forward_text = Some(forward_text.into());
        self
    }

    /// Username of a bot, which will be used for user authorization
    ///
    /// See [Setting up a bot][1] for more details
    ///
    /// If not specified, the current bot username will be assumed
    ///
    /// The url domain must be the same as the domain linked with the bot
    ///
    /// See [Linking your domain to the bot][2] for more details
    ///
    /// [1]: https://core.telegram.org/widgets/login#setting-up-a-bot
    /// [2]: https://core.telegram.org/widgets/login#linking-your-domain-to-the-bot
    pub fn bot_username<S>(mut self, bot_username: S) -> Self
    where
        S: Into<String>,
    {
        self.bot_username = Some(bot_username.into());
        self
    }

    /// Pass True to request the permission for your bot to send messages to the user
    pub fn request_write_access(mut self, request_write_access: bool) -> Self {
        self.request_write_access = Some(request_write_access);
        self
    }
}

impl<S> From<S> for LoginUrl
where
    S: Into<String>,
{
    fn from(url: S) -> Self {
        Self::new(url)
    }
}
