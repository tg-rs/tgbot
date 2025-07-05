use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{ChatAdministratorRights, ChatId, Integer, StarAmount},
};

#[cfg(test)]
mod tests;

/// Represents information about a bot returned in [`GetBot`].
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Bot {
    /// The first name of the bot.
    pub first_name: String,
    /// The unique identifier for the bot.
    pub id: Integer,
    /// The username of the bot.
    pub username: String,
    /// Whether the bot can be connected to a Telegram Business account to receive its messages.
    pub can_connect_to_business: bool,
    /// Indicates whether the bot can be invited to groups.
    pub can_join_groups: bool,
    /// Indicates whether privacy mode is disabled, allowing the bot to read all group messages.
    pub can_read_all_group_messages: bool,
    /// Indicates whether the bot has a main Web App.
    pub has_main_web_app: bool,
    /// The last name of the bot.
    pub last_name: Option<String>,
    /// Indicates whether the bot supports inline queries.
    pub supports_inline_queries: bool,
}

impl Bot {
    /// Creates a new `Bot`.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for the bot.
    /// * `username` - The username of the bot.
    /// * `first_name` - The first name of the bot.
    pub fn new<A, B>(id: Integer, username: A, first_name: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            first_name: first_name.into(),
            id,
            username: username.into(),
            can_connect_to_business: false,
            can_join_groups: false,
            can_read_all_group_messages: false,
            has_main_web_app: false,
            last_name: None,
            supports_inline_queries: false,
        }
    }

    /// Sets a new value for the `can_connect_to_business` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the bot can be connected to a Telegram Business account.
    pub fn with_can_connect_to_business(mut self, value: bool) -> Self {
        self.can_connect_to_business = value;
        self
    }

    /// Sets a new value for the `can_join_groups` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the bot can be invited to groups.
    pub fn with_can_join_groups(mut self, value: bool) -> Self {
        self.can_join_groups = value;
        self
    }

    /// Sets a new value for the `can_read_all_group_messages` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether privacy mode is disabled.
    pub fn with_can_read_all_group_messages(mut self, value: bool) -> Self {
        self.can_read_all_group_messages = value;
        self
    }

    /// Sets a new value for the `has_main_web_app` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the bot has a main Web App.
    pub fn with_has_main_web_app(mut self, value: bool) -> Self {
        self.has_main_web_app = value;
        self
    }

    /// Sets a new value for the last name of the bot.
    ///
    /// # Arguments
    ///
    /// * `value` - The last name of the bot.
    pub fn with_last_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.last_name = Some(value.into());
        self
    }

    /// Sets a new value for the `supports_inline_queries` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the bot supports inline queries.
    pub fn with_supports_inline_queries(mut self, value: bool) -> Self {
        self.supports_inline_queries = value;
        self
    }
}

/// Represents a command of a bot.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BotCommand {
    #[serde(rename = "command")]
    name: String,
    description: String,
}

impl BotCommand {
    const MIN_NAME_LEN: usize = 1;
    const MAX_NAME_LEN: usize = 32;
    const MIN_DESCRIPTION_LEN: usize = 3;
    const MAX_DESCRIPTION_LEN: usize = 256;

    /// Creates a new `BotCommand`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the command; 1-32 characters;
    ///   can contain only lowercase English letters, digits and underscores.
    /// * `description` - The description of the command; 3-256 characters.
    pub fn new<C, D>(name: C, description: D) -> Result<Self, BotCommandError>
    where
        C: Into<String>,
        D: Into<String>,
    {
        let name = name.into();
        let description = description.into();
        let name_len = name.len();
        let description_len = description.len();
        if !(Self::MIN_NAME_LEN..=Self::MAX_NAME_LEN).contains(&name_len) {
            Err(BotCommandError::BadNameLen(name_len))
        } else if !(Self::MIN_DESCRIPTION_LEN..=Self::MAX_DESCRIPTION_LEN).contains(&description_len) {
            Err(BotCommandError::BadDescriptionLen(description_len))
        } else {
            Ok(Self { name, description })
        }
    }

    /// Returns the name of the command.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Sets a new name for the command.
    ///
    /// # Arguments
    ///
    /// * `value` - The new name.
    pub fn with_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.name = value.into();
        self
    }

    /// Returns the description of the command.
    pub fn description(&self) -> &str {
        &self.description
    }

    /// Sets a new description for the command.
    ///
    /// # Arguments
    ///
    /// * `value` - The new description.
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.description = value.into();
        self
    }
}

/// Represents an error that can occur when creating a new [`BotCommand`].
#[derive(Debug)]
pub enum BotCommandError {
    /// The provided name has an invalid length.
    BadNameLen(usize),
    /// The provided description has an invalid length.
    BadDescriptionLen(usize),
}

impl Error for BotCommandError {}

impl fmt::Display for BotCommandError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::BotCommandError::*;
        match self {
            BadNameLen(len) => write!(
                out,
                "command name can have a length of {} up to {} characters, got {}",
                BotCommand::MIN_NAME_LEN,
                BotCommand::MAX_NAME_LEN,
                len
            ),
            BadDescriptionLen(len) => write!(
                out,
                "command description can have a length of {} up to {} characters, got {}",
                BotCommand::MIN_DESCRIPTION_LEN,
                BotCommand::MAX_DESCRIPTION_LEN,
                len
            ),
        }
    }
}

/// Represents a scope to which bot commands are applied.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BotCommandScope {
    /// All group and supergroup chat administrators.
    AllChatAdministrators,
    /// All group and supergroup chats.
    AllGroupChats,
    /// All private chats.
    AllPrivateChats,
    /// A specific chat.
    Chat {
        /// Unique identifier of the target chat.
        chat_id: ChatId,
    },
    /// All administrators of a specific group or supergroup chat.
    ChatAdministrators {
        /// Unique identifier of the target chat.
        chat_id: ChatId,
    },
    /// A specific member of a group or supergroup chat.
    ChatMember {
        /// Unique identifier of the target chat.
        chat_id: ChatId,
        /// Unique identifier of the target user.
        user_id: Integer,
    },
    /// Default scope.
    ///
    /// Default commands are used if no commands with a narrower scope are specified for a user.
    Default,
}

impl BotCommandScope {
    /// Creates a new `BotCommandScope` covering a specific chat.
    ///
    /// # Arguments
    ///
    /// * `value` - Chat ID.
    pub fn chat<T>(value: T) -> Self
    where
        T: Into<ChatId>,
    {
        Self::Chat { chat_id: value.into() }
    }

    /// Creates a new `BotCommandScope` covering all administrators
    /// of a specific group or supergroup chat.
    ///
    /// # Arguments
    ///
    /// * `value` - Chat ID.
    pub fn chat_administrators<T>(value: T) -> Self
    where
        T: Into<ChatId>,
    {
        Self::ChatAdministrators { chat_id: value.into() }
    }

    /// Creates a new `BotCommandScope` covering a specific member of a group or supergroup chat.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Chat ID.
    /// * `user_id` - User ID.
    pub fn chat_member<A>(chat_id: A, user_id: Integer) -> Self
    where
        A: Into<ChatId>,
    {
        Self::ChatMember {
            chat_id: chat_id.into(),
            user_id,
        }
    }
}

/// Represents a description of a bot.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct BotDescription {
    /// The description of the bot.
    pub description: String,
}

impl BotDescription {
    /// Creates a new `BotDescription`.
    ///
    /// # Arguments
    ///
    /// * `value` - The description of the bot.
    pub fn new<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            description: value.into(),
        }
    }
}

/// Represents a name of a bot.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct BotName {
    /// The name of the bot.
    pub name: String,
}

impl BotName {
    /// Creates a new `BotName`.
    ///
    /// # Arguments
    ///
    /// * `value` - The name of the bot.
    pub fn new<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self { name: value.into() }
    }
}

/// Represents a short description of a bot.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct BotShortDescription {
    /// The short description of the bot.
    pub short_description: String,
}

impl BotShortDescription {
    /// Creates a new `BotShortDescription`.
    ///
    /// # Arguments
    ///
    /// * `value` - The short description of the bot.
    pub fn new<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            short_description: value.into(),
        }
    }
}

/// Closes a bot instance before moving it from one local server to another.
///
/// You need to delete the webhook before calling this method to ensure
/// that the bot isn't launched again after server restart.
///
/// The method will return error 429 in the first 10 minutes after the bot is launched.
#[derive(Clone, Copy, Debug)]
pub struct Close;

impl Method for Close {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::empty("close")
    }
}

/// Deletes a list of a bot commands of a given scope and user language.
///
/// After deletion, higher level commands will be shown to affected users.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize)]
pub struct DeleteBotCommands {
    language_code: Option<String>,
    scope: Option<BotCommandScope>,
}

impl DeleteBotCommands {
    /// Sets a new language code.
    ///
    /// # Arguments
    ///
    /// * `value` - Two-letter ISO 639-1 language code or an empty string.
    pub fn with_language_code<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.language_code = Some(value.into());
        self
    }

    /// Sets a new scope of users.
    ///
    /// # Arguments
    ///
    /// * `value` - Scope; default - [`BotCommandScope::Default`].
    pub fn with_scope(mut self, value: BotCommandScope) -> Self {
        self.scope = Some(value);
        self
    }
}

impl Method for DeleteBotCommands {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("deleteMyCommands", self)
    }
}

/// Returns a basic information about a bot.
#[derive(Clone, Copy, Debug)]
pub struct GetBot;

impl Method for GetBot {
    type Response = Bot;

    fn into_payload(self) -> Payload {
        Payload::empty("getMe")
    }
}

/// Returns the current list of bot commands.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetBotCommands {
    language_code: Option<String>,
    scope: Option<BotCommandScope>,
}

impl GetBotCommands {
    /// Sets a new language code.
    ///
    /// # Arguments
    ///
    /// * `value` - Two-letter ISO 639-1 language code or an empty string.
    pub fn with_language_code<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.language_code = Some(value.into());
        self
    }

    /// Sets a new scope.
    ///
    /// # Arguments
    ///
    /// * `value` - Scope of users; default - [`BotCommandScope::Default`].
    pub fn with_scope(mut self, value: BotCommandScope) -> Self {
        self.scope = Some(value);
        self
    }
}

impl Method for GetBotCommands {
    type Response = Vec<BotCommand>;

    fn into_payload(self) -> Payload {
        Payload::json("getMyCommands", self)
    }
}

/// Returns the current default administrator rights of a bot.
#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct GetBotDefaultAdministratorRights {
    for_channels: Option<bool>,
}

impl GetBotDefaultAdministratorRights {
    /// Sets a new value of a `for_channels` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - For channels - `true`; for groups and supergroups - `false`.
    pub fn with_for_channels(mut self, value: bool) -> Self {
        self.for_channels = Some(value);
        self
    }
}

impl Method for GetBotDefaultAdministratorRights {
    type Response = ChatAdministratorRights;

    fn into_payload(self) -> Payload {
        Payload::json("getMyDefaultAdministratorRights", self)
    }
}

/// Returns the current description of a bot for a given user language.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetBotDescription {
    language_code: Option<String>,
}

impl GetBotDescription {
    /// Sets a new language code.
    ///
    /// # Arguments
    ///
    /// * `value` - Two-letter ISO 639-1 language code or an empty string.
    pub fn with_language_code<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.language_code = Some(value.into());
        self
    }
}

impl Method for GetBotDescription {
    type Response = BotDescription;

    fn into_payload(self) -> Payload {
        Payload::json("getMyDescription", self)
    }
}

/// Returns the current name of a bot for a given user language.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetBotName {
    language_code: Option<String>,
}

impl GetBotName {
    /// Sets a new language code.
    ///
    /// # Arguments
    ///
    /// * `value` - Two-letter ISO 639-1 language code or an empty string.
    pub fn with_language_code<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.language_code = Some(value.into());
        self
    }
}

impl Method for GetBotName {
    type Response = BotName;

    fn into_payload(self) -> Payload {
        Payload::json("getMyName", self)
    }
}

/// Returns the current short description of a bot for a given user language.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetBotShortDescription {
    language_code: Option<String>,
}

impl GetBotShortDescription {
    /// Sets a new language code.
    ///
    /// # Arguments
    ///
    /// * `value` - Two-letter ISO 639-1 language code or an empty string.
    pub fn with_language_code<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.language_code = Some(value.into());
        self
    }
}

impl Method for GetBotShortDescription {
    type Response = BotShortDescription;

    fn into_payload(self) -> Payload {
        Payload::json("getMyShortDescription", self)
    }
}

/// Returns the current Telegram Stars balance of the bot.
#[derive(Clone, Copy, Debug)]
pub struct GetBotStarBalance;

impl Method for GetBotStarBalance {
    type Response = StarAmount;

    fn into_payload(self) -> Payload {
        Payload::empty("getMyStarBalance")
    }
}

/// Logs out from the Cloud Bot API.
///
/// You must log out a bot before running it locally,
/// otherwise there is no guarantee that the bot will receive updates.
///
/// After a successful call, you can immediately log in on a local server,
/// but will not be able to log in back to the Cloud Bot API server for 10 minutes.
#[derive(Clone, Copy, Debug)]
pub struct LogOut;

impl Method for LogOut {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::empty("logOut")
    }
}

/// Changes a list of commands of a bot.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SetBotCommands {
    commands: Vec<BotCommand>,
    language_code: Option<String>,
    scope: Option<BotCommandScope>,
}

impl SetBotCommands {
    /// Creates a new `SetBotCommands`.
    ///
    /// # Arguments
    ///
    /// * `commands` - Commands to set.
    pub fn new(commands: impl IntoIterator<Item = BotCommand>) -> Self {
        Self {
            commands: commands.into_iter().collect(),
            language_code: None,
            scope: None,
        }
    }

    /// Sets a new language code.
    ///
    /// # Arguments
    ///
    /// * `value` - Two-letter ISO 639-1 language code;
    ///   if empty, commands will be applied to all users from the given scope,
    ///   for whose language there are no dedicated commands.
    pub fn with_language_code<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.language_code = Some(value.into());
        self
    }

    /// Sets a new scope.
    ///
    /// # Arguments
    ///
    /// * `value` - Scope of users for which the commands are relevant;
    ///   default - [`BotCommandScope::Default`].
    pub fn with_scope(mut self, value: BotCommandScope) -> Self {
        self.scope = Some(value);
        self
    }
}

impl Method for SetBotCommands {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setMyCommands", self)
    }
}

/// Changes default administrator rights requested by a bot
/// when it's added as an administrator to groups or channels.
///
/// These rights will be suggested to users,
/// but they are free to modify the list before adding the bot.
#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug, Default, Serialize)]
pub struct SetBotDefaultAdministratorRights {
    for_channels: Option<bool>,
    rights: Option<ChatAdministratorRights>,
}

impl SetBotDefaultAdministratorRights {
    /// Sets a new value of a `for_channels` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - For channels - `true`; for groups and supergroups - `false`.
    pub fn with_for_channels(mut self, value: bool) -> Self {
        self.for_channels = Some(value);
        self
    }

    /// Sets new default administrator rights
    ///
    /// # Arguments
    ///
    /// * `value` - Default administrator rights;
    ///   if not specified, the default administrator rights will be cleared.
    pub fn with_rights(mut self, value: ChatAdministratorRights) -> Self {
        self.rights = Some(value);
        self
    }
}

impl Method for SetBotDefaultAdministratorRights {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setMyDefaultAdministratorRights", self)
    }
}

/// Changes the description of a bot, which is shown in a chat with the bot if the chat is empty.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize)]
pub struct SetBotDescription {
    description: Option<String>,
    language_code: Option<String>,
}

impl SetBotDescription {
    /// Sets a new description.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the bot; 0-512 characters;
    ///   pass an empty string to remove the dedicated description
    ///   of the given language.
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.description = Some(value.into());
        self
    }

    /// Sets a new language code.
    ///
    /// # Arguments
    ///
    /// * `value` - Two-letter ISO 639-1 language code;
    ///   if empty, the description will be applied to all users
    ///   for whose language there is no dedicated description.
    pub fn with_language_code<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.language_code = Some(value.into());
        self
    }
}

impl Method for SetBotDescription {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setMyDescription", self)
    }
}

/// Changes the name of a bot.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize)]
pub struct SetBotName {
    language_code: Option<String>,
    name: Option<String>,
}

impl SetBotName {
    /// Sets a new language code.
    ///
    /// # Arguments
    ///
    /// * `value` - Two-letter ISO 639-1 language code;
    ///   if empty, the name will be shown to all users
    ///   for whose language there is no dedicated name.
    pub fn with_language_code<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.language_code = Some(value.into());
        self
    }

    /// Sets a new name of a bot.
    ///
    /// # Arguments
    ///
    /// * `value` - New name of the bot; 0-64 characters;
    ///   pass an empty string to remove the dedicated name
    ///   of the given language.
    pub fn with_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.name = Some(value.into());
        self
    }
}

impl Method for SetBotName {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setMyName", self)
    }
}

/// Changes the short description of a bot, which is shown on the bot profile page
/// and is sent together with the link when users share the bot.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize)]
pub struct SetBotShortDescription {
    language_code: Option<String>,
    short_description: Option<String>,
}

impl SetBotShortDescription {
    /// Sets a new language code.
    ///
    /// # Arguments
    ///
    /// * `value` - Two-letter ISO 639-1 language code;
    ///   if empty, the short description will be applied
    ///   to all users for whose language there is no dedicated short description.
    pub fn with_language_code<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.language_code = Some(value.into());
        self
    }

    /// Sets a new short description.
    ///
    /// # Arguments
    ///
    /// * `value` - Short description of a bot; 0-120 characters;
    ///   pass an empty string to remove the dedicated short description
    ///   of the given language.
    pub fn with_short_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.short_description = Some(value.into());
        self
    }
}

impl Method for SetBotShortDescription {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setMyShortDescription", self)
    }
}
