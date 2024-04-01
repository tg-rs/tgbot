use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{Integer, ParseMode, PhotoSize},
};

#[cfg(test)]
mod tests;

/// Represents the date of birth of a user.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Birthdate {
    /// Day of the user's birth; 1-31
    pub day: Integer,
    /// Month of the user's birth; 1-12
    pub month: Integer,
    /// Year of the user's birth
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<Integer>,
}

impl Birthdate {
    /// Creates a new `Birthdate`.
    ///
    /// # Arguments
    ///
    /// * `day` - Day.
    /// * `month` - Month.
    pub fn new(day: Integer, month: Integer) -> Self {
        Self { day, month, year: None }
    }

    /// Sets a new year.
    ///
    /// # Arguments
    ///
    /// * `year` - Year.
    pub fn with_year(mut self, value: Integer) -> Self {
        self.year = Some(value);
        self
    }
}

/// Contains information about a user that
/// was shared with the bot using a [`crate::types::KeyboardButtonRequestUsers`] button.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct SharedUser {
    /// Identifier of the shared user.
    ///
    /// The bot may not have access to the user and could be unable to use this identifier,
    /// unless the user is already known to the bot by some other means.
    pub user_id: Integer,
    /// First name of the user, if the name was requested by the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Last name of the user, if the name was requested by the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Available sizes of the user photo, if the photo was requested by the bot
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo: Option<Vec<PhotoSize>>,
    /// Username of the user, if the username was requested by the bot.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl SharedUser {
    /// Creates a new `SharedUser`.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Identifier of the shared user.
    pub fn new(user_id: Integer) -> Self {
        Self {
            user_id,
            first_name: None,
            last_name: None,
            photo: None,
            username: None,
        }
    }

    /// Sets a new first name.
    ///
    /// # Arguments
    ///
    /// * `value` - First name.
    pub fn with_first_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.first_name = Some(value.into());
        self
    }

    /// Sets a new last name.
    ///
    /// # Arguments
    ///
    /// * `value` - Last name.
    pub fn with_last_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.last_name = Some(value.into());
        self
    }

    /// Sets new photo sizes.
    ///
    /// # Arguments
    ///
    /// * `value` - Available sizes of photo.
    pub fn with_photo<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = PhotoSize>,
    {
        self.photo = Some(value.into_iter().collect());
        self
    }

    /// Sets a new username.
    ///
    /// # Arguments
    ///
    /// * `value` - Username.
    pub fn with_username<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.username = Some(value.into());
        self
    }
}

/// Represents a user.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct User {
    /// First name of the user.
    pub first_name: String,
    /// Unique identifier of the user.
    pub id: UserPeerId,
    /// Indicates whether the user is a bot.
    pub is_bot: bool,
    /// Indicates whether the user added the bot to the attachment menu.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added_to_attachment_menu: Option<bool>,
    /// Indicates whether the user is a Telegram Premium user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,
    /// [IETF language tag][1] of the user's language.
    ///
    /// [1]: https://en.wikipedia.org/wiki/IETF_language_tag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
    /// Last name of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Username of the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<UserUsername>,
}

impl User {
    /// Creates a new `User`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the user.
    /// * `first_name` - First name of the user.
    /// * `is_bot` - Indicates whether the user is a bot.
    pub fn new<A, B>(id: A, first_name: B, is_bot: bool) -> Self
    where
        A: Into<UserPeerId>,
        B: Into<String>,
    {
        Self {
            first_name: first_name.into(),
            id: id.into(),
            is_bot,
            added_to_attachment_menu: None,
            is_premium: None,
            language_code: None,
            last_name: None,
            username: None,
        }
    }

    /// Returns the full name of the user (first name + last name).
    pub fn get_full_name(&self) -> String {
        let mut full_name = self.first_name.clone();
        if let Some(ref last_name) = self.last_name {
            full_name.push(' ');
            full_name += last_name;
        }
        full_name
    }

    /// Returns the link to the user (`tg://user?id=xxx`).
    ///
    /// These links will work only if they are used inside an inline link.
    /// For example, they will not work,
    /// when used in an inline keyboard button or in a message text.
    pub fn get_link(&self) -> String {
        format!("tg://user?id={}", self.id.0)
    }

    /// Returns the mention for the user.
    ///
    /// # Arguments
    ///
    /// * `parse_mode` - A parse mode for formatting the mention.
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

    /// Sets a new value for an `added_to_attachment_menu` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user added the bot to the attachment menu.
    pub fn with_added_to_attachment_menu(mut self, value: bool) -> Self {
        self.added_to_attachment_menu = Some(value);
        self
    }

    /// Sets a new value for an `is_premium` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user is a Telegram Premium user.
    pub fn with_is_premium(mut self, value: bool) -> Self {
        self.is_premium = Some(value);
        self
    }

    /// Sets a new language code.
    ///
    /// # Arguments
    ///
    /// * `value` - Language code.
    pub fn with_language_code<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.language_code = Some(value.into());
        self
    }

    /// Sets a new last name.
    ///
    /// # Arguments
    ///
    /// * `value` - Last name.
    pub fn with_last_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.last_name = Some(value.into());
        self
    }

    /// Sets a new username.
    ///
    /// # Arguments
    ///
    /// * `value` - Username.
    pub fn with_username<T>(mut self, value: T) -> Self
    where
        T: Into<UserUsername>,
    {
        self.username = Some(value.into());
        self
    }
}

/// Represents an error occurred when getting user mention.
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

/// Represents a list of profile pictures of a user.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct UserProfilePhotos {
    /// Requested profile pictures (in up to 4 sizes each).
    pub photos: Vec<Vec<PhotoSize>>,
    /// Total number of profile pictures the target user has.
    pub total_count: Integer,
}

impl UserProfilePhotos {
    /// Creates a new `UserProfilePhotos`.
    ///
    /// # Arguments
    ///
    /// * `photos` - A list of photos.
    /// * `total_count` - Total number of photos.
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

/// ID of a user.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(from = "Integer", into = "Integer")]
pub struct UserPeerId(Integer);

impl From<Integer> for UserPeerId {
    fn from(value: Integer) -> Self {
        Self(value)
    }
}

impl From<UserPeerId> for Integer {
    fn from(value: UserPeerId) -> Self {
        value.0
    }
}

impl fmt::Display for UserPeerId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<Integer> for UserPeerId {
    fn eq(&self, other: &Integer) -> bool {
        self.0.eq(other)
    }
}

/// Username of a user in the format `@username`.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(from = "String", into = "String")]
pub struct UserUsername(String);

impl From<&str> for UserUsername {
    fn from(value: &str) -> Self {
        Self(String::from(value))
    }
}

impl From<String> for UserUsername {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<UserUsername> for String {
    fn from(value: UserUsername) -> Self {
        value.0
    }
}

impl fmt::Display for UserUsername {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl PartialEq<String> for UserUsername {
    fn eq(&self, other: &String) -> bool {
        self.0.eq(other)
    }
}

impl PartialEq<str> for UserUsername {
    fn eq(&self, other: &str) -> bool {
        self.0.eq(other)
    }
}

/// Represents a user ID.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum UserId {
    /// ID of a user.
    Id(UserPeerId),
    /// @username of a user.
    Username(UserUsername),
}

impl From<&str> for UserId {
    fn from(username: &str) -> UserId {
        UserId::Username(String::from(username).into())
    }
}

impl From<String> for UserId {
    fn from(username: String) -> UserId {
        UserId::Username(username.into())
    }
}

impl From<Integer> for UserId {
    fn from(id: Integer) -> UserId {
        UserId::Id(id.into())
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UserId::Id(chat_id) => write!(out, "{}", chat_id.0),
            UserId::Username(username) => write!(out, "{}", username.0),
        }
    }
}

impl From<UserPeerId> for UserId {
    fn from(value: UserPeerId) -> Self {
        UserId::Id(value)
    }
}

impl From<UserUsername> for UserId {
    fn from(value: UserUsername) -> Self {
        UserId::Username(value)
    }
}

/// Returns a list of profile pictures for a user.
#[derive(Clone, Debug, Serialize)]
pub struct GetUserProfilePhotos {
    user_id: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offset: Option<Integer>,
}

impl GetUserProfilePhotos {
    /// Creates a new `GetUserProfilePhotos`.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Unique identifier of the target user.
    pub fn new(user_id: Integer) -> Self {
        GetUserProfilePhotos {
            user_id,
            offset: None,
            limit: None,
        }
    }

    /// Sets a new limit.
    ///
    /// # Arguments
    ///
    /// * `value` - Limit of the number of photos to be retrieved; 1—100; default - 100.
    pub fn with_limit(mut self, limit: Integer) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets a new offset.
    ///
    /// # Arguments
    ///
    /// * `value` - Sequential number of the first photo to be returned.
    ///
    /// By default, all photos are returned.
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
