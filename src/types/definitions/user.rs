use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{Audio, Integer, ParseMode, PhotoSize},
};

/// Represents the date of birth of a user.
#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Birthdate {
    /// Day of the user's birth; 1-31
    pub day: Integer,
    /// Month of the user's birth; 1-12
    pub month: Integer,
    /// Year of the user's birth
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
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct SharedUser {
    /// Identifier of the shared user.
    ///
    /// The bot may not have access to the user and could be unable to use this identifier,
    /// unless the user is already known to the bot by some other means.
    pub user_id: Integer,
    /// First name of the user, if the name was requested by the bot.
    pub first_name: Option<String>,
    /// Last name of the user, if the name was requested by the bot.
    pub last_name: Option<String>,
    /// Available sizes of the user photo, if the photo was requested by the bot
    pub photo: Option<Vec<PhotoSize>>,
    /// Username of the user, if the username was requested by the bot.
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
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct User {
    /// First name of the user.
    pub first_name: String,
    /// Unique identifier of the user.
    pub id: UserPeerId,
    /// Indicates whether the user is a bot.
    pub is_bot: bool,
    /// Indicates whether the user added the bot to the attachment menu.
    pub added_to_attachment_menu: Option<bool>,
    /// Indicates whether the user is a Telegram Premium user.
    pub is_premium: Option<bool>,
    /// [IETF language tag][1] of the user's language.
    ///
    /// [1]: https://en.wikipedia.org/wiki/IETF_language_tag
    pub language_code: Option<String>,
    /// Last name of the user.
    pub last_name: Option<String>,
    /// Username of the user.
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
            ParseMode::MarkdownV2 => format!(r#"[{full_name}]({user_link})"#),
            ParseMode::Html => format!(r#"<a href="{user_link}">{full_name}</a>"#),
        })
    }

    /// Sets a new value for the `added_to_attachment_menu` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the user added the bot to the attachment menu.
    pub fn with_added_to_attachment_menu(mut self, value: bool) -> Self {
        self.added_to_attachment_menu = Some(value);
        self
    }

    /// Sets a new value for the `is_premium` flag.
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
                write!(out, "can not mention with {parse_mode} parse mode")
            }
        }
    }
}

/// Represents the audios displayed on a user's profile.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserProfileAudios {
    /// Requested profile audios.
    pub audios: Vec<Audio>,
    /// Total number of profile audios for the target user.
    pub total_count: Integer,
}

impl UserProfileAudios {
    /// Creates a new `UserProfileAudios`.
    ///
    /// # Arguments
    ///
    /// * `audios` - A list of audios.
    /// * `total_count` - Total number of audios.
    pub fn new<T>(audios: T, total_count: Integer) -> Self
    where
        T: IntoIterator<Item = Audio>,
    {
        Self {
            audios: audios.into_iter().collect(),
            total_count,
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

/// Describes the rating of a user based on their Telegram Star spendings.
#[derive(Copy, Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct UserRating {
    /// The rating value required to get the current level.
    pub current_level_rating: Integer,
    /// Current level of the user, indicating their reliability when purchasing digital goods and services.
    ///
    /// A higher level suggests a more trustworthy customer; a negative level is likely reason for concern.
    pub level: Integer,
    /// Numerical value of the user's rating; the higher the rating, the better.
    pub rating: Integer,
    /// The rating value required to get to the next level; omitted if the maximum level was reached.
    pub next_level_rating: Option<Integer>,
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

/// Returns a list of profile audios for a user.
#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug, Serialize)]
pub struct GetUserProfileAudios {
    user_id: Integer,
    limit: Option<Integer>,
    offset: Option<Integer>,
}

impl GetUserProfileAudios {
    /// Creates a new `GetUserProfileAudios`.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Unique identifier of the target user.
    pub fn new(user_id: Integer) -> Self {
        Self {
            user_id,
            limit: None,
            offset: None,
        }
    }

    /// Sets a new limit.
    ///
    /// # Arguments
    ///
    /// * `value` - Sequential number of the first audio to be returned.
    ///
    /// By default, all audios are returned.
    pub fn with_limit(mut self, value: Integer) -> Self {
        self.limit = Some(value);
        self
    }

    /// Sets a new offset
    ///
    /// # Arguments
    ///
    /// * `value` - Limits the number of audios to be retrieved.
    ///
    /// Values between 1-100 are accepted. Defaults to 100.
    pub fn with_offset(mut self, value: Integer) -> Self {
        self.offset = Some(value);
        self
    }
}

impl Method for GetUserProfileAudios {
    type Response = UserProfileAudios;

    fn into_payload(self) -> Payload {
        Payload::json("getUserProfileAudios", self)
    }
}

/// Returns a list of profile pictures for a user.
#[serde_with::skip_serializing_none]
#[derive(Clone, Copy, Debug, Serialize)]
pub struct GetUserProfilePhotos {
    user_id: Integer,
    limit: Option<Integer>,
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

/// Changes the emoji status for a given user
/// that previously allowed the bot to manage their emoji status
/// via the Mini App method `requestEmojiStatusAccess`.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Serialize)]
pub struct SetUserEmojiStatus {
    user_id: Integer,
    emoji_status_custom_emoji_id: Option<String>,
    emoji_status_expiration_date: Option<Integer>,
}

impl SetUserEmojiStatus {
    /// Creates a new `SetUserEmojiStatus`.
    ///
    /// # Arguments
    ///
    /// * `user_id` - Unique identifier of the target user
    pub fn new(user_id: Integer) -> Self {
        Self {
            user_id,
            emoji_status_custom_emoji_id: None,
            emoji_status_expiration_date: None,
        }
    }

    /// Sets a new emoji ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Custom emoji identifier of the emoji status to set.
    ///   Pass an empty string to remove the status.
    pub fn with_emoji_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.emoji_status_custom_emoji_id = Some(value.into());
        self
    }

    /// Sets a new expiration date.
    ///
    /// # Arguments
    ///
    /// * `value` - Expiration date of the emoji status, if any.
    pub fn with_expiration_date(mut self, value: Integer) -> Self {
        self.emoji_status_expiration_date = Some(value);
        self
    }
}

impl Method for SetUserEmojiStatus {
    type Response = bool;

    fn into_payload(self) -> Payload {
        Payload::json("setUserEmojiStatus", self)
    }
}
