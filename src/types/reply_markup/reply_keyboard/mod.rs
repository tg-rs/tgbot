use std::ops::Not;

use serde::{Deserialize, Serialize};

use crate::types::{ChatAdministratorRights, Integer, PollKind, True, WebAppInfo};

#[cfg(test)]
mod tests;

/// Custom keyboard with reply options
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ReplyKeyboardMarkup {
    /// Array of button rows, each represented by an Array of KeyboardButton objects
    keyboard: Vec<Vec<KeyboardButton>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_persistent: Option<bool>,
    #[serde(default, skip_serializing_if = "Not::not")]
    resize_keyboard: bool,
    #[serde(default, skip_serializing_if = "Not::not")]
    one_time_keyboard: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_field_placeholder: Option<String>,
    #[serde(default, skip_serializing_if = "Not::not")]
    selective: bool,
}

impl ReplyKeyboardMarkup {
    /// Returns a KeyboardMarkup with given keyboard
    pub fn from_vec(keyboard: Vec<Vec<KeyboardButton>>) -> Self {
        ReplyKeyboardMarkup {
            keyboard,
            is_persistent: None,
            resize_keyboard: false,
            one_time_keyboard: false,
            input_field_placeholder: None,
            selective: false,
        }
    }

    /// Requests clients to always show the keyboard when the regular keyboard is hidden.
    ///
    /// Defaults to false, in which case the custom keyboard can be hidden
    /// and opened with a keyboard icon.
    pub fn persistent(mut self, value: bool) -> Self {
        self.is_persistent = Some(value);
        self
    }

    /// Requests clients to resize the keyboard vertically for optimal fit
    ///
    /// (e.g., make the keyboard smaller if there are just two rows of buttons)
    /// Defaults to false, in which case the custom keyboard
    /// is always of the same height as the app's standard keyboard
    pub fn resize_keyboard(mut self, value: bool) -> Self {
        self.resize_keyboard = value;
        self
    }

    /// Requests clients to hide the keyboard as soon as it's been used
    ///
    /// The keyboard will still be available, but clients will automatically
    /// display the usual letter-keyboard in the chat – the user
    /// can press a special button in the input field to see the custom keyboard again
    /// Defaults to false
    pub fn one_time_keyboard(mut self, value: bool) -> Self {
        self.one_time_keyboard = value;
        self
    }

    /// The placeholder to be shown in the input field when the keyboard is active; 1-64 characters
    pub fn input_field_placeholder<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.input_field_placeholder = Some(value.into());
        self
    }

    /// Use this parameter if you want to show the keyboard to specific users only
    ///
    /// Targets:
    ///
    /// 1. users that are @mentioned in the text of the Message object;
    /// 2. if the bot message is a reply (has reply_to_message_id), sender of the original message
    ///
    /// Example: A user requests to change the bot‘s language,
    /// bot replies to the request with a keyboard to select the new language
    /// Other users in the group don’t see the keyboard
    pub fn selective(mut self, value: bool) -> Self {
        self.selective = value;
        self
    }

    /// Adds a row to keyboard
    pub fn row(mut self, value: Vec<KeyboardButton>) -> Self {
        self.keyboard.push(value);
        self
    }
}

impl From<Vec<Vec<KeyboardButton>>> for ReplyKeyboardMarkup {
    fn from(keyboard: Vec<Vec<KeyboardButton>>) -> ReplyKeyboardMarkup {
        ReplyKeyboardMarkup::from_vec(keyboard)
    }
}

/// Button of the reply keyboard
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct KeyboardButton {
    text: String,
    #[serde(flatten)]
    #[serde(skip_serializing_if = "Option::is_none")]
    kind: Option<KeyboardButtonKind>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[allow(clippy::enum_variant_names)]
#[serde(rename_all = "snake_case")]
enum KeyboardButtonKind {
    RequestChat(KeyboardButtonRequestChat),
    RequestContact(True),
    RequestLocation(True),
    RequestPoll(KeyboardButtonPollType),
    RequestUser(KeyboardButtonRequestUser),
    WebApp(WebAppInfo),
}

impl KeyboardButton {
    /// Creates a new KeyboardButton
    ///
    /// # Arguments
    ///
    /// * text - Text of the button
    ///          If none of the optional fields are used,
    ///          it will be sent as a message when the button is pressed
    pub fn new<S: Into<String>>(text: S) -> Self {
        KeyboardButton {
            text: text.into(),
            kind: None,
        }
    }

    /// If specified, pressing the button will open a list of suitable chats
    ///
    /// Tapping on a chat will send its identifier to the bot in a “chat_shared” service message.
    /// Available in private chats only.
    pub fn request_chat(mut self, value: KeyboardButtonRequestChat) -> Self {
        self.kind = Some(KeyboardButtonKind::RequestChat(value));
        self
    }

    /// The user's phone number will be sent as a contact when the button is pressed
    ///
    /// Available in private chats only
    pub fn request_contact(mut self) -> Self {
        self.kind = Some(KeyboardButtonKind::RequestContact(True));
        self
    }

    /// The user's current location will be sent when the button is pressed
    ///
    /// Available in private chats only
    pub fn request_location(mut self) -> Self {
        self.kind = Some(KeyboardButtonKind::RequestLocation(True));
        self
    }

    /// The user will be asked to create a poll and send it to the bot when the button is pressed
    ///
    /// Available in private chats only
    ///
    /// If quiz is passed, the user will be allowed to create only polls in the quiz mode.
    /// If regular is passed, only regular polls will be allowed.
    /// Otherwise, the user will be allowed to create a poll of any type.
    pub fn request_poll<T>(mut self, button_type: T) -> Self
    where
        T: Into<KeyboardButtonPollType>,
    {
        self.kind = Some(KeyboardButtonKind::RequestPoll(button_type.into()));
        self
    }

    /// If specified, pressing the button will open a list of suitable users
    ///
    /// Tapping on any user will send their identifier
    /// to the bot in a “user_shared” service message.
    /// Available in private chats only.
    pub fn request_user(mut self, value: KeyboardButtonRequestUser) -> Self {
        self.kind = Some(KeyboardButtonKind::RequestUser(value));
        self
    }

    /// The described Web App will be launched when the button is pressed
    ///
    /// Available in private chats only.
    ///
    /// The Web App will be able to send a “web_app_data” service message.
    pub fn web_app(mut self, web_app_info: WebAppInfo) -> Self {
        self.kind = Some(KeyboardButtonKind::WebApp(web_app_info));
        self
    }
}

/// This object represents type of a poll which is allowed to be created
/// and sent when the corresponding button is pressed
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct KeyboardButtonPollType {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    kind: Option<PollKind>,
}

impl From<PollKind> for KeyboardButtonPollType {
    fn from(kind: PollKind) -> Self {
        KeyboardButtonPollType { kind: Some(kind) }
    }
}

impl From<Option<PollKind>> for KeyboardButtonPollType {
    fn from(kind: Option<PollKind>) -> Self {
        KeyboardButtonPollType { kind }
    }
}

/// Defines the criteria used to request a suitable chat
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
    chat_is_forum: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_has_username: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_is_created: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_administrator_rights: Option<ChatAdministratorRights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bot_administrator_rights: Option<ChatAdministratorRights>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bot_is_member: Option<bool>,
}

impl KeyboardButtonRequestChat {
    /// Creates a new KeyboardButtonRequestChat
    ///
    /// # Arguments
    ///
    /// * request_id - Signed 32-bit identifier of the request,
    ///                which will be received back in the ChatShared object.
    ///                Must be unique within the message
    /// * chat_is_channel - Pass True to request a channel chat,
    ///                     pass False to request a group or a supergroup chat.
    pub fn new(request_id: Integer, chat_is_channel: bool) -> Self {
        Self {
            request_id,
            chat_is_channel,
            chat_is_forum: None,
            chat_has_username: None,
            chat_is_created: None,
            user_administrator_rights: None,
            bot_administrator_rights: None,
            bot_is_member: None,
        }
    }

    /// Pass True to request a forum supergroup,
    /// pass False to request a non - forum chat
    ///
    /// If not specified, no additional restrictions are applied.
    pub fn chat_is_forum(mut self, value: bool) -> Self {
        self.chat_is_forum = Some(value);
        self
    }

    /// Pass True to request a supergroup or a channel with a username,
    /// pass False to request a chat without a username
    ///
    /// If not specified, no additional restrictions are applied.
    pub fn chat_has_username(mut self, value: bool) -> Self {
        self.chat_has_username = Some(value);
        self
    }

    /// Pass True to request a chat owned by the user
    ///
    /// Otherwise, no additional restrictions are applied.
    pub fn chat_is_created(mut self, value: bool) -> Self {
        self.chat_is_created = Some(value);
        self
    }

    /// An object listing the required administrator rights of the user in the chat
    ///
    /// The rights must be a superset of bot_administrator_rights.
    /// If not specified, no additional restrictions are applied.
    pub fn user_administrator_rights(mut self, value: ChatAdministratorRights) -> Self {
        self.user_administrator_rights = Some(value);
        self
    }

    /// An object listing the required administrator rights of the bot in the chat
    ///
    /// The rights must be a subset of user_administrator_rights.
    /// If not specified, no additional restrictions are applied.
    pub fn bot_administrator_rights(mut self, value: ChatAdministratorRights) -> Self {
        self.bot_administrator_rights = Some(value);
        self
    }

    /// Pass True to request a chat with the bot as a member
    ///
    /// Otherwise, no additional restrictions are applied.
    pub fn bot_is_member(mut self, value: bool) -> Self {
        self.bot_is_member = Some(value);
        self
    }
}

/// Defines the criteria used to request a suitable user
///
/// The identifier of the selected user will be shared with
/// the bot when the corresponding button is pressed.
///
/// [More about requesting users](https://core.telegram.org/bots/features#chat-and-user-selection)
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct KeyboardButtonRequestUser {
    request_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_is_bot: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_is_premium: Option<bool>,
}

impl KeyboardButtonRequestUser {
    /// Creates a new KeyboardButtonRequestUser
    ///
    /// # Arguments
    ///
    /// * request_id - Signed 32-bit identifier of the request,
    ///                which will be received back in the UserShared object.
    ///                Must be unique within the message
    pub fn new(request_id: Integer) -> Self {
        Self {
            request_id,
            user_is_bot: None,
            user_is_premium: None,
        }
    }

    /// Pass True to request a bot, pass False to request a regular user
    ///
    /// If not specified, no additional restrictions are applied.
    pub fn user_is_bot(mut self, value: bool) -> Self {
        self.user_is_bot = Some(value);
        self
    }

    /// Pass True to request a premium user, pass False to request a non-premium user
    ///
    /// If not specified, no additional restrictions are applied.
    pub fn user_is_premium(mut self, value: bool) -> Self {
        self.user_is_premium = Some(value);
        self
    }
}

/// Requests clients to remove the custom keyboard
///
/// (user will not be able to summon this keyboard;
/// if you want to hide the keyboard from sight but keep it accessible,
/// use one_time_keyboard in ReplyKeyboardMarkup)
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct ReplyKeyboardRemove {
    remove_keyboard: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    selective: Option<bool>,
}

impl Default for ReplyKeyboardRemove {
    /// Returns an new keyboard
    fn default() -> ReplyKeyboardRemove {
        ReplyKeyboardRemove {
            remove_keyboard: true,
            selective: None,
        }
    }
}

impl ReplyKeyboardRemove {
    /// Use this parameter if you want to remove the keyboard for specific users only
    ///
    /// Targets:
    ///
    /// 1. users that are @mentioned in the text of the Message object;
    /// 2. if the bot message is a reply (has reply_to_message_id), sender of the original message
    ///
    /// Example: A user votes in a poll, bot returns confirmation message
    /// in reply to the vote and removes the keyboard for that user,
    /// while still showing the keyboard with poll options to users who haven't voted yet
    pub fn selective(mut self, selective: bool) -> Self {
        self.selective = Some(selective);
        self
    }
}
