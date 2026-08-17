use std::{
    convert::TryFrom,
    error::Error,
    fmt,
    ops::{Index, IndexMut, Range},
};

use serde::{Deserialize, Serialize};

use crate::types::{Integer, User};

/// Represents a collection of text entities.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
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
    /// Formatted date and time.
    DateTime {
        /// Position of entity in text.
        position: TextEntityPosition,
        /// Unix time associated with the entity.
        unix_time: Option<Integer>,
        /// String that defines the formatting of the date and time.
        ///
        /// See (date-time entity formatting)[1] for more details.
        ///
        /// [1]: https://core.telegram.org/bots/api#date-time-entity-formatting
        format: Option<String>,
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
            pub fn $method_name<T: Into<TextEntityPosition>>(pos: T) -> Self {
                Self::$enum_variant(pos.into())
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
    pub fn custom_emoji<A, B>(pos: A, custom_emoji_id: B) -> Self
    where
        A: Into<TextEntityPosition>,
        B: Into<String>,
    {
        Self::CustomEmoji {
            position: pos.into(),
            custom_emoji_id: custom_emoji_id.into(),
        }
    }

    /// Creates a new `TextEntity`.
    ///
    /// # Arguments
    ///
    /// * `pos` - Position of the entity in UTF-16 code units.
    /// * `unix_time` - Unix time associated with the entity.
    /// * `format` - String that defines the formatting of the date and time.
    pub fn date_time<A, B>(pos: A, unix_time: Option<Integer>, format: Option<B>) -> Self
    where
        A: Into<TextEntityPosition>,
        B: Into<String>,
    {
        Self::DateTime {
            position: pos.into(),
            unix_time,
            format: format.map(Into::into),
        }
    }

    /// Creates a new `TextEntity`.
    ///
    /// # Arguments
    ///
    /// * `pos` - Position of the entity in UTF-16 code units.
    /// * `language` - The programming language of the entity text.
    pub fn pre<A, B>(pos: A, language: Option<B>) -> Self
    where
        A: Into<TextEntityPosition>,
        B: Into<String>,
    {
        Self::Pre {
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
    pub fn text_link<A, B>(pos: A, url: B) -> Self
    where
        A: Into<TextEntityPosition>,
        B: Into<String>,
    {
        Self::TextLink {
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
    pub fn text_mention<T>(pos: T, user: User) -> Self
    where
        T: Into<TextEntityPosition>,
    {
        Self::TextMention {
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
    CustomEmoji {
        custom_emoji_id: Option<String>,
    },
    DateTime {
        date_time_format: Option<String>,
        unix_time: Option<Integer>,
    },
    Email,
    ExpandableBlockquote,
    Hashtag,
    Italic,
    Mention,
    PhoneNumber,
    Pre {
        language: Option<String>,
    },
    Spoiler,
    Strikethrough,
    TextLink {
        url: Option<String>,
    },
    TextMention {
        user: Option<User>,
    },
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
}

impl Error for TextEntityError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::NoCustomEmoji | Self::NoUrl | Self::NoUser => None,
        }
    }
}

impl fmt::Display for TextEntityError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(
            out,
            "{}",
            match self {
                Self::NoCustomEmoji => "custom emoji is required for custom_emoji entity",
                Self::NoUrl => "URL is required for text_link entity",
                Self::NoUser => "user is required for text_mention entity",
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
            RawTextEntityType::Blockquote => Self::Blockquote(position),
            RawTextEntityType::Bold => Self::Bold(position),
            RawTextEntityType::BotCommand => Self::BotCommand(position),
            RawTextEntityType::Cashtag => Self::Cashtag(position),
            RawTextEntityType::Code => Self::Code(position),
            RawTextEntityType::CustomEmoji { custom_emoji_id } => Self::CustomEmoji {
                position,
                custom_emoji_id: custom_emoji_id.ok_or(TextEntityError::NoCustomEmoji)?,
            },
            RawTextEntityType::DateTime {
                unix_time,
                date_time_format,
            } => Self::DateTime {
                position,
                unix_time,
                format: date_time_format,
            },
            RawTextEntityType::Email => Self::Email(position),
            RawTextEntityType::ExpandableBlockquote => Self::ExpandableBlockquote(position),
            RawTextEntityType::Hashtag => Self::Hashtag(position),
            RawTextEntityType::Italic => Self::Italic(position),
            RawTextEntityType::Mention => Self::Mention(position),
            RawTextEntityType::PhoneNumber => Self::PhoneNumber(position),
            RawTextEntityType::Pre { language } => Self::Pre { position, language },
            RawTextEntityType::Spoiler => Self::Spoiler(position),
            RawTextEntityType::Strikethrough => Self::Strikethrough(position),
            RawTextEntityType::TextLink { url } => Self::TextLink {
                position,
                url: url.ok_or(TextEntityError::NoUrl)?,
            },
            RawTextEntityType::TextMention { user } => Self::TextMention {
                position,
                user: user.ok_or(TextEntityError::NoUser)?,
            },
            RawTextEntityType::Underline => Self::Underline(position),
            RawTextEntityType::Url => Self::Url(position),
        })
    }
}

impl From<TextEntity> for RawTextEntity {
    fn from(entity: TextEntity) -> Self {
        macro_rules! raw {
            ($entity_type:ident($position:ident $( $($item:ident)+ )?)) => {
                Self {
                    entity_type: RawTextEntityType::$entity_type $( { $($item: $item.into(),)+ } )?,
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
            } => raw!(CustomEmoji(p custom_emoji_id)),
            TextEntity::DateTime {
                position: p,
                unix_time,
                format: date_time_format,
            } => raw!(DateTime(p unix_time date_time_format)),
            TextEntity::Email(p) => raw!(Email(p)),
            TextEntity::ExpandableBlockquote(p) => raw!(ExpandableBlockquote(p)),
            TextEntity::Hashtag(p) => raw!(Hashtag(p)),
            TextEntity::Italic(p) => raw!(Italic(p)),
            TextEntity::Mention(p) => raw!(Mention(p)),
            TextEntity::PhoneNumber(p) => raw!(PhoneNumber(p)),
            TextEntity::Pre { position: p, language } => raw!(Pre(p language)),
            TextEntity::Spoiler(p) => raw!(Spoiler(p)),
            TextEntity::Strikethrough(p) => raw!(Strikethrough(p)),
            TextEntity::TextLink { position: p, url } => raw!(TextLink(p url)),
            TextEntity::TextMention { position: p, user } => raw!(TextMention(p user)),
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

#[cfg(test)]
mod tests {
    use std::error::Error;

    use crate::types::*;

    #[test]
    fn collection() {
        let mut obj = TextEntities::default();
        obj.push(TextEntity::pre(0..2, Some("test")));
        obj.push(TextEntity::text_link(0..2, "test"));
        obj.push(TextEntity::text_mention(0..2, User::new(1, "John", false)));
        assert!(matches!(obj[0], TextEntity::Pre { .. }));
        assert!(matches!(obj[1], TextEntity::TextLink { .. }));
        assert!(matches!(obj[2], TextEntity::TextMention { .. }));
        obj[0] = TextEntity::underline(0..3);
        let i = obj.into_iter();
        assert_eq!(i.count(), 3);
    }

    #[test]
    fn deserialize() {
        let input = serde_json::json!({
            "message_id": 1, "date": 0,
            "from": {"id": 1, "first_name": "firstname", "is_bot": false},
            "chat": {"id": 1, "type": "supergroup", "title": "super-group-title"},
            "text": "b /c $c cd u@h.z #h i @m p pre l tm url u s sx pre 🤡 bq ebq 20:20 0",
            "entities": [
                {"type": "bold", "offset": 0, "length": 1},
                {"type": "bot_command", "offset": 3, "length": 2},
                {"type": "cashtag", "offset": 6, "length": 2},
                {"type": "code", "offset": 9, "length": 2},
                {"type": "email", "offset": 12, "length": 5},
                {"type": "hashtag", "offset": 18, "length": 2},
                {"type": "italic", "offset": 21, "length": 1},
                {"type": "mention", "offset": 23, "length": 2},
                {"type": "phone_number", "offset": 26, "length": 1},
                {"type": "pre", "offset": 28, "length": 3},
                {"type": "text_link", "offset": 32, "length": 1, "url": "https://example.com"},
                {
                    "type": "text_mention",
                    "offset": 34,
                    "length": 2,
                    "user": {
                        "id": 1,
                        "first_name": "test",
                        "is_bot": false
                    }
                },
                {"type": "url", "offset": 37, "length": 3},
                {"type": "underline", "offset": 41, "length": 1},
                {"type": "spoiler", "offset": 43, "length": 1},
                {"type": "strikethrough", "offset": 45, "length": 2},
                {"type": "pre", "offset": 48, "length": 3, "language": "rust"},
                {"type": "custom_emoji", "offset": 52, "length": 2, "custom_emoji_id": "emoji-id"},
                {"type": "blockquote", "offset": 55, "length": 2},
                {"type": "expandable_blockquote", "offset": 58, "length": 3},
                {"type": "date_time", "offset": 62, "length": 5},
                {"type": "date_time", "offset": 68, "length": 1, "unix_time": 0, "date_time_format": "r"},
            ]
        });
        let msg: Message = serde_json::from_value(input).unwrap();
        if let MessageData::Text(text) = msg.data {
            let entities: Vec<TextEntity> = text.entities.unwrap().into();
            assert_eq!(
                vec![
                    TextEntity::Bold(TextEntityPosition { offset: 0, length: 1 }),
                    TextEntity::bot_command(TextEntityPosition { offset: 3, length: 2 }),
                    TextEntity::Cashtag(TextEntityPosition { offset: 6, length: 2 }),
                    TextEntity::Code(TextEntityPosition { offset: 9, length: 2 }),
                    TextEntity::Email(TextEntityPosition { offset: 12, length: 5 }),
                    TextEntity::Hashtag(TextEntityPosition { offset: 18, length: 2 }),
                    TextEntity::Italic(TextEntityPosition { offset: 21, length: 1 }),
                    TextEntity::Mention(TextEntityPosition { offset: 23, length: 2 }),
                    TextEntity::PhoneNumber(TextEntityPosition { offset: 26, length: 1 }),
                    TextEntity::Pre {
                        position: TextEntityPosition { offset: 28, length: 3 },
                        language: None,
                    },
                    TextEntity::TextLink {
                        position: TextEntityPosition { offset: 32, length: 1 },
                        url: String::from("https://example.com"),
                    },
                    TextEntity::TextMention {
                        position: TextEntityPosition { offset: 34, length: 2 },
                        user: User::new(1, "test", false),
                    },
                    TextEntity::Url(TextEntityPosition { offset: 37, length: 3 }),
                    TextEntity::Underline(TextEntityPosition { offset: 41, length: 1 }),
                    TextEntity::Spoiler(TextEntityPosition { offset: 43, length: 1 }),
                    TextEntity::Strikethrough(TextEntityPosition { offset: 45, length: 2 }),
                    TextEntity::Pre {
                        position: TextEntityPosition { offset: 48, length: 3 },
                        language: Some(String::from("rust")),
                    },
                    TextEntity::CustomEmoji {
                        custom_emoji_id: String::from("emoji-id"),
                        position: TextEntityPosition { offset: 52, length: 2 },
                    },
                    TextEntity::Blockquote(TextEntityPosition { offset: 55, length: 2 }),
                    TextEntity::ExpandableBlockquote(TextEntityPosition { offset: 58, length: 3 }),
                    TextEntity::DateTime {
                        position: TextEntityPosition { offset: 62, length: 5 },
                        unix_time: None,
                        format: None
                    },
                    TextEntity::DateTime {
                        position: TextEntityPosition { offset: 68, length: 1 },
                        unix_time: Some(0),
                        format: Some(String::from("r"))
                    },
                ],
                entities
            );
        } else {
            panic!("Unexpected message data: {:?}", msg.data);
        }
    }

    #[test]
    fn deserialize_failed() {
        for (input, error) in [
            (
                serde_json::json!([{"type": "text_link", "offset": 0, "length": 2}]),
                "URL is required for text_link entity",
            ),
            (
                serde_json::json!([{"type": "text_mention", "offset": 0, "length": 2}]),
                "user is required for text_mention entity",
            ),
            (
                serde_json::json!([{"type": "custom_emoji", "offset": 0, "length": 2}]),
                "custom emoji is required for custom_emoji entity",
            ),
        ] {
            let err = serde_json::from_value::<TextEntities>(input).unwrap_err();
            assert_eq!(err.to_string(), error);
            assert!(err.source().is_none());
        }
    }

    #[test]
    fn serialize() {
        for (entity, expected) in vec![
            (
                TextEntity::Blockquote(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "blockquote",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Bold(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "bold",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::bot_command(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "bot_command",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Cashtag(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "cashtag",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Code(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "code",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::custom_emoji(0..2, "emoji-id"),
                serde_json::json!({
                    "type": "custom_emoji",
                    "offset": 0,
                    "length": 2,
                    "custom_emoji_id": "emoji-id"
                }),
            ),
            (
                TextEntity::date_time(0..2, None, None::<String>),
                serde_json::json!({
                    "type": "date_time",
                    "offset": 0,
                    "length": 2,
                }),
            ),
            (
                TextEntity::date_time(0..2, Some(0), Some("r")),
                serde_json::json!({
                    "type": "date_time",
                    "offset": 0,
                    "length": 2,
                    "unix_time": 0,
                    "date_time_format": "r",
                }),
            ),
            (
                TextEntity::Email(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "email",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::ExpandableBlockquote(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "expandable_blockquote",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Hashtag(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "hashtag",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Italic(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "italic",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Mention(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "mention",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::PhoneNumber(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "phone_number",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Pre {
                    position: TextEntityPosition { offset: 0, length: 10 },
                    language: None,
                },
                serde_json::json!({
                    "type": "pre",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Pre {
                    position: TextEntityPosition { offset: 0, length: 10 },
                    language: Some(String::from("rust")),
                },
                serde_json::json!({
                    "type": "pre",
                    "offset": 0,
                    "length": 10,
                    "language": "rust"
                }),
            ),
            (
                TextEntity::Spoiler(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "spoiler",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Strikethrough(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "strikethrough",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::TextLink {
                    position: TextEntityPosition { offset: 0, length: 21 },
                    url: String::from("https://rust-lang.org"),
                },
                serde_json::json!({
                    "type": "text_link",
                    "offset": 0,
                    "length": 21,
                    "url": "https://rust-lang.org"
                }),
            ),
            (
                TextEntity::TextMention {
                    position: TextEntityPosition { offset: 0, length: 4 },
                    user: User::new(1, "test", false),
                },
                serde_json::json!({
                    "type": "text_mention",
                    "offset": 0,
                    "length": 4,
                    "user": {
                        "id": 1,
                        "first_name": "test",
                        "is_bot": false
                    }
                }),
            ),
            (
                TextEntity::Underline(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "underline",
                    "offset": 0,
                    "length": 10
                }),
            ),
            (
                TextEntity::Url(TextEntityPosition { offset: 0, length: 10 }),
                serde_json::json!({
                    "type": "url",
                    "offset": 0,
                    "length": 10
                }),
            ),
        ] {
            let value: serde_json::Value = serde_json::from_str(&serde_json::to_string(&entity).unwrap()).unwrap();
            assert_eq!(value, expected);
        }
    }
}
