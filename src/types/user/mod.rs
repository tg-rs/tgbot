use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{Integer, ParseMode, PhotoSize},
};

#[cfg(test)]
mod tests;

/// Represents a user
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct User {
    /// First name
    pub first_name: String,
    /// Unique identifier
    pub id: Integer,
    /// Whether the user is a bot
    pub is_bot: bool,
    /// Whether the user added the bot to the attachment menu
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_to_attachment_menu: Option<bool>,
    /// Whether the user is a Telegram Premium user
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,
    /// [IETF language tag][1] of the user's language
    ///
    /// [1]: https://en.wikipedia.org/wiki/IETF_language_tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl User {
    /// Creates a new User
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier
    /// * first_name - First name
    /// * is_bot - Whether the user is a bot
    pub fn new<T>(id: Integer, first_name: T, is_bot: bool) -> Self
    where
        T: Into<String>,
    {
        Self {
            first_name: first_name.into(),
            id,
            is_bot,
            added_to_attachment_menu: None,
            is_premium: None,
            language_code: None,
            last_name: None,
            username: None,
        }
    }

    /// Returns a full name of the user (first name + last name)
    pub fn get_full_name(&self) -> String {
        let mut full_name = self.first_name.clone();
        if let Some(ref last_name) = self.last_name {
            full_name.push(' ');
            full_name += last_name;
        }
        full_name
    }

    /// Returns a link to the user (`tg://user?id=xxx`)
    ///
    /// These links will work only if they are used inside an inline link.
    /// For example, they will not work,
    /// when used in an inline keyboard button or in a message text.
    pub fn get_link(&self) -> String {
        format!("tg://user?id={}", self.id)
    }

    /// Returns a mention for the user
    ///
    /// These mentions are only guaranteed to work if the user has contacted the bot in the past,
    /// has sent a callback query to the bot via inline button or is a member
    /// in the group where he was mentioned.
    pub fn get_link_mention(&self, parse_mode: ParseMode) -> Result<String, MentionError> {
        let full_name = parse_mode.escape(self.get_full_name());
        let user_link = self.get_link();
        Ok(match parse_mode {
            ParseMode::Markdown => return Err(MentionError::UnsupportedParseMode(parse_mode)),
            ParseMode::MarkdownV2 => format!(r#"[{}]({})"#, full_name, user_link),
            ParseMode::Html => format!(r#"<a href="{}">{}</a>"#, user_link, full_name),
        })
    }

    /// Sets a new value for the `added_to_attachment_menu` flag
    ///
    /// # Arguments
    ///
    /// * value - Value of the flag
    pub fn with_added_to_attachment_menu(mut self, value: bool) -> Self {
        self.added_to_attachment_menu = Some(value);
        self
    }

    /// Sets a new value for the `is_premium` flag
    ///
    /// # Arguments
    ///
    /// * value - Value of the flag
    pub fn with_is_premium(mut self, value: bool) -> Self {
        self.is_premium = Some(value);
        self
    }

    /// Sets a new language code
    ///
    /// # Arguments
    ///
    /// * value - Language code
    pub fn with_language_code<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.language_code = Some(value.into());
        self
    }

    /// Sets a new last name
    ///
    /// # Arguments
    ///
    /// * value - Last name
    pub fn with_last_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.last_name = Some(value.into());
        self
    }

    /// Sets a new username
    ///
    /// # Arguments
    ///
    /// * value - Username
    pub fn with_username<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.username = Some(value.into());
        self
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

/// Represents a list of profile pictures of a user
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct UserProfilePhotos {
    /// Requested profile pictures (in up to 4 sizes each)
    pub photos: Vec<Vec<PhotoSize>>,
    /// Total number of profile pictures the target user has
    pub total_count: Integer,
}

impl UserProfilePhotos {
    /// Creates a new UserProfilePhotos
    ///
    /// # Arguments
    ///
    /// * photos - A list of photos
    /// * total_count - Total number of photos
    pub fn new<A, B>(photos: A, total_count: Integer) -> Self
    where
        A: IntoIterator<Item = B>,
        B: IntoIterator<Item = PhotoSize>,
    {
        Self {
            photos: photos.into_iter().map(|x| x.into_iter().collect()).collect(),
            total_count,
        }
    }
}

/// Represents a user ID
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

/// Returns a list of profile pictures for a user
#[derive(Clone, Debug, Serialize)]
pub struct GetUserProfilePhotos {
    user_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<Integer>,
}

impl GetUserProfilePhotos {
    /// Creates a new GetUserProfilePhotos
    ///
    /// # Arguments
    ///
    /// * user_id - Unique identifier of the target user
    pub fn new(user_id: Integer) -> Self {
        GetUserProfilePhotos {
            user_id,
            offset: None,
            limit: None,
        }
    }

    /// Sets a new limit
    ///
    /// # Arguments
    ///
    /// * value - Limit of the number of photos to be retrieved; 1—100; default - 100
    pub fn with_limit(mut self, limit: Integer) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets a new offset
    ///
    /// # Arguments
    ///
    /// * value - Sequential number of the first photo to be returned
    ///
    /// By default, all photos are returned
    pub fn with_offset(mut self, offset: Integer) -> Self {
        self.offset = Some(offset);
        self
    }
}

impl Method for GetUserProfilePhotos {
    type Response = UserProfilePhotos;

    fn into_payload(self) -> Payload {
        Payload::json("getUserProfilePhotos", self)
    }
}
