use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{ChatId, Integer},
};

#[cfg(test)]
mod tests;

/// A Bot info returned in getMe
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Bot {
    /// Unique identifier of this bot
    pub id: Integer,
    /// Bots username
    pub username: String,
    /// Bots first name
    pub first_name: String,
    /// Bots last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// True, if the bot can be invited to groups
    pub can_join_groups: bool,
    /// True, if privacy mode is disabled for the bot
    pub can_read_all_group_messages: bool,
    /// True, if the bot supports inline queries
    pub supports_inline_queries: bool,
}

const MIN_NAME_LEN: usize = 1;
const MAX_NAME_LEN: usize = 32;
const MIN_DESCRIPTION_LEN: usize = 3;
const MAX_DESCRIPTION_LEN: usize = 256;

/// This object represents a bot command
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BotCommand {
    command: String,
    description: String,
}

impl BotCommand {
    /// Creates a new BotCommand
    ///
    /// # Arguments
    ///
    /// * name - Name of the command, 1-32 characters
    ///          Can contain only lowercase English letters, digits and underscores
    /// * description - Description of the command, 3-256 characters
    pub fn new<C, D>(name: C, description: D) -> Result<Self, BotCommandError>
    where
        C: Into<String>,
        D: Into<String>,
    {
        let name = name.into();
        let description = description.into();
        let name_len = name.len();
        let description_len = description.len();
        if !(MIN_NAME_LEN..=MAX_NAME_LEN).contains(&name_len) {
            Err(BotCommandError::BadNameLen(name_len))
        } else if !(MIN_DESCRIPTION_LEN..=MAX_DESCRIPTION_LEN).contains(&description_len) {
            Err(BotCommandError::BadDescriptionLen(description_len))
        } else {
            Ok(Self {
                command: name,
                description,
            })
        }
    }

    /// Returns the command name
    pub fn name(&self) -> &str {
        &self.command
    }

    /// Returns the command description
    pub fn description(&self) -> &str {
        &self.description
    }
}

/// An error when creating a new BotCommand
#[derive(Debug)]
pub enum BotCommandError {
    /// Got a name with invalid length
    BadNameLen(usize),
    /// Got a description with invalid length
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
                MIN_NAME_LEN, MAX_NAME_LEN, len
            ),
            BadDescriptionLen(len) => write!(
                out,
                "command description can have a length of {} up to {} characters, got {}",
                MIN_DESCRIPTION_LEN, MAX_DESCRIPTION_LEN, len
            ),
        }
    }
}

/// Represents the scope to which bot commands are applied
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum BotCommandScope {
    /// Represents the default scope of bot commands
    ///
    /// Default commands are used if no commands with a narrower scope are specified for the user
    Default,
    /// Represents the scope of bot commands, covering all private chats
    AllPrivateChats,
    /// Represents the scope of bot commands, covering all group and supergroup chats
    AllGroupChats,
    /// Represents the scope of bot commands, covering all group and supergroup chat administrators.
    AllChatAdministrators,
    /// Represents the scope of bot commands, covering a specific chat.
    Chat {
        /// Unique identifier for the target chat or username of the target supergroup
        chat_id: ChatId,
    },
    /// Represents the scope of bot commands, covering all administrators
    /// of a specific group or supergroup chat.
    ChatAdministrators {
        /// Unique identifier for the target chat or username of the target supergroup
        chat_id: ChatId,
    },
    /// Represents the scope of bot commands, covering a specific member
    /// of a group or supergroup chat.
    ChatMember {
        /// Unique identifier for the target chat or username of the target supergroup
        chat_id: ChatId,
        /// Unique identifier of the target user
        user_id: Integer,
    },
}

impl BotCommandScope {
    /// Creates a new scope covering a specific chat
    pub fn chat<T>(value: T) -> Self
    where
        T: Into<ChatId>,
    {
        Self::Chat { chat_id: value.into() }
    }

    /// Creates a new scope covering all administrators of a specific group or supergroup chat
    pub fn chat_administrators<T>(value: T) -> Self
    where
        T: Into<ChatId>,
    {
        Self::ChatAdministrators { chat_id: value.into() }
    }

    /// Creates a new scope covering a specific member of a group or supergroup chat
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

/// Close the bot instance before moving it from one local server to another
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

/// Use this method to delete the list of the bot commands for the given scope and user language
///
///  After deletion, higher level commands will be shown to affected users
#[derive(Clone, Debug, Default, Serialize)]
pub struct DeleteBotCommands {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

impl DeleteBotCommands {
    /// An object, describing scope of users
    ///
    /// Defaults to BotCommandScopeDefault
    pub fn scope(mut self, value: BotCommandScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// A two-letter ISO 639-1 language code or an empty string
    pub fn language_code<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.language_code = Some(value.into());
        self
    }
}

impl Method for DeleteBotCommands {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("deleteMyCommands", self)
    }
}

/// Returns basic information about the bot
#[derive(Clone, Copy, Debug)]
pub struct GetBot;

impl Method for GetBot {
    type Response = Bot;

    fn into_payload(self) -> Payload {
        Payload::empty("getMe")
    }
}

/// Use this method to get the current list of the bot commands
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetBotCommands {
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

impl GetBotCommands {
    /// An object, describing scope of users
    ///
    /// Defaults to BotCommandScopeDefault
    pub fn scope(mut self, value: BotCommandScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// A two-letter ISO 639-1 language code or an empty string
    pub fn language_code<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.language_code = Some(value.into());
        self
    }
}

impl Method for GetBotCommands {
    type Response = Vec<BotCommand>;

    fn into_payload(self) -> Payload {
        Payload::json("getMyCommands", self)
    }
}

/// Log out from the cloud Bot API
///
/// You must log out the bot before running it locally,
/// otherwise there is no guarantee that the bot will receive updates.
/// After a successful call, you can immediately log in on a local server,
/// but will not be able to log in back to the cloud Bot API server for 10 minutes.
#[derive(Clone, Copy, Debug)]
pub struct LogOut;

impl Method for LogOut {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::empty("logOut")
    }
}

/// Use this method to change the list of the bot commands
#[derive(Clone, Debug, Serialize)]
pub struct SetBotCommands {
    commands: Vec<BotCommand>,
    #[serde(skip_serializing_if = "Option::is_none")]
    scope: Option<BotCommandScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    language_code: Option<String>,
}

impl SetBotCommands {
    /// Creates a new SetBotCommands
    ///
    /// # Arguments
    ///
    /// * commands - Commands to set
    pub fn new(commands: impl IntoIterator<Item = BotCommand>) -> Self {
        Self {
            commands: commands.into_iter().collect(),
            scope: None,
            language_code: None,
        }
    }

    /// Sets a scope of users for which the commands are relevant
    ///
    /// Defaults to BotCommandScopeDefault
    pub fn scope(mut self, value: BotCommandScope) -> Self {
        self.scope = Some(value);
        self
    }

    /// A two-letter ISO 639-1 language code
    ///
    /// If empty, commands will be applied to all users from the given scope,
    /// for whose language there are no dedicated commands
    pub fn language_code<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.language_code = Some(value.into());
        self
    }
}

impl Method for SetBotCommands {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setMyCommands", self)
    }
}
