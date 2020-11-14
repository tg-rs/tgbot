use crate::types::{primitive::Integer, user::User};
use serde::{Serialize, Serializer};
use std::convert::TryFrom;

use super::{
    error::TextEntityError,
    raw::{RawTextEntity, RawTextEntityKind},
};

/// Respresents an entity in a text
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum TextEntity {
    /// Bold text
    Bold(TextEntityPosition),
    /// Bot command
    BotCommand(TextEntityPosition),
    /// Cashtag
    Cashtag(TextEntityPosition),
    /// Monowidth string
    Code(TextEntityPosition),
    /// E-Mail
    Email(TextEntityPosition),
    /// Hashtag
    Hashtag(TextEntityPosition),
    /// Italic text
    Italic(TextEntityPosition),
    /// User mention (e.g. @username)
    Mention(TextEntityPosition),
    /// Phone number
    PhoneNumber(TextEntityPosition),
    /// Monowidth block
    Pre {
        /// Position of entity in text
        position: TextEntityPosition,
        /// Name of the programming language
        language: Option<String>,
    },
    /// Strikethrough text
    Strikethrough(TextEntityPosition),
    /// Clickable text URLs
    TextLink {
        /// Position of entity in text
        position: TextEntityPosition,
        /// URL that will be opened after user taps on the text
        url: String,
    },
    /// Mention user without username
    TextMention {
        /// Position of entity in text
        position: TextEntityPosition,
        /// Mentioned user
        user: User,
    },
    /// Underlined text
    Underline(TextEntityPosition),
    /// URL
    Url(TextEntityPosition),
}

macro_rules! text_entity_factory {
    ($($method_name:ident => $enum_variant: ident),*) => {
        $(
            /// Creates a new TextEntity
            ///
            /// # Arguments
            ///
            /// * offset - Offset in UTF-16 code units to the start of the entity
            /// * length - Length of the entity in UTF-16 code units
            pub fn $method_name(offset: Integer, length: Integer) -> TextEntity {
                TextEntity::$enum_variant(TextEntityPosition { offset, length })
            }
        )*
    };
}

impl TextEntity {
    text_entity_factory!(
        bold => Bold,
        bot_command => BotCommand,
        cashtag => Cashtag,
        code => Code,
        email => Email,
        hashtag => Hashtag,
        italic => Italic,
        mention => Mention,
        phone_number => PhoneNumber,
        strikethrough => Strikethrough,
        underline => Underline
    );

    /// Creates a new TextEntity
    ///
    /// # Arguments
    ///
    /// * offset - Offset in UTF-16 code units to the start of the entity
    /// * length - Length of the entity in UTF-16 code units
    /// * language - The programming language of the entity text
    pub fn pre<L: Into<String>>(offset: Integer, length: Integer, language: Option<L>) -> TextEntity {
        TextEntity::Pre {
            position: TextEntityPosition { offset, length },
            language: language.map(|x| x.into()),
        }
    }

    /// Creates a new TextEntity
    ///
    /// # Arguments
    ///
    /// * offset - Offset in UTF-16 code units to the start of the entity
    /// * length - Length of the entity in UTF-16 code units
    /// * url - URL that will be opened after user taps on the text
    pub fn text_link<U: Into<String>>(offset: Integer, length: Integer, url: U) -> TextEntity {
        TextEntity::TextLink {
            position: TextEntityPosition { offset, length },
            url: url.into(),
        }
    }

    /// Creates a new TextEntity
    ///
    /// # Arguments
    ///
    /// * offset - Offset in UTF-16 code units to the start of the entity
    /// * length - Length of the entity in UTF-16 code units
    /// * user - User to be mentioned
    pub fn text_mention(offset: Integer, length: Integer, user: User) -> TextEntity {
        TextEntity::TextMention {
            position: TextEntityPosition { offset, length },
            user,
        }
    }
}

impl TryFrom<RawTextEntity> for TextEntity {
    type Error = TextEntityError;

    fn try_from(raw: RawTextEntity) -> Result<Self, Self::Error> {
        let position = TextEntityPosition {
            offset: raw.offset,
            length: raw.length,
        };
        Ok(match raw.kind {
            RawTextEntityKind::Bold => TextEntity::Bold(position),
            RawTextEntityKind::BotCommand => TextEntity::BotCommand(position),
            RawTextEntityKind::Cashtag => TextEntity::Cashtag(position),
            RawTextEntityKind::Code => TextEntity::Code(position),
            RawTextEntityKind::Email => TextEntity::Email(position),
            RawTextEntityKind::Hashtag => TextEntity::Hashtag(position),
            RawTextEntityKind::Italic => TextEntity::Italic(position),
            RawTextEntityKind::Mention => TextEntity::Mention(position),
            RawTextEntityKind::PhoneNumber => TextEntity::PhoneNumber(position),
            RawTextEntityKind::Pre => TextEntity::Pre {
                position,
                language: raw.language,
            },
            RawTextEntityKind::Strikethrough => TextEntity::Strikethrough(position),
            RawTextEntityKind::TextLink => match raw.url {
                Some(url) => TextEntity::TextLink { position, url },
                None => return Err(TextEntityError::NoUrl),
            },
            RawTextEntityKind::TextMention => match raw.user {
                Some(user) => TextEntity::TextMention { position, user },
                None => return Err(TextEntityError::NoUser),
            },
            RawTextEntityKind::Underline => TextEntity::Underline(position),
            RawTextEntityKind::Url => TextEntity::Url(position),
        })
    }
}

impl From<TextEntity> for RawTextEntity {
    fn from(entity: TextEntity) -> Self {
        macro_rules! raw {
            ($kind:ident($position:ident)) => {
                raw!($kind($position, url = None, user = None, language = None))
            };
            ($kind:ident($position:ident, url=$url:expr)) => {
                raw!($kind($position, url = Some($url), user = None, language = None))
            };
            ($kind:ident($position:ident, user=$user:expr)) => {
                raw!($kind($position, url = None, user = Some($user), language = None))
            };
            ($kind:ident($position:ident, language=$language:expr)) => {
                raw!($kind($position, url = None, user = None, language = $language))
            };
            ($kind:ident(
                $position:ident,
                url=$url:expr,
                user=$user:expr,
                language=$language:expr
            )) => {
                RawTextEntity {
                    kind: RawTextEntityKind::$kind,
                    offset: $position.offset,
                    length: $position.length,
                    url: $url,
                    user: $user,
                    language: $language,
                }
            };
        }
        match entity {
            TextEntity::Bold(p) => raw!(Bold(p)),
            TextEntity::BotCommand(p) => raw!(BotCommand(p)),
            TextEntity::Cashtag(p) => raw!(Cashtag(p)),
            TextEntity::Code(p) => raw!(Code(p)),
            TextEntity::Email(p) => raw!(Email(p)),
            TextEntity::Hashtag(p) => raw!(Hashtag(p)),
            TextEntity::Italic(p) => raw!(Italic(p)),
            TextEntity::Mention(p) => raw!(Mention(p)),
            TextEntity::PhoneNumber(p) => raw!(PhoneNumber(p)),
            TextEntity::Pre {
                position: p,
                language: l,
            } => raw!(Pre(p, language = l)),
            TextEntity::Strikethrough(p) => raw!(Strikethrough(p)),
            TextEntity::TextLink { position: p, url: u } => raw!(TextLink(p, url = u)),
            TextEntity::TextMention { position: p, user: u } => raw!(TextMention(p, user = u)),
            TextEntity::Underline(p) => raw!(Underline(p)),
            TextEntity::Url(p) => raw!(Url(p)),
        }
    }
}

impl Serialize for TextEntity {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        RawTextEntity::from(self.clone()).serialize(serializer)
    }
}

/// Describes a bot command found in text
///
/// Use TextEntity::BotCommand in order to get command position
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TextEntityBotCommand {
    /// Actual command
    pub command: String,
    /// Bot's username
    pub bot_name: Option<String>,
}

/// Describes position of entity in text
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct TextEntityPosition {
    /// Offset in UTF-16 code units to the start of the entity
    pub offset: Integer,
    /// Length of the entity in UTF-16 code units
    pub length: Integer,
}

/// Converts raw text entities to user-friendly representation
///
/// # Arguments
///
/// * raw - List of raw entities
/// * text_len - Length of the related text in UTF-16
pub(super) fn convert_entities(raw: Vec<RawTextEntity>, text_len: i64) -> Result<Vec<TextEntity>, TextEntityError> {
    let mut result = Vec::new();
    for raw_entity in raw {
        let (offset, length) = (raw_entity.offset, raw_entity.length);
        if offset > text_len || offset < 0 {
            return Err(TextEntityError::BadOffset(offset));
        }
        let limit = offset + length;
        if limit > text_len || limit < 0 {
            return Err(TextEntityError::BadLength(length));
        }
        result.push(TextEntity::try_from(raw_entity)?)
    }
    Ok(result)
}

pub(crate) fn serialize_text_entities(entities: &[TextEntity]) -> Result<String, TextEntityError> {
    serde_json::to_string(entities).map_err(TextEntityError::Serialize)
}
