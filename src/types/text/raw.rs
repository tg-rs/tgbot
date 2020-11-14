use crate::types::{primitive::Integer, user::User};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct RawTextEntity {
    #[serde(rename = "type")]
    pub(super) kind: RawTextEntityKind,
    pub(super) offset: Integer,
    pub(super) length: Integer,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) user: Option<User>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) language: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
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
    Pre,
    Strikethrough,
    TextLink,
    TextMention,
    Underline,
    Url,
}
