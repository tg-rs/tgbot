use crate::types::{primitive::Integer, user::User};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct RawTextEntity {
    pub(super) offset: Integer,
    pub(super) length: Integer,
    #[serde(flatten)]
    pub(super) kind: RawTextEntityKind,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub(super) enum RawTextEntityKind {
    Bold,
    BotCommand,
    Cashtag,
    Code,
    Email,
    Hashtag,
    Italic,
    Mention,
    PhoneNumber,
    Pre { language: Option<String> },
    Strikethrough,
    TextLink { url: Option<String> },
    TextMention { user: Option<User> },
    Underline,
    Url,
}
