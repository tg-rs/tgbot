use std::{error::Error, fmt};

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Error as JsonError;

use crate::types::{True, WebAppInfo};

/// Represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct InlineKeyboardMarkup {
    inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardMarkup {
    /// Adds a row to the markup.
    ///
    /// # Arguments
    ///
    /// * `value` - The row to add.
    pub fn add_row<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = InlineKeyboardButton>,
    {
        self.inline_keyboard.push(value.into_iter().collect());
        self
    }

    pub(crate) fn serialize(&self) -> Result<String, InlineKeyboardError> {
        serde_json::to_string(self).map_err(InlineKeyboardError::SerializeMarkup)
    }
}

impl<A, B> From<A> for InlineKeyboardMarkup
where
    A: IntoIterator<Item = B>,
    B: IntoIterator<Item = InlineKeyboardButton>,
{
    fn from(value: A) -> InlineKeyboardMarkup {
        Self {
            inline_keyboard: value.into_iter().map(|x| x.into_iter().collect()).collect(),
        }
    }
}

impl From<InlineKeyboardMarkup> for Vec<Vec<InlineKeyboardButton>> {
    fn from(value: InlineKeyboardMarkup) -> Self {
        value.inline_keyboard
    }
}

/// Represents a button of an inline keyboard.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineKeyboardButton {
    text: String,
    icon_custom_emoji_id: Option<String>,
    style: Option<InlineKeyboardButtonStyle>,
    #[serde(flatten)]
    button_type: InlineKeyboardButtonType,
}

impl InlineKeyboardButton {
    fn new<T>(text: T, button_type: InlineKeyboardButtonType) -> Self
    where
        T: Into<String>,
    {
        Self {
            text: text.into(),
            icon_custom_emoji_id: None,
            style: None,
            button_type,
        }
    }

    /// Creates a new `InlineKeyboardButton`.
    ///
    /// # Arguments
    ///
    /// * `text` - Text of the button.
    /// * `data` - Data to be sent in a callback query to the bot when button is pressed; 1-64 bytes.
    pub fn for_callback_data<A, B>(text: A, data: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonType::CallbackData(data.into()))
    }

    /// Creates a new `InlineKeyboardButton`.
    ///
    /// # Arguments
    ///
    /// * `text` - Text of the button.
    /// * `data` - Data to be sent in a callback query.
    ///
    /// Same as [`Self::for_callback_data`], but takes a serializable type.
    ///
    /// Data will be serialized using [`serde_json`].
    pub fn for_callback_data_struct<A, B>(text: A, data: &B) -> Result<Self, InlineKeyboardError>
    where
        A: Into<String>,
        B: Serialize,
    {
        let data = serde_json::to_string(data).map_err(InlineKeyboardError::SerializeCallbackData)?;
        Ok(Self::new(text, InlineKeyboardButtonType::CallbackData(data)))
    }

    /// Creates a new `InlineKeyboardButton`.
    ///
    /// # Arguments
    ///
    /// * `text` - Text of the button.
    ///
    /// A game will be launched when the user presses the button.
    ///
    /// # Notes
    ///
    /// This type of button must always be the first button in the first row.
    pub fn for_callback_game<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonType::CallbackGame)
    }

    /// Creates a new `InlineKeyboardButton`.
    ///
    /// # Arguments
    ///
    /// * `text` - Text of the button.
    /// * `value` - The text to be copied to the clipboard;
    ///   1-256 characters.
    pub fn for_copy_text<A, B>(text: A, value: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonType::CopyText(value.into()))
    }

    /// Creates a new `InlineKeyboardButton`.
    ///
    /// # Arguments
    ///
    /// * `text` - Text of the button.
    /// * `data` - An HTTPs URL used to automatically authorize the user.
    ///
    /// Can be used as a replacement for the [Telegram Login Widget][1].
    ///
    /// [1]: https://core.telegram.org/widgets/login
    pub fn for_login_url<A, B>(text: A, data: B) -> Self
    where
        A: Into<String>,
        B: Into<LoginUrl>,
    {
        Self::new(text, InlineKeyboardButtonType::LoginUrl(data.into()))
    }

    /// Creates a new `InlineKeyboardButton`.
    ///
    /// # Arguments
    ///
    /// * `text` - Text of the button.
    ///
    /// Represents a pay button.
    ///
    /// <https://core.telegram.org/bots/payments>
    ///
    /// # Notes
    ///
    /// This type of button must always be the first button in the
    /// first row and can only be used in invoice messages.
    pub fn for_pay<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonType::Pay)
    }

    /// Creates a new `InlineKeyboardButton`.
    ///
    /// # Arguments
    ///
    /// * `text` - Text of the button.
    /// * `data` - Text of an inline query.
    ///
    /// Pressing the button will prompt the user to select one of their chats,
    /// open that chat and insert the bot‘s username and
    /// the specified inline query in the input field.
    ///
    /// Can be empty, in which case just the bot’s username will be inserted.
    ///
    /// # Notes
    ///
    /// This offers an easy way for users to start using your bot
    /// in inline mode when they are currently in a private chat with it.
    ///
    /// Especially useful when combined with switch_pm… actions – in this case the user
    /// will be automatically returned to the chat they switched from,
    /// skipping the chat selection screen.
    pub fn for_switch_inline_query<A, B>(text: A, data: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonType::SwitchInlineQuery(data.into()))
    }

    /// Creates a new `InlineKeyboardButton`.
    ///
    /// # Arguments
    ///
    /// * `text` - Text of the button.
    /// * `data` - Inline query parameters.
    ///
    /// Pressing the button will prompt the user to select one of their chats of the specified type,
    /// open that chat and insert the bot username and the specified inline query in the input field.
    pub fn for_switch_inline_query_chosen_chat<T>(text: T, data: SwitchInlineQueryChosenChat) -> Self
    where
        T: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonType::SwitchInlineQueryChosenChat(data))
    }

    /// Creates a new `InlineKeyboardButton`.
    ///
    /// # Arguments
    ///
    /// * `text` - Text of the button.
    /// * `data` - Text of an inline query.
    ///
    /// Pressing the button will insert the bot‘s username and
    /// the specified inline query in the current chat's input field.
    ///
    /// Can be empty, in which case only the bot’s username will be inserted
    /// This offers a quick way for the user to open your bot in
    /// inline mode in the same chat – good for selecting something from multiple options.
    pub fn for_switch_inline_query_current_chat<A, B>(text: A, data: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self::new(
            text,
            InlineKeyboardButtonType::SwitchInlineQueryCurrentChat(data.into()),
        )
    }

    /// Creates a new `InlineKeyboardButton`.
    ///
    /// # Arguments
    ///
    /// * `text` - Text of the button
    /// * `data` - HTTP or `tg://` URL to be opened when button is pressed.
    pub fn for_url<A, B>(text: A, data: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonType::Url(data.into()))
    }

    /// Creates a new `InlineKeyboardButton`.
    ///
    /// # Arguments
    ///
    /// * `text` - Text of the button.
    /// * `data` - Description of the Web App that will be launched when the user presses the button.
    pub fn for_web_app<T>(text: T, data: WebAppInfo) -> Self
    where
        T: Into<String>,
    {
        Self::new(text, InlineKeyboardButtonType::WebApp(data))
    }

    /// Returns the type of the button.
    pub fn button_type(&self) -> &InlineKeyboardButtonType {
        &self.button_type
    }

    /// Returns the text of the button.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Sets a new icon custom emoji ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the custom emoji shown before the text of the button.
    ///
    /// Can only be used by bots that purchased additional usernames on Fragment
    /// or in the messages directly sent by the bot to private, group and supergroup chats
    /// if the owner of the bot has a Telegram Premium subscription.
    pub fn with_icon_custom_emoji_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.icon_custom_emoji_id = Some(value.into());
        self
    }

    /// Sets a new style.
    ///
    /// # Arguments
    ///
    /// * `value` - Style of the button.
    ///
    /// If omitted, then an app-specific style is used.
    pub fn with_style(mut self, value: InlineKeyboardButtonStyle) -> Self {
        self.style = Some(value);
        self
    }
}

/// Represents a style of an inline keyboard button.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InlineKeyboardButtonStyle {
    /// Red.
    Danger,
    /// Blue.
    Primary,
    /// Green.
    Success,
}

/// Represents a type of an inline keyboard button.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InlineKeyboardButtonType {
    /// Data to be sent in a callback query to the bot when button is pressed; 1-64 bytes.
    CallbackData(String),
    /// Description of the game that will be launched when the user presses the button.
    ///
    /// # Notes
    ///
    /// This type of button must always be the first button in the first row.
    #[serde(
        deserialize_with = "RawButtonEmpty::deserialize_value",
        serialize_with = "RawButtonEmpty::serialize_value"
    )]
    CallbackGame,
    /// Description of the button that copies the specified text to the clipboard.
    #[serde(
        deserialize_with = "RawButtonText::deserialize_value",
        serialize_with = "RawButtonText::serialize_value"
    )]
    CopyText(String),
    /// An HTTP URL used to automatically authorize the user.
    ///
    /// Can be used as a replacement for the [Telegram Login Widget][1].
    ///
    /// [1]: https://core.telegram.org/widgets/login
    LoginUrl(LoginUrl),
    /// Send a Pay button.
    ///
    /// # Notes
    ///
    /// This type of button must always be the first button in the first row.
    #[serde(
        deserialize_with = "RawButtonFlag::deserialize_value",
        serialize_with = "RawButtonFlag::serialize_value"
    )]
    Pay,
    /// Pressing the button will prompt the user to select one of their chats,
    /// open that chat and insert the bot‘s username and
    /// the specified inline query in the input field.
    ///
    /// Can be empty, in which case just the bot’s username will be inserted.
    ///
    /// # Notes
    ///
    /// This offers an easy way for users to start using your bot
    /// in inline mode when they are currently in a private chat with it.
    ///
    /// Especially useful when combined with switch_pm… actions – in this case the user
    /// will be automatically returned to the chat they switched from,
    /// skipping the chat selection screen.
    SwitchInlineQuery(String),
    /// Pressing the button will prompt the user to select one of their chats of the specified type,
    /// open that chat and insert the bot username and the specified inline query in the input field.
    SwitchInlineQueryChosenChat(SwitchInlineQueryChosenChat),
    /// Pressing the button will insert the bot‘s username and
    /// the specified inline query in the current chat's input field.
    ///
    /// Can be empty, in which case only the bot’s username will be inserted.
    /// This offers a quick way for the user to open your bot in
    /// inline mode in the same chat – good for selecting something from multiple options.
    SwitchInlineQueryCurrentChat(String),
    /// HTTP or tg:// url to be opened when button is pressed.
    Url(String),
    /// Description of the Web App that will be launched when the user presses the button.
    ///
    /// The Web App will be able to send an arbitrary message on behalf
    /// of the user using the method [`crate::types::AnswerWebAppQuery`].
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

#[derive(Deserialize, Serialize)]
struct RawButtonText {
    text: String,
}

impl RawButtonText {
    fn deserialize_value<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        RawButtonText::deserialize(deserializer).map(|RawButtonText { text }| text)
    }

    fn serialize_value<S>(value: &str, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        RawButtonText {
            text: String::from(value),
        }
        .serialize(serializer)
    }
}

/// Represents an error occurred with an inline keyboard.
#[derive(Debug)]
pub enum InlineKeyboardError {
    /// Can not serialize callback data.
    SerializeCallbackData(JsonError),
    /// Can not serialize markup.
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
            SerializeCallbackData(err) => write!(out, "failed to serialize callback data: {err}"),
            SerializeMarkup(err) => write!(out, "failed to serialize markup: {err}"),
        }
    }
}

/// Represents a parameter of the inline keyboard button used to automatically authorize a user.
///
/// Serves as a great replacement for the
/// Telegram Login Widget when the user is coming from Telegram.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct LoginUrl {
    url: String,
    bot_username: Option<String>,
    forward_text: Option<String>,
    request_write_access: Option<bool>,
}

impl LoginUrl {
    /// Creates a new `LoginUrl`.
    ///
    /// # Arguments
    ///
    /// * `value` - An HTTP URL to be opened with user authorization data
    ///   added to the query string when the button is pressed.
    ///
    /// If the user refuses to provide authorization data,
    /// the original URL without information about the user will be opened.
    ///
    /// The data added is the same as described in [Receiving authorization data][1].
    ///
    /// # Notes
    ///
    /// You **must** always check the hash of the received data to verify the authentication
    /// and the integrity of the data as described in [Checking authorization][2].
    ///
    /// [1]: https://core.telegram.org/widgets/login#receiving-authorization-data
    /// [2]: https://core.telegram.org/widgets/login#checking-authorization
    pub fn new<T>(url: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            url: url.into(),
            bot_username: None,
            forward_text: None,
            request_write_access: None,
        }
    }

    /// Sets a new username of a bot.
    ///
    /// # Arguments
    ///
    /// * `value` - The username of the bot, which will be used for user authorization.
    ///
    /// See [Setting up a bot][1] for more details.
    /// If not specified, the current bot username will be assumed.
    /// The url domain must be the same as the domain linked with the bot.
    /// See [Linking your domain to the bot][2] for more details.
    ///
    /// [1]: https://core.telegram.org/widgets/login#setting-up-a-bot
    /// [2]: https://core.telegram.org/widgets/login#linking-your-domain-to-the-bot
    pub fn with_bot_username<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.bot_username = Some(value.into());
        self
    }

    /// Sets a new forward text.
    ///
    /// # Arguments
    ///
    /// * `value` - New text of the button in forwarded messages.
    pub fn with_forward_text<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.forward_text = Some(value.into());
        self
    }

    /// Sets a new value for the `request_write_access` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to request the permission
    ///   for your bot to send messages to the user.
    pub fn with_request_write_access(mut self, value: bool) -> Self {
        self.request_write_access = Some(value);
        self
    }
}

impl<T> From<T> for LoginUrl
where
    T: Into<String>,
{
    fn from(url: T) -> Self {
        Self::new(url)
    }
}

/// Represents an inline button that switches the current user
/// to inline mode in a chosen chat, with an optional default inline query.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct SwitchInlineQueryChosenChat {
    query: String,
    allow_bot_chats: Option<bool>,
    allow_channel_chats: Option<bool>,
    allow_group_chats: Option<bool>,
    allow_user_chats: Option<bool>,
}

impl SwitchInlineQueryChosenChat {
    /// Creates a new `SwitchInlineQueryChosenChat`.
    ///
    /// # Arguments
    ///
    /// * `query` - The default inline query to be inserted in the input field.
    ///   If left empty, only the bot username will be inserted.
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

    /// Sets a new value for the `allow_bot_chats` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether private chats with users can be chosen.
    pub fn with_allow_bot_chats(mut self, value: bool) -> Self {
        self.allow_bot_chats = Some(value);
        self
    }

    /// Sets a new value for the `allow_channel_chats` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether private chats with bots can be chosen.
    pub fn with_allow_channel_chats(mut self, value: bool) -> Self {
        self.allow_channel_chats = Some(value);
        self
    }

    /// Sets a new value for the `allow_group_chats` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether group and supergroup chats can be chosen.
    pub fn with_allow_group_chats(mut self, value: bool) -> Self {
        self.allow_group_chats = Some(value);
        self
    }

    /// Sets a new value for the `allow_user_chats` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether channel chats can be chosen.
    pub fn with_allow_user_chats(mut self, value: bool) -> Self {
        self.allow_user_chats = Some(value);
        self
    }
}
