use std::{error::Error, fmt};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Error as JsonError;

use crate::types::{True, WebAppInfo};

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
    button_type: InlineKeyboardButtonType,
}

impl InlineKeyboardButton {
    /// Creates a new InlineKeyboardButton
    ///
    /// # Arguments
    ///
    /// * text - Text of the button
    /// * kind - Data for the button
    pub fn new<S: Into<String>>(text: S, button_type: InlineKeyboardButtonType) -> Self {
        Self {
            text: text.into(),
            button_type,
        }
    }

    /// HTTP or tg:// url to be opened when button is pressed
    pub fn with_url<T: Into<String>, D: Into<String>>(text: T, url: D) -> Self {
        Self::new(text, InlineKeyboardButtonType::Url(url.into()))
    }

    /// Description of the Web App that will be launched when the user presses the button
    pub fn with_web_app<T: Into<String>>(text: T, web_app_info: WebAppInfo) -> Self {
        Self::new(text, InlineKeyboardButtonType::WebApp(web_app_info))
    }

    /// Data to be sent in a callback query to the bot when button is pressed, 1-64 bytes
    pub fn with_callback_data<T: Into<String>, D: Into<String>>(text: T, data: D) -> Self {
        Self::new(text, InlineKeyboardButtonType::CallbackData(data.into()))
    }

    /// Same as with_callback_data, but takes a serializable type
    ///
    /// Data will be serialized using serde_json
    pub fn with_callback_data_struct<T: Into<String>, D: Serialize>(
        text: T,
        data: &D,
    ) -> Result<Self, InlineKeyboardError> {
        let data = serde_json::to_string(data).map_err(InlineKeyboardError::SerializeCallbackData)?;
        Ok(Self::new(text, InlineKeyboardButtonType::CallbackData(data)))
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
        Self::new(text, InlineKeyboardButtonType::SwitchInlineQuery(data.into()))
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
            InlineKeyboardButtonType::SwitchInlineQueryCurrentChat(data.into()),
        )
    }

    /// Pressing the button will prompt the user to select one of their chats of the specified type,
    /// open that chat and insert the bot username and the specified inline query in the input field
    pub fn with_switch_inline_query_chosen_chat<T>(text: T, value: SwitchInlineQueryChosenChat) -> Self
    where
        T: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonType::SwitchInlineQueryChosenChat(value))
    }

    /// Description of the game that will be launched when the user presses the button
    ///
    /// NOTE: This type of button must always be the first button in the first row
    pub fn with_callback_game<T: Into<String>>(text: T) -> Self {
        Self::new(text, InlineKeyboardButtonType::CallbackGame)
    }

    /// Send a Pay button
    ///
    /// NOTE: This type of button must always be the first button in the first row
    pub fn with_pay<T: Into<String>>(text: T) -> Self {
        Self::new(text, InlineKeyboardButtonType::Pay)
    }

    /// An HTTP URL used to automatically authorize the user
    ///
    /// Can be used as a replacement for the [Telegram Login Widget][1]
    ///
    /// [1]: https://core.telegram.org/widgets/login
    pub fn with_login_url<T: Into<String>, D: Into<LoginUrl>>(text: T, data: D) -> Self {
        Self::new(text, InlineKeyboardButtonType::LoginUrl(data.into()))
    }

    /// Returns a text of the button
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns a data for the button
    pub fn button_type(&self) -> &InlineKeyboardButtonType {
        &self.button_type
    }
}

/// Variant of inline keyboard button
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InlineKeyboardButtonType {
    /// Description of the game that will be launched when the user presses the button
    ///
    /// NOTE: This type of button must always be the first button in the first row
    #[serde(
        deserialize_with = "RawButtonEmpty::deserialize_value",
        serialize_with = "RawButtonEmpty::serialize_value"
    )]
    CallbackGame,
    /// Data to be sent in a callback query to the bot when button is pressed, 1-64 bytes
    CallbackData(String),
    /// An HTTP URL used to automatically authorize the user
    ///
    /// Can be used as a replacement for the [Telegram Login Widget][1]
    ///
    /// [1]: https://core.telegram.org/widgets/login
    LoginUrl(LoginUrl),
    /// Send a Pay button
    ///
    /// NOTE: This type of button must always be the first button in the first row
    #[serde(
        deserialize_with = "RawButtonFlag::deserialize_value",
        serialize_with = "RawButtonFlag::serialize_value"
    )]
    Pay,
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
    /// Pressing the button will prompt the user to select one of their chats of the specified type,
    /// open that chat and insert the bot username and the specified inline query in the input field
    SwitchInlineQueryChosenChat(SwitchInlineQueryChosenChat),
    /// Pressing the button will insert the bot‘s username and
    /// the specified inline query in the current chat's input field
    ///
    /// Can be empty, in which case only the bot’s username will be inserted
    /// This offers a quick way for the user to open your bot in
    /// inline mode in the same chat – good for selecting something from multiple options
    SwitchInlineQueryCurrentChat(String),
    /// HTTP or tg:// url to be opened when button is pressed
    Url(String),
    /// Description of the Web App that will be launched when the user presses the button
    ///
    /// The Web App will be able to send an arbitrary message on behalf
    /// of the user using the method answerWebAppQuery.
    /// Available only in private chats between a user and the bot.
    WebApp(WebAppInfo),
}

#[derive(Debug, Deserialize, Serialize)]
struct RawButtonEmpty {}

impl RawButtonEmpty {
    fn deserialize_value<'de, D>(deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        RawButtonEmpty::deserialize(deserializer).map(|_| ())
    }

    fn serialize_value<S>(serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        RawButtonEmpty {}.serialize(serializer)
    }
}

#[derive(Deserialize, Serialize)]
struct RawButtonFlag;

impl RawButtonFlag {
    fn deserialize_value<'de, D>(deserializer: D) -> Result<(), D::Error>
    where
        D: Deserializer<'de>,
    {
        True::deserialize(deserializer).map(|_| ())
    }

    fn serialize_value<S>(serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        True.serialize(serializer)
    }
}

/// An error occurred with inline keyboard
#[derive(Debug)]
pub enum InlineKeyboardError {
    /// Can not serialize callback data
    SerializeCallbackData(JsonError),
    /// Can not serialize markup
    SerializeMarkup(JsonError),
}

impl Error for InlineKeyboardError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        use self::InlineKeyboardError::*;
        match self {
            SerializeCallbackData(err) => Some(err),
            SerializeMarkup(err) => Some(err),
        }
    }
}

impl fmt::Display for InlineKeyboardError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::InlineKeyboardError::*;
        match self {
            SerializeCallbackData(err) => write!(out, "failed to serialize callback data: {}", err),
            SerializeMarkup(err) => write!(out, "failed to serialize markup: {}", err),
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

/// Represents an inline button that switches the current user
/// to inline mode in a chosen chat, with an optional default inline query
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct SwitchInlineQueryChosenChat {
    query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_bot_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_channel_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_group_chats: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_user_chats: Option<bool>,
}

impl SwitchInlineQueryChosenChat {
    /// Creates a new SwitchInlineQueryChosenChat
    ///
    /// # Arguments
    ///
    /// * query - The default inline query to be inserted in the input field.
    ///           If left empty, only the bot username will be inserted.
    pub fn new<T>(query: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            query: query.into(),
            allow_bot_chats: None,
            allow_channel_chats: None,
            allow_group_chats: None,
            allow_user_chats: None,
        }
    }

    /// True, if private chats with users can be chosen
    pub fn allow_bot_chats(mut self, value: bool) -> Self {
        self.allow_bot_chats = Some(value);
        self
    }

    /// True, if private chats with bots can be chosen
    pub fn allow_channel_chats(mut self, value: bool) -> Self {
        self.allow_channel_chats = Some(value);
        self
    }

    /// True, if group and supergroup chats can be chosen
    pub fn allow_group_chats(mut self, value: bool) -> Self {
        self.allow_group_chats = Some(value);
        self
    }

    /// True, if channel chats can be chosen
    pub fn allow_user_chats(mut self, value: bool) -> Self {
        self.allow_user_chats = Some(value);
        self
    }
}
