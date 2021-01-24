use crate::types::user::User;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct RawTextEntity {
    pub(super) offset: u32,
    pub(super) length: u32,
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
    Pre {
        #[serde(skip_serializing_if = "Option::is_none")]
        language: Option<String>,
    },
    Strikethrough,
    TextLink {
        #[serde(skip_serializing_if = "Option::is_none")]
        url: Option<String>,
    },
    TextMention {
        #[serde(skip_serializing_if = "Option::is_none")]
        user: Option<User>,
    },
    Underline,
    Url,
}
