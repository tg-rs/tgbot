use std::{
    convert::TryFrom,
    error::Error,
    fmt,
    ops::{Index, IndexMut, Range},
};

use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;

use crate::types::User;

/// Represents a collection of text entities.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(into = "Vec<TextEntity>", try_from = "Vec<RawTextEntity>")]
pub struct TextEntities {
    items: Vec<TextEntity>,
}

impl TextEntities {
    /// Pushes a new entity into the collection.
    ///
    /// # Arguments
    ///
    /// * `value` - The entity to push.
    pub fn push(&mut self, value: TextEntity) {
        self.items.push(value);
    }

    /// Serializes text entities into a JSON string.
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

/// Represents an entity in a text.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(try_from = "RawTextEntity", into = "RawTextEntity")]
pub enum TextEntity {
    /// A block quotation.
    Blockquote(TextEntityPosition),
    /// A bold text.
    Bold(TextEntityPosition),
    /// A bot command.
    BotCommand(TextEntityPosition),
    /// A cashtag.
    Cashtag(TextEntityPosition),
    /// A monospace string.
    Code(TextEntityPosition),
    /// An inline custom emoji sticker.
    CustomEmoji {
        /// Unique identifier of the custom emoji.
        ///
        /// Use [`crate::types::GetCustomEmojiStickers`] to get full information about the sticker.
        custom_emoji_id: String,
        /// Position of entity in text.
        position: TextEntityPosition,
    },
    /// An E-Mail.
    Email(TextEntityPosition),
    /// Collapsed-by-default block quotation.
    ExpandableBlockquote(TextEntityPosition),
    /// A hashtag.
    Hashtag(TextEntityPosition),
    /// An italic text.
    Italic(TextEntityPosition),
    /// A user mention (e.g. @username).
    Mention(TextEntityPosition),
    /// A phone number.
    PhoneNumber(TextEntityPosition),
    /// A monospace block.
    Pre {
        /// The position of the entity in the text.
        position: TextEntityPosition,
        /// The name of the programming language.
        language: Option<String>,
    },
    /// A spoiler message.
    Spoiler(TextEntityPosition),
    /// A strikethrough text.
    Strikethrough(TextEntityPosition),
    /// A clickable text URLs.
    TextLink {
        /// The position of the entity in the text.
        position: TextEntityPosition,
        /// URL that will be opened after user taps on the text.
        url: String,
    },
    /// A user mention without a username.
    TextMention {
        /// The position of the entity in the text.
        position: TextEntityPosition,
        /// The mentioned user.
        user: User,
    },
    /// An underlined text.
    Underline(TextEntityPosition),
    /// An URL.
    Url(TextEntityPosition),
}

macro_rules! text_entity_factory {
    ($($method_name:ident => $enum_variant: ident),*) => {
        $(
            /// Creates a new `TextEntity`.
            ///
            /// # Arguments
            ///
            /// * `pos` - Position of TextEntity in UTF-16 code units.
            pub fn $method_name<T: Into<TextEntityPosition>>(pos: T) -> TextEntity {
                TextEntity::$enum_variant(pos.into())
            }
        )*
    };
}

impl TextEntity {
    text_entity_factory!(
        blockquote => Blockquote,
        bold => Bold,
        bot_command => BotCommand,
        cashtag => Cashtag,
        code => Code,
        email => Email,
        expandable_blockquote => ExpandableBlockquote,
        hashtag => Hashtag,
        italic => Italic,
        mention => Mention,
        phone_number => PhoneNumber,
        spoiler => Spoiler,
        strikethrough => Strikethrough,
        underline => Underline
    );

    /// Creates a new `TextEntity`.
    ///
    /// # Arguments
    ///
    /// * `pos` - Position of the entity in UTF-16 code units.
    /// * `custom_emoji_id` - Unique identifier of the custom emoji.
    pub fn custom_emoji<P: Into<TextEntityPosition>, I: Into<String>>(pos: P, custom_emoji_id: I) -> TextEntity {
        TextEntity::CustomEmoji {
            position: pos.into(),
            custom_emoji_id: custom_emoji_id.into(),
        }
    }

    /// Creates a new `TextEntity`.
    ///
    /// # Arguments
    ///
    /// * `pos` - Position of the entity in UTF-16 code units.
    /// * `language` - The programming language of the entity text.
    pub fn pre<P: Into<TextEntityPosition>, L: Into<String>>(pos: P, language: Option<L>) -> TextEntity {
        TextEntity::Pre {
            position: pos.into(),
            language: language.map(|x| x.into()),
        }
    }

    /// Creates a new `TextEntity`.
    ///
    /// # Arguments
    ///
    /// * `pos` - The position of the entity in UTF-16 code units.
    /// * `url` - The URL that will be opened after user taps on the text.
    pub fn text_link<P: Into<TextEntityPosition>, U: Into<String>>(pos: P, url: U) -> TextEntity {
        TextEntity::TextLink {
            position: pos.into(),
            url: url.into(),
        }
    }

    /// Creates a new `TextEntity`.
    ///
    /// # Arguments
    ///
    /// * `pos` - The position of the entity in UTF-16 code units.
    /// * `user` - The user to be mentioned.
    pub fn text_mention<P: Into<TextEntityPosition>>(pos: P, user: User) -> TextEntity {
        TextEntity::TextMention {
            position: pos.into(),
            user,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RawTextEntity {
    offset: u32,
    length: u32,
    #[serde(flatten)]
    entity_type: RawTextEntityType,
}

#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum RawTextEntityType {
    Blockquote,
    Bold,
    BotCommand,
    Cashtag,
    Code,
    CustomEmoji { custom_emoji_id: Option<String> },
    Email,
    ExpandableBlockquote,
    Hashtag,
    Italic,
    Mention,
    PhoneNumber,
    Pre { language: Option<String> },
    Spoiler,
    Strikethrough,
    TextLink { url: Option<String> },
    TextMention { user: Option<User> },
    Underline,
    Url,
}

/// Represents an error when parsing/serializing entities.
#[derive(Debug)]
pub enum TextEntityError {
    /// Custom emoji is required for custom_emoji entity.
    NoCustomEmoji,
    /// URL is required for `text_link` entity.
    NoUrl,
    /// User is required for `text_mention` entity.
    NoUser,
    /// Failed to serialize entities.
    Serialize(JsonError),
}

impl Error for TextEntityError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Serialize(err) => Some(err),
            _ => None,
        }
    }
}

impl fmt::Display for TextEntityError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        use self::TextEntityError::*;
        write!(
            out,
            "{}",
            match self {
                NoCustomEmoji => String::from("Custom emoji is required for custom_emoji entity"),
                NoUrl => String::from("URL is required for text_link entity"),
                NoUser => String::from("user is required for text_mention entity"),
                Serialize(err) => format!("failed to serialize text entities: {err}"),
            }
        )
    }
}

impl TryFrom<RawTextEntity> for TextEntity {
    type Error = TextEntityError;

    fn try_from(raw: RawTextEntity) -> Result<Self, Self::Error> {
        let position = TextEntityPosition {
            offset: raw.offset,
            length: raw.length,
        };

        Ok(match raw.entity_type {
            RawTextEntityType::Blockquote => TextEntity::Blockquote(position),
            RawTextEntityType::Bold => TextEntity::Bold(position),
            RawTextEntityType::BotCommand => TextEntity::BotCommand(position),
            RawTextEntityType::Cashtag => TextEntity::Cashtag(position),
            RawTextEntityType::Code => TextEntity::Code(position),
            RawTextEntityType::CustomEmoji { custom_emoji_id } => TextEntity::CustomEmoji {
                position,
                custom_emoji_id: custom_emoji_id.ok_or(TextEntityError::NoCustomEmoji)?,
            },
            RawTextEntityType::Email => TextEntity::Email(position),
            RawTextEntityType::ExpandableBlockquote => TextEntity::ExpandableBlockquote(position),
            RawTextEntityType::Hashtag => TextEntity::Hashtag(position),
            RawTextEntityType::Italic => TextEntity::Italic(position),
            RawTextEntityType::Mention => TextEntity::Mention(position),
            RawTextEntityType::PhoneNumber => TextEntity::PhoneNumber(position),
            RawTextEntityType::Pre { language } => TextEntity::Pre { position, language },
            RawTextEntityType::Spoiler => TextEntity::Spoiler(position),
            RawTextEntityType::Strikethrough => TextEntity::Strikethrough(position),
            RawTextEntityType::TextLink { url } => TextEntity::TextLink {
                position,
                url: url.ok_or(TextEntityError::NoUrl)?,
            },
            RawTextEntityType::TextMention { user } => TextEntity::TextMention {
                position,
                user: user.ok_or(TextEntityError::NoUser)?,
            },
            RawTextEntityType::Underline => TextEntity::Underline(position),
            RawTextEntityType::Url => TextEntity::Url(position),
        })
    }
}

impl From<TextEntity> for RawTextEntity {
    fn from(entity: TextEntity) -> Self {
        macro_rules! raw {
            ($entity_type:ident($position:ident $( , $item:ident )?)) => {
                RawTextEntity {
                    entity_type: RawTextEntityType::$entity_type $( { $item: $item.into() } )?,
                    offset: $position.offset as _,
                    length: $position.length as _,
                }
            };
        }
        match entity {
            TextEntity::Blockquote(p) => raw!(Blockquote(p)),
            TextEntity::Bold(p) => raw!(Bold(p)),
            TextEntity::BotCommand(p) => raw!(BotCommand(p)),
            TextEntity::Cashtag(p) => raw!(Cashtag(p)),
            TextEntity::Code(p) => raw!(Code(p)),
            TextEntity::CustomEmoji {
                position: p,
                custom_emoji_id,
            } => raw!(CustomEmoji(p, custom_emoji_id)),
            TextEntity::Email(p) => raw!(Email(p)),
            TextEntity::ExpandableBlockquote(p) => raw!(ExpandableBlockquote(p)),
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

/// Represents a bot command found in text.
///
/// Use [`TextEntity::BotCommand`] to get a position of the command.
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct TextEntityBotCommand {
    /// Actual command.
    pub command: String,
    /// Username of a bot.
    pub bot_name: Option<String>,
}

/// Represents a position of an entity in a text.
#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct TextEntityPosition {
    /// Offset in UTF-16 code units to the start of the entity.
    pub offset: u32,
    /// Length of the entity in UTF-16 code units.
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
