use crate::types::login_url::LoginUrl;
use serde::{Deserialize, Serialize};
use serde_json::{Error as JsonError, Value as JsonValue};
use std::{convert::TryFrom, error::Error as StdError, fmt};

/// Inline keyboard that appears right next to the message it belongs to
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Serialize, Deserialize)]
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

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
struct InlineKeyboardButtonKindRaw {
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
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

impl StdError for InlineKeyboardError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ReplyMarkup;

    #[derive(Serialize)]
    struct CallbackData {
        value: String,
    }

    #[test]
    fn serialize() {
        let callback_data = CallbackData {
            value: String::from("cdstruct"),
        };

        let markup: ReplyMarkup = vec![vec![
            InlineKeyboardButton::with_url("url", "tg://user?id=1"),
            InlineKeyboardButton::with_callback_data("cd", "cd"),
            InlineKeyboardButton::with_callback_data_struct("cd", &callback_data).unwrap(),
            InlineKeyboardButton::with_switch_inline_query("siq", "siq"),
            InlineKeyboardButton::with_switch_inline_query_current_chat("siqcc", "siqcc"),
            InlineKeyboardButton::with_callback_game("cg"),
            InlineKeyboardButton::with_pay("pay"),
            InlineKeyboardButton::with_login_url("login url", "http://example.com"),
        ]]
        .into();
        let data = serde_json::to_value(&markup).unwrap();
        assert_eq!(
            data,
            serde_json::json!({
                "inline_keyboard": [
                    [
                        {"text":"url","url":"tg://user?id=1"},
                        {"text":"cd","callback_data":"cd"},
                        {"text":"cd","callback_data":"{\"value\":\"cdstruct\"}"},
                        {"text":"siq","switch_inline_query":"siq"},
                        {"text":"siqcc","switch_inline_query_current_chat":"siqcc"},
                        {"text":"cg","callback_game":{}},
                        {"text":"pay","pay":true},
                        {"text":"login url","login_url":{"url":"http://example.com"}}
                    ]
                ]
            })
        );
    }

    #[test]
    fn deserialize() {
        let buttons: Vec<InlineKeyboardButton> = serde_json::from_value(serde_json::json!(
            [
                {"text":"url","url":"tg://user?id=1"},
                {"text":"cd","callback_data":"cd"},
                {"text":"cd","callback_data":"{\"value\":\"cdstruct\"}"},
                {"text":"siq","switch_inline_query":"siq"},
                {"text":"siqcc","switch_inline_query_current_chat":"siqcc"},
                {"text":"cg","callback_game":{}},
                {"text":"pay","pay":true},
                {"text":"login url","login_url":{"url":"http://example.com"}}
            ]
        ))
        .unwrap();
        assert_eq!(buttons.len(), 8);
        assert_eq!(buttons[0].text, "url");
        assert_eq!(buttons[1].text, "cd");
        assert_eq!(buttons[2].text, "cd");
        assert_eq!(buttons[3].text, "siq");
        assert_eq!(buttons[4].text, "siqcc");
        assert_eq!(buttons[5].text, "cg");
        assert_eq!(buttons[6].text, "pay");
        assert_eq!(buttons[7].text, "login url");
    }

    #[test]
    fn convert() {
        let a = vec![vec![InlineKeyboardButton::with_url("url", "tg://user?id=1")]];
        let b: Vec<Vec<InlineKeyboardButton>> = InlineKeyboardMarkup::from(a.clone()).into();
        assert_eq!(a.len(), b.len())
    }
}
