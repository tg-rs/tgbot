use serde::{Deserialize, Serialize};

use crate::types::{LoginUrl, RichText, SwitchInlineQueryChosenChat, WebAppInfo};

/// Represents a style of the rich message button.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RichMessageButtonStyle {
    /// Danger.
    Danger,
    /// Success.
    Success,
    /// Primary.
    Primary,
    /// Link.
    Link,
}

/// Represents a button in a rich message.
///
/// Exactly one of the fields other than text and style must be used to specify the type of the button.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RichMessageButton {
    /// Text of the button.
    pub text: RichText,
    /// Type of the button.
    #[serde(flatten)]
    pub button_type: RichMessageButtonType,
    /// Style of the button.
    pub style: Option<RichMessageButtonStyle>,
}

impl<T> From<(T, RichMessageButtonType)> for RichMessageButton
where
    T: Into<RichText>,
{
    fn from((text, button_type): (T, RichMessageButtonType)) -> Self {
        Self {
            text: text.into(),
            button_type,
            style: None,
        }
    }
}

impl RichMessageButton {
    /// Sets a new style for the button.
    ///
    /// # Arguments
    ///
    /// * `value` - The style to set.
    pub fn with_style(mut self, value: RichMessageButtonStyle) -> Self {
        self.style = Some(value);
        self
    }
}

/// Represents a rich message button type.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RichMessageButtonType {
    /// Data to be sent in a callaback query
    /// to the bot when the button is pressed;
    /// 1-64 bytes.
    CallbackData(String),
    /// A button that copies the specified text to the clipboard.
    CopyText {
        /// The text to be copied to the clipboard; 1-256 characters.
        text: String,
    },
    /// The button is disabled and does nothing.
    Disabled {},
    /// An HTTPs URL used to automatically authorize the user.
    ///
    /// Can be used as a replacement for the Telegram Login Widget.
    ///
    /// Not supported for ephemeral messages.
    LoginUrl(LoginUrl),
    /// Pressing the button will prompt the user to select
    /// one of their chats, open that chat and insert the bot's username
    /// and the specified inline query in the input field.
    ///
    /// May be empty, in which case just the bot's username will be inserted.
    ///
    /// Not supported for messages sent in channel direct messages chats
    /// and on behalf of a business account.
    SwitchInlineQuery(String),
    /// Pressing the button will insert the bot's username
    /// and the specified inline query in the current chat's input field.
    ///
    /// May be empty, in which case only the bot's username will be inserted.
    ///
    /// Not supported in channels and for messages sent in
    /// channel direct messages chats and on behalf of a business account.
    SwitchInlineQueryCurrentChat(String),
    /// Pressing the button will prompt the user
    /// to select one of their chats of the specified type,
    /// open that chat and insert the bot's username
    /// and the specified inline query in the input field.
    ///
    /// Not supported for messages sent in channel direct messages chats
    /// and on behalf of a business account.
    SwitchInlineQueryChosenChat(SwitchInlineQueryChosenChat),
    /// Pressing the button will insert the bot's username
    /// and the specified inline query in the current chat's input field.
    ///
    /// May be empty, in which case only the bot's username will be inserted.
    ///
    /// Not supported in channels and for messages sent in
    /// channel direct messages chats and on behalf of a business account.
    Url(String),
    /// Description of the Web App that will be
    /// launched when the user presses the button.
    ///
    /// The Web App will be able to send an arbitrary message
    /// on behalf of the user using the method AnswerWebAppQuery.
    ///
    /// Available only in private chats between a user and the bot.
    ///
    /// Not supported for the messages sent on behalf of a
    /// business account.
    WebApp(WebAppInfo),
}
