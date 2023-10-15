use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{Integer, ParseMode, PhotoSize},
};

#[cfg(test)]
mod tests;

/// Telegram user or bot
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct User {
    /// Unique identifier for this user or bot
    pub id: Integer,
    /// True, if this user is a bot
    pub is_bot: bool,
    /// User‘s or bot’s first name
    pub first_name: String,
    /// User‘s or bot’s last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// User‘s or bot’s username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// IETF language tag of the user's language
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// True, if this user is a Telegram Premium user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,
    /// True, if this user added the bot to the attachment menu
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_to_attachment_menu: Option<bool>,
}

impl User {
    /// Returns full name of the user (first name + last name)
    pub fn get_full_name(&self) -> String {
        let mut full_name = self.first_name.clone();
        if let Some(ref last_name) = self.last_name {
            full_name.push(' ');
            full_name += last_name;
        }
        full_name
    }

    /// Returns a link to the user (tg://user?id=xxx)
    ///
    /// These links will work only if they are used inside an inline link.
    /// For example, they will not work, when used in an inline keyboard button or in a message text.
    pub fn get_link(&self) -> String {
        format!("tg://user?id={}", self.id)
    }

    /// Returns a mention for the user
    ///
    /// These mentions are only guaranteed to work if the user has contacted the bot in the past,
    /// has sent a callback query to the bot via inline button or is a member
    /// in the group where he was mentioned.
    pub fn get_mention(&self, parse_mode: ParseMode) -> Result<String, MentionError> {
        let full_name = parse_mode.escape(self.get_full_name());
        let user_link = self.get_link();
        Ok(match parse_mode {
            ParseMode::Markdown => return Err(MentionError::UnsupportedParseMode(parse_mode)),
            ParseMode::MarkdownV2 => format!(r#"[{}]({})"#, full_name, user_link),
            ParseMode::Html => format!(r#"<a href="{}">{}</a>"#, user_link, full_name),
        })
    }
}

/// An error occurred when getting user mention
#[derive(Debug)]
pub enum MentionError {
    /// Parse mode is not supported
    UnsupportedParseMode(ParseMode),
}

impl Error for MentionError {}

impl fmt::Display for MentionError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MentionError::UnsupportedParseMode(parse_mode) => {
                write!(out, "can not mention with {} parse mode", parse_mode)
            }
        }
    }
}

/// User's profile pictures
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct UserProfilePhotos {
    /// Total number of profile pictures the target user has
    pub total_count: Integer,
    /// Requested profile pictures (in up to 4 sizes each)
    pub photos: Vec<Vec<PhotoSize>>,
}

/// User ID
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum UserId {
    /// @username of a user
    Username(String),
    /// ID of a user
    Id(Integer),
}

impl From<&str> for UserId {
    fn from(username: &str) -> UserId {
        UserId::Username(String::from(username))
    }
}

impl From<String> for UserId {
    fn from(username: String) -> UserId {
        UserId::Username(username)
    }
}

impl From<Integer> for UserId {
    fn from(id: Integer) -> UserId {
        UserId::Id(id)
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserId::Username(username) => write!(out, "{}", username),
            UserId::Id(chat_id) => write!(out, "{}", chat_id),
        }
    }
}

/// Get a list of profile pictures for a user
#[derive(Clone, Debug, Serialize)]
pub struct GetUserProfilePhotos {
    user_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Integer>,
}

impl GetUserProfilePhotos {
    /// Creates a new GetUserProfilePhotos with empty optional parameters
    ///
    /// # Arguments
    ///
    /// user_id - Unique identifier of the target user
    pub fn new(user_id: Integer) -> Self {
        GetUserProfilePhotos {
            user_id,
            offset: None,
            limit: None,
        }
    }

    /// Sequential number of the first photo to be returned
    ///
    /// By default, all photos are returned
    pub fn offset(mut self, offset: Integer) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Limits the number of photos to be retrieved
    ///
    /// Values between 1—100 are accepted
    /// Defaults to 100
    pub fn limit(mut self, limit: Integer) -> Self {
        self.limit = Some(limit);
        self
    }
}

impl Method for GetUserProfilePhotos {
    type Response = UserProfilePhotos;

    fn into_payload(self) -> Payload {
        Payload::json("getUserProfilePhotos", self)
    }
}
