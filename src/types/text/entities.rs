use std::{
    convert::TryFrom,
    ops::{Index, IndexMut, Range},
};

use serde::{Deserialize, Serialize};

use crate::types::{text::raw::RawTextEntityKind, user::User};

use super::{error::TextEntityError, raw::RawTextEntity};

/// A collection of text entities
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(into = "Vec<TextEntity>", try_from = "Vec<RawTextEntity>")]
pub struct TextEntities {
    items: Vec<TextEntity>,
}

impl TextEntities {
    /// Pushes a new item into the collection
    pub fn push(&mut self, item: TextEntity) {
        self.items.push(item);
    }

    /// Serializes text entities into JSON string
    pub fn serialize(&self) -> Result<String, TextEntityError> {
        serde_json::to_string(self).map_err(TextEntityError::Serialize)
    }
}

impl TryFrom<Vec<RawTextEntity>> for TextEntities {
    type Error = TextEntityError;

    fn try_from(entities: Vec<RawTextEntity>) -> Result<Self, Self::Error> {
        entities
            .into_iter()
            .map(TryFrom::try_from)
            .collect::<Result<Vec<TextEntity>, _>>()
            .map(|items| Self { items })
    }
}

impl FromIterator<TextEntity> for TextEntities {
    fn from_iter<T: IntoIterator<Item = TextEntity>>(iter: T) -> Self {
        Self {
            items: iter.into_iter().collect(),
        }
    }
}

impl IntoIterator for TextEntities {
    type Item = TextEntity;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<'a> IntoIterator for &'a TextEntities {
    type Item = &'a TextEntity;
    type IntoIter = std::slice::Iter<'a, TextEntity>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.as_slice().iter()
    }
}

impl<'a> IntoIterator for &'a mut TextEntities {
    type Item = &'a mut TextEntity;
    type IntoIter = std::slice::IterMut<'a, TextEntity>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.as_mut_slice().iter_mut()
    }
}

impl Index<usize> for TextEntities {
    type Output = TextEntity;

    fn index(&self, index: usize) -> &Self::Output {
        &self.items[index]
    }
}

impl IndexMut<usize> for TextEntities {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.items[index]
    }
}

impl From<TextEntities> for Vec<TextEntity> {
    fn from(entities: TextEntities) -> Self {
        entities.items
    }
}

/// Represents an entity in a text
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(try_from = "RawTextEntity", into = "RawTextEntity")]
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
    /// Spoiler message
    Spoiler(TextEntityPosition),
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
            /// * pos - position of TextEntity in UTF-16 code units
            pub fn $method_name<T: Into<TextEntityPosition>>(pos: T) -> TextEntity {
                TextEntity::$enum_variant(pos.into())
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
        spoiler => Spoiler,
        strikethrough => Strikethrough,
        underline => Underline
    );

    /// Creates a new TextEntity
    ///
    /// # Arguments
    ///
    /// * pos - position of TextEntity in UTF-16 code units
    /// * language - The programming language of the entity text
    pub fn pre<P: Into<TextEntityPosition>, L: Into<String>>(pos: P, language: Option<L>) -> TextEntity {
        TextEntity::Pre {
            position: pos.into(),
            language: language.map(|x| x.into()),
        }
    }

    /// Creates a new TextEntity
    ///
    /// # Arguments
    ///
    /// * pos - position of TextEntity in UTF-16 code units
    /// * url - URL that will be opened after user taps on the text
    pub fn text_link<P: Into<TextEntityPosition>, U: Into<String>>(pos: P, url: U) -> TextEntity {
        TextEntity::TextLink {
            position: pos.into(),
            url: url.into(),
        }
    }

    /// Creates a new TextEntity
    ///
    /// # Arguments
    ///
    /// * pos - position of TextEntity in UTF-16 code units
    /// * user - User to be mentioned
    pub fn text_mention<P: Into<TextEntityPosition>>(pos: P, user: User) -> TextEntity {
        TextEntity::TextMention {
            position: pos.into(),
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
            RawTextEntityKind::Pre { language } => TextEntity::Pre { position, language },
            RawTextEntityKind::Spoiler => TextEntity::Spoiler(position),
            RawTextEntityKind::Strikethrough => TextEntity::Strikethrough(position),
            RawTextEntityKind::TextLink { url } => TextEntity::TextLink {
                position,
                url: url.ok_or(TextEntityError::NoUrl)?,
            },
            RawTextEntityKind::TextMention { user } => TextEntity::TextMention {
                position,
                user: user.ok_or(TextEntityError::NoUser)?,
            },
            RawTextEntityKind::Underline => TextEntity::Underline(position),
            RawTextEntityKind::Url => TextEntity::Url(position),
        })
    }
}

impl From<TextEntity> for RawTextEntity {
    fn from(entity: TextEntity) -> Self {
        macro_rules! raw {
            ($kind:ident($position:ident $( , $item:ident )?)) => {
                RawTextEntity {
                    kind: RawTextEntityKind::$kind $( { $item: $item.into() } )?,
                    offset: $position.offset as _,
                    length: $position.length as _,
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
            TextEntity::Pre { position: p, language } => raw!(Pre(p, language)),
            TextEntity::Spoiler(p) => raw!(Spoiler(p)),
            TextEntity::Strikethrough(p) => raw!(Strikethrough(p)),
            TextEntity::TextLink { position: p, url } => raw!(TextLink(p, url)),
            TextEntity::TextMention { position: p, user } => raw!(TextMention(p, user)),
            TextEntity::Underline(p) => raw!(Underline(p)),
            TextEntity::Url(p) => raw!(Url(p)),
        }
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
    pub offset: u32,
    /// Length of the entity in UTF-16 code units
    pub length: u32,
}

impl From<Range<u32>> for TextEntityPosition {
    fn from(range: Range<u32>) -> Self {
        Self {
            offset: range.start,
            length: range.end - range.start,
        }
    }
}