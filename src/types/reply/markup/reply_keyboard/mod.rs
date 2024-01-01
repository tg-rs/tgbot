use std::ops::Not;

use serde::{Deserialize, Serialize};

use crate::types::{ChatAdministratorRights, Integer, PollType, True, WebAppInfo};

#[cfg(test)]
mod tests;

/// Represents a custom keyboard with reply options.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ReplyKeyboardMarkup {
    keyboard: Vec<Vec<KeyboardButton>>,
    #[serde(default, skip_serializing_if = "Not::not")]
    one_time_keyboard: bool,
    #[serde(default, skip_serializing_if = "Not::not")]
    resize_keyboard: bool,
    #[serde(default, skip_serializing_if = "Not::not")]
    selective: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_field_placeholder: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_persistent: Option<bool>,
}

impl ReplyKeyboardMarkup {
    /// Adds a row to the markup.
    ///
    /// # Arguments
    ///
    /// * `value` - The row to add.
    pub fn add_row<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = KeyboardButton>,
    {
        self.keyboard.push(value.into_iter().collect());
        self
    }

    /// Sets a new input field placeholder.
    ///
    /// # Arguments
    ///
    /// * `value` - The placeholder to be shown
    ///             in the input field
    ///             when the keyboard is active;
    ///             1-64 characters.
    pub fn with_input_field_placeholder<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.input_field_placeholder = Some(value.into());
        self
    }

    /// Sets a new value for an `one_time_keyboard` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to request clients to hide the keyboard
    ///             as soon as it's been used; default - false.
    ///
    /// The keyboard will still be available, but clients will automatically
    /// display the usual letter-keyboard in the chat – the user
    /// can press a special button in the input field to see the custom keyboard again.
    pub fn with_one_time_keyboard(mut self, value: bool) -> Self {
        self.one_time_keyboard = value;
        self
    }

    /// Sets a new value for an `is_persistent` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to request clients to always show
    ///             the keyboard when the regular keyboard is hidden.
    ///
    /// Defaults to false, in which case the custom keyboard can be hidden
    /// and opened with a keyboard icon.
    pub fn with_is_persistent(mut self, value: bool) -> Self {
        self.is_persistent = Some(value);
        self
    }

    /// Sets a new value for a `resize_keyboard` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to request clients
    ///             to resize the keyboard vertically for optimal fit.
    ///
    /// E.g., make the keyboard smaller if there are just two rows of buttons.
    /// Defaults to false, in which case the custom keyboard
    /// is always of the same height as the app's standard keyboard.
    pub fn with_resize_keyboard(mut self, value: bool) -> Self {
        self.resize_keyboard = value;
        self
    }

    /// Sets a new value for a `selective` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to show the keyboard to specific users only.
    ///
    /// Targets:
    ///
    /// 1. users that are @mentioned in the text of the [`crate::types::Message`] object.
    /// 2. if the bot message is a reply (has `reply_to_message_id`), sender of the original message.
    ///
    /// Example: A user requests to change the bot‘s language,
    /// bot replies to the request with a keyboard to select the new language.
    /// Other users in the group don’t see the keyboard.
    pub fn with_selective(mut self, value: bool) -> Self {
        self.selective = value;
        self
    }
}

impl<A, B> From<A> for ReplyKeyboardMarkup
where
    A: IntoIterator<Item = B>,
    B: IntoIterator<Item = KeyboardButton>,
{
    fn from(value: A) -> ReplyKeyboardMarkup {
        Self {
            keyboard: value.into_iter().map(|x| x.into_iter().collect()).collect(),
            one_time_keyboard: false,
            resize_keyboard: false,
            selective: false,
            input_field_placeholder: None,
            is_persistent: None,
        }
    }
}

/// Represents a button of the reply keyboard.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct KeyboardButton {
    text: String,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    button_type: Option<KeyboardButtonType>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[allow(clippy::enum_variant_names)]
#[serde(rename_all = "snake_case")]
enum KeyboardButtonType {
    RequestChat(KeyboardButtonRequestChat),
    RequestContact(True),
    RequestLocation(True),
    RequestPoll(KeyboardButtonPollType),
    RequestUsers(KeyboardButtonRequestUsers),
    WebApp(WebAppInfo),
}

impl KeyboardButton {
    /// Creates a new `KeyboardButton`.
    ///
    /// # Arguments
    ///
    /// * `text` - Text of the button.
    ///
    /// If none of the optional fields are used,
    /// it will be sent as a message when the button is pressed.
    pub fn new<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            text: text.into(),
            button_type: None,
        }
    }

    /// Changes button type to a chat request.
    ///
    /// # Arguments
    ///
    /// * `value` - Criteria used to request a suitable chat.
    ///
    /// If specified, pressing the button will open a list of suitable chats.
    /// Tapping on a chat will send its identifier
    /// to the bot in a [`crate::types::MessageData::ChatShared`] message.
    ///
    /// Available in private chats only.
    pub fn with_request_chat(mut self, value: KeyboardButtonRequestChat) -> Self {
        self.button_type = Some(KeyboardButtonType::RequestChat(value));
        self
    }

    /// Changes button type to a contact request.
    ///
    /// The user's phone number will be sent as a contact when the button is pressed.
    ///
    /// Available in private chats only.
    pub fn with_request_contact(mut self) -> Self {
        self.button_type = Some(KeyboardButtonType::RequestContact(True));
        self
    }

    /// Changes button type to a location request.
    ///
    /// The user's current location will be sent when the button is pressed.
    ///
    /// Available in private chats only.
    pub fn with_request_location(mut self) -> Self {
        self.button_type = Some(KeyboardButtonType::RequestLocation(True));
        self
    }

    /// Changes button type to a poll request.
    ///
    /// # Arguments
    ///
    /// * `value` - Type of a poll.
    ///
    /// The user will be asked to create a poll and send it to the bot when the button is pressed.
    /// If quiz is passed, the user will be allowed to create only polls in the quiz mode.
    /// If regular is passed, only regular polls will be allowed.
    /// Otherwise, the user will be allowed to create a poll of any type.
    ///
    /// Available in private chats only.
    pub fn with_request_poll<T>(mut self, button_type: T) -> Self
    where
        T: Into<KeyboardButtonPollType>,
    {
        self.button_type = Some(KeyboardButtonType::RequestPoll(button_type.into()));
        self
    }

    /// Changes button type to a user request.
    ///
    /// # Arguments
    ///
    /// * `value` - Criteria used to request a suitable user.
    ///
    /// If specified, pressing the button will open a list of suitable users.
    /// Tapping on any user will send their identifier
    /// to the bot in a [`crate::types::MessageData::UserShared`] message.
    ///
    /// Available in private chats only.
    pub fn with_request_users(mut self, value: KeyboardButtonRequestUsers) -> Self {
        self.button_type = Some(KeyboardButtonType::RequestUsers(value));
        self
    }

    /// Changes button type to a web app.
    ///
    /// # Arguments
    ///
    /// * `value` - The Web App that will be launched when the button is pressed.
    ///
    /// The Web App will be able to send a [`crate::types::MessageData::WebAppData`] message.
    ///
    /// Available in private chats only.
    pub fn with_web_app(mut self, web_app_info: WebAppInfo) -> Self {
        self.button_type = Some(KeyboardButtonType::WebApp(web_app_info));
        self
    }
}

/// Represents a type of a poll which is allowed to be created
/// and sent when the corresponding button is pressed.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct KeyboardButtonPollType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    poll_type: Option<PollType>,
}

impl From<PollType> for KeyboardButtonPollType {
    fn from(value: PollType) -> Self {
        Self { poll_type: Some(value) }
    }
}

impl From<Option<PollType>> for KeyboardButtonPollType {
    fn from(value: Option<PollType>) -> Self {
        Self { poll_type: value }
    }
}

/// Represents a criteria used to request a suitable chat.
///
/// The identifier of the selected chat will be shared with
/// the bot when the corresponding button is pressed.
///
/// [More about requesting chats](https://core.telegram.org/bots/features#chat-and-user-selection)
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct KeyboardButtonRequestChat {
    request_id: Integer,
    chat_is_channel: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    bot_administrator_rights: Option<ChatAdministratorRights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bot_is_member: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_is_created: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_is_forum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_has_username: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_administrator_rights: Option<ChatAdministratorRights>,
}

impl KeyboardButtonRequestChat {
    /// Creates a new `KeyboardButtonRequestChat`.
    ///
    /// # Arguments
    ///
    /// * `request_id` - Signed 32-bit identifier of the request,
    ///                  which will be received back
    ///                  in the [`crate::types::MessageDataChatShared`] object;
    ///                  must be unique within the message.
    /// * `chat_is_channel` - Indicates whether to request a channel chat
    ///                       or a group/supergroup chat.
    pub fn new(request_id: Integer, chat_is_channel: bool) -> Self {
        Self {
            request_id,
            chat_is_channel,
            bot_administrator_rights: None,
            bot_is_member: None,
            chat_is_created: None,
            chat_is_forum: None,
            chat_has_username: None,
            user_administrator_rights: None,
        }
    }

    /// Sets a new bot administrator rights.
    ///
    /// # Arguments
    ///
    /// * `value` - An object listing the required administrator rights of the bot in the chat.
    ///
    /// The rights must be a subset of `user_administrator_rights`.
    /// If not specified, no additional restrictions are applied.
    pub fn with_bot_administrator_rights(mut self, value: ChatAdministratorRights) -> Self {
        self.bot_administrator_rights = Some(value);
        self
    }

    /// Sets a new value for a `bot_is_member` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to request a chat with the bot as a member.
    ///
    /// If not specified, no additional restrictions are applied.
    pub fn with_bot_is_member(mut self, value: bool) -> Self {
        self.bot_is_member = Some(value);
        self
    }

    /// Sets a new value for a `chat_is_created` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to request a chat owned by the user.
    ///
    /// If not specified, no additional restrictions are applied.
    pub fn with_chat_is_created(mut self, value: bool) -> Self {
        self.chat_is_created = Some(value);
        self
    }

    /// Sets a new value for a `chat_is_forum` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to request a forum supergroup or a non-forum chat.
    ///
    /// If not specified, no additional restrictions are applied.
    pub fn with_chat_is_forum(mut self, value: bool) -> Self {
        self.chat_is_forum = Some(value);
        self
    }

    /// Sets a new value for a `chat_has_username` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to request a supergroup or a channel with a username.
    ///
    /// If not specified, no additional restrictions are applied.
    pub fn with_chat_has_username(mut self, value: bool) -> Self {
        self.chat_has_username = Some(value);
        self
    }

    /// Sets a new user administrator rights.
    ///
    /// # Arguments
    ///
    /// * `value` - An object listing the required administrator rights of the user in the chat.
    ///
    /// The rights must be a superset of `bot_administrator_rights`.
    /// If not specified, no additional restrictions are applied.
    pub fn with_user_administrator_rights(mut self, value: ChatAdministratorRights) -> Self {
        self.user_administrator_rights = Some(value);
        self
    }
}

/// Represents a criteria used to request a suitable user.
///
/// The identifier of the selected user will be shared with
/// the bot when the corresponding button is pressed.
///
/// [More about requesting users](https://core.telegram.org/bots/features#chat-and-user-selection)
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct KeyboardButtonRequestUsers {
    request_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_quantity: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_is_bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_is_premium: Option<bool>,
}

impl KeyboardButtonRequestUsers {
    /// Creates a new `KeyboardButtonRequestUsers`.
    ///
    /// # Arguments
    ///
    /// * `request_id` - Signed 32-bit identifier of the request,
    ///                  which will be received back
    ///                  in the [`crate::types::MessageDataUserShared`] object;
    ///                  must be unique within the message.
    pub fn new(request_id: Integer) -> Self {
        Self {
            request_id,
            max_quantity: None,
            user_is_bot: None,
            user_is_premium: None,
        }
    }

    /// Sets a new max quantity
    ///
    /// # Arguments
    ///
    /// * `value` - The maximum number of users to be selected; 1-10; default - 1.
    pub fn with_max_quantity(mut self, value: Integer) -> Self {
        self.max_quantity = Some(value);
        self
    }

    /// Sets a new value for a `user_is_bot` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to request a bot or a regular user.
    ///
    /// If not specified, no additional restrictions are applied.
    pub fn with_user_is_bot(mut self, value: bool) -> Self {
        self.user_is_bot = Some(value);
        self
    }

    /// Sets a new value for a `user_is_premium` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to request a premium user.
    ///
    /// If not specified, no additional restrictions are applied.
    pub fn with_user_is_premium(mut self, value: bool) -> Self {
        self.user_is_premium = Some(value);
        self
    }
}

/// Represents a trigger to remove the custom keyboard.
///
/// User will not be able to summon this keyboard.
/// If you want to hide the keyboard from sight but keep it accessible,
/// use [`ReplyKeyboardMarkup::with_one_time_keyboard`].
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ReplyKeyboardRemove {
    remove_keyboard: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    selective: Option<bool>,
}

impl Default for ReplyKeyboardRemove {
    fn default() -> Self {
        Self {
            remove_keyboard: true,
            selective: None,
        }
    }
}

impl ReplyKeyboardRemove {
    /// Sets a new value for a `selective` flag.
    ///
    ///# Arguments
    ///
    /// * `value` - Indicates whether to remove the keyboard for specific users only.
    ///
    /// Targets:
    ///
    /// 1. users that are @mentioned in the text of the Message object;
    /// 2. if the bot message is a reply (has `reply_to_message_id`),
    ///    sender of the original message
    ///
    /// Example: A user votes in a poll, bot returns confirmation message
    /// in reply to the vote and removes the keyboard for that user,
    /// while still showing the keyboard with poll options to users who haven't voted yet.
    pub fn with_selective(mut self, value: bool) -> Self {
        self.selective = Some(value);
        self
    }
}
