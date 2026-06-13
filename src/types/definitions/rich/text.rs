use serde::{Deserialize, Serialize};

use crate::types::{Integer, User};

/// Represents a rich formatted text.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(from = "RawRichText", into = "RawRichText")]
pub enum RichText {
    /// An anchor.
    Anchor(String),
    /// A link to an anchor.
    AnchorLink(RichTextValue),
    /// A list of rich text.
    Array(Vec<RichText>),
    /// A text with a bank card number.
    BankCardNumber(RichTextValue),
    /// A bold text.
    Bold(Box<RichText>),
    /// A bot command.
    BotCommand(RichTextValue),
    /// A cashtag.
    Cashtag(RichTextValue),
    /// A monowidth text.
    Code(Box<RichText>),
    /// A custom emoji.
    CustomEmoji(RichTextCustomEmoji),
    /// Formatted date and time.
    DateTime(RichTextDateTime),
    /// A text with an email address.
    EmailAddress(RichTextValue),
    /// A hashtag.
    Hashtag(RichTextValue),
    /// An italicized text.
    Italic(Box<RichText>),
    /// A marked text.
    Marked(Box<RichText>),
    /// A mathematical expression.
    MathematicalExpression(String),
    /// A mention by a username.
    Mention(RichTextValue),
    /// A text with a phone number.
    PhoneNumber(RichTextValue),
    /// A plain text.
    PlainText(String),
    /// A reference.
    Reference(String),
    /// A link to a reference.
    ReferenceLink(RichTextValue),
    /// A text covered by a spoler.
    Spoiler(Box<RichText>),
    /// A strikethrough text.
    Strikethrough(Box<RichText>),
    /// A subscript text.
    Subscript(Box<RichText>),
    /// A superscript text.
    Superscript(Box<RichText>),
    /// A mention of a Telegram user by their identifier.
    TextMention(RichTextTextMention),
    /// An underlined text.
    Underline(Box<RichText>),
    /// A text with a link.
    Url(RichTextValue),
}

impl RichText {
    /// Creates a new anchor `RichText`.
    ///
    /// # Arguments
    ///
    /// * `value` - The name of the anchor.
    pub fn anchor<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self::Anchor(value.into())
    }

    /// Creates a new anchor link `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    /// * `value` - The name of the anchor.
    pub fn anchor_link<A, B>(text: A, value: B) -> Self
    where
        A: Into<RichText>,
        B: Into<String>,
    {
        Self::AnchorLink(RichTextValue::new(text, value))
    }

    /// Creates a new array `RichText`.
    ///
    /// # Arguments
    ///
    /// * `value` - The rich text elements.
    pub fn array<A, B>(value: A) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<RichText>,
    {
        Self::Array(value.into_iter().map(Into::into).collect())
    }

    /// Creates a new bank card number `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    /// * `value` - The number.
    pub fn bank_card_number<A, B>(text: A, value: B) -> Self
    where
        A: Into<RichText>,
        B: Into<String>,
    {
        Self::BankCardNumber(RichTextValue::new(text, value))
    }

    /// Creates a new bold `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    pub fn bold<T>(text: T) -> Self
    where
        T: Into<RichText>,
    {
        Self::Bold(Box::new(text.into()))
    }

    /// Creates a new bot command `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    /// * `value` - The command.
    pub fn bot_command<A, B>(text: A, value: B) -> Self
    where
        A: Into<RichText>,
        B: Into<String>,
    {
        Self::BotCommand(RichTextValue::new(text, value))
    }

    /// Creates a new cashtag `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    /// * `value` - The cashtag.
    pub fn cashtag<A, B>(text: A, value: B) -> Self
    where
        A: Into<RichText>,
        B: Into<String>,
    {
        Self::Cashtag(RichTextValue::new(text, value))
    }

    /// Creates a new code `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    pub fn code<T>(text: T) -> Self
    where
        T: Into<RichText>,
    {
        Self::Code(Box::new(text.into()))
    }

    /// Creates a new custom emoji `RichText`.
    ///
    /// # Arguments
    ///
    /// * `id` - The emoji ID.
    /// * `alternative_text` - The alternative emoji.
    pub fn custom_emoji<A, B>(id: A, alternative_text: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self::CustomEmoji(RichTextCustomEmoji::new(id, alternative_text))
    }

    /// Creates a new date time `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    /// * `unix_time` - The unix time.
    /// * `format` - The format.
    pub fn date_time<A, B>(text: A, unix_time: Integer, format: B) -> Self
    where
        A: Into<RichText>,
        B: Into<String>,
    {
        Self::DateTime(RichTextDateTime::new(text, unix_time, format))
    }

    /// Creates a new email address `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    /// * `value` - The email.
    pub fn email_address<A, B>(text: A, value: B) -> Self
    where
        A: Into<RichText>,
        B: Into<String>,
    {
        Self::EmailAddress(RichTextValue::new(text, value))
    }

    /// Creates a new hashtag `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    /// * `value` - The hashtag.
    pub fn hashtag<A, B>(text: A, value: B) -> Self
    where
        A: Into<RichText>,
        B: Into<String>,
    {
        Self::Hashtag(RichTextValue::new(text, value))
    }

    /// Creates a new italic `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    pub fn italic<T>(text: T) -> Self
    where
        T: Into<RichText>,
    {
        Self::Italic(Box::new(text.into()))
    }

    /// Creates a new marked `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    pub fn marked<T>(text: T) -> Self
    where
        T: Into<RichText>,
    {
        Self::Marked(Box::new(text.into()))
    }

    /// Creates a new mathematical expression `RichText`.
    ///
    /// # Arguments
    ///
    /// * `value` - The expression.
    pub fn mathematical_expression<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self::MathematicalExpression(value.into())
    }

    /// Creates a new mention `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    /// * `value` - The username.
    pub fn mention<A, B>(text: A, value: B) -> Self
    where
        A: Into<RichText>,
        B: Into<String>,
    {
        Self::Mention(RichTextValue::new(text, value))
    }

    /// Creates a new phone number `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    /// * `value` - The phone number.
    pub fn phone_number<A, B>(text: A, value: B) -> Self
    where
        A: Into<RichText>,
        B: Into<String>,
    {
        Self::PhoneNumber(RichTextValue::new(text, value))
    }

    /// Creates a new plain text `RichText`.
    ///
    /// # Arguments
    ///
    /// * `value` - The text.
    pub fn plain_text<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self::PlainText(value.into())
    }

    /// Creates a new reference `RichText`.
    ///
    /// # Arguments
    ///
    /// * `value` - The reference.
    pub fn reference<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self::Reference(value.into())
    }

    /// Creates a new reference link `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    /// * `value` - The link.
    pub fn reference_link<A, B>(text: A, value: B) -> Self
    where
        A: Into<RichText>,
        B: Into<String>,
    {
        Self::ReferenceLink(RichTextValue::new(text, value))
    }

    /// Creates a new spoiler `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    pub fn spoiler<T>(text: T) -> Self
    where
        T: Into<RichText>,
    {
        Self::Spoiler(Box::new(text.into()))
    }

    /// Creates a new strikethrough `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    pub fn strikethrough<T>(text: T) -> Self
    where
        T: Into<RichText>,
    {
        Self::Strikethrough(Box::new(text.into()))
    }

    /// Creates a new subscript `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    pub fn subscript<T>(text: T) -> Self
    where
        T: Into<RichText>,
    {
        Self::Subscript(Box::new(text.into()))
    }

    /// Creates a new superscript `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    pub fn superscript<T>(text: T) -> Self
    where
        T: Into<RichText>,
    {
        Self::Superscript(Box::new(text.into()))
    }

    /// Creates a new text mention `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    /// * `user` - The mentioned user.
    pub fn text_mention<T>(text: T, user: User) -> Self
    where
        T: Into<RichText>,
    {
        Self::TextMention(RichTextTextMention::new(text, user))
    }

    /// Creates a new underline `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    pub fn underline<T>(text: T) -> Self
    where
        T: Into<RichText>,
    {
        Self::Underline(Box::new(text.into()))
    }

    /// Creates a new URL `RichText`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    /// * `value` - The URL.
    pub fn url<A, B>(text: A, value: B) -> Self
    where
        A: Into<RichText>,
        B: Into<String>,
    {
        Self::Url(RichTextValue::new(text, value))
    }
}

impl From<String> for RichText {
    fn from(value: String) -> Self {
        Self::PlainText(value)
    }
}

impl From<&str> for RichText {
    fn from(value: &str) -> Self {
        Self::plain_text(value)
    }
}

/// Represents a custom emoji in a rich text.
#[derive(Clone, Debug)]
pub struct RichTextCustomEmoji {
    /// Unique identifier of the custom emoji.
    pub id: String,
    /// Alternative emoji for the custom emoji.
    pub alternative_text: String,
}

impl RichTextCustomEmoji {
    /// Creates a new `RichTextCustomEmoji`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the custom emoji.
    /// * `alternative_text` - Alternative emoji for the custom emoji.
    pub fn new<A, B>(id: A, alternative_text: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            id: id.into(),
            alternative_text: alternative_text.into(),
        }
    }
}

/// Represents a formatted date and time in a rich text.
#[derive(Clone, Debug)]
pub struct RichTextDateTime {
    /// The text.
    pub text: Box<RichText>,
    /// The unix time associated with the entity.
    pub unix_time: Integer,
    /// The string that defines the formatting of the date and time.
    pub format: String,
}

impl RichTextDateTime {
    /// Creates a new `RichTextDateTime`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    /// * `unix_time` - The unix time.
    /// * `format` - The format.
    pub fn new<A, B>(text: A, unix_time: Integer, format: B) -> Self
    where
        A: Into<RichText>,
        B: Into<String>,
    {
        Self {
            text: Box::new(text.into()),
            unix_time,
            format: format.into(),
        }
    }
}

/// Represents a mention of a Telegram user in a rich text.
#[derive(Clone, Debug)]
pub struct RichTextTextMention {
    /// The text.
    pub text: Box<RichText>,
    /// The mentioned user.
    pub user: User,
}

impl RichTextTextMention {
    /// Creates a new `RichTextTextMention`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    /// * `user` - The mentioned user.
    pub fn new<T>(text: T, user: User) -> Self
    where
        T: Into<RichText>,
    {
        Self {
            text: Box::new(text.into()),
            user,
        }
    }
}

/// Represents a value in a rich text.
#[derive(Debug, Clone)]
pub struct RichTextValue {
    /// The text.
    pub text: Box<RichText>,
    /// The associated value.
    pub value: String,
}

impl RichTextValue {
    /// Creates a new `RichTextValue`.
    ///
    /// # Arguments
    ///
    /// * `text` - The text.
    /// * `value` - The associated value.
    pub fn new<A, B>(text: A, value: B) -> Self
    where
        A: Into<RichText>,
        B: Into<String>,
    {
        Self {
            text: Box::new(text.into()),
            value: value.into(),
        }
    }
}

impl From<RawRichText> for RichText {
    fn from(value: RawRichText) -> Self {
        match value {
            RawRichText::Anchor { name } => Self::Anchor(name),
            RawRichText::AnchorLink {
                text,
                anchor_name: value,
            } => Self::AnchorLink(RichTextValue {
                text: Box::new(Self::from(*text)),
                value,
            }),
            RawRichText::Array(value) => Self::Array(value.into_iter().map(Self::from).collect()),
            RawRichText::BankCardNumber {
                text,
                bank_card_number: value,
            } => Self::BankCardNumber(RichTextValue {
                text: Box::new(Self::from(*text)),
                value,
            }),
            RawRichText::Bold { text } => Self::Bold(Box::new(Self::from(*text))),
            RawRichText::BotCommand {
                text,
                bot_command: value,
            } => Self::BotCommand(RichTextValue {
                text: Box::new(Self::from(*text)),
                value,
            }),
            RawRichText::Cashtag { text, cashtag: value } => Self::Cashtag(RichTextValue {
                text: Box::new(Self::from(*text)),
                value,
            }),
            RawRichText::Code { text } => Self::Code(Box::new(Self::from(*text))),
            RawRichText::CustomEmoji {
                custom_emoji_id: id,
                alternative_text,
            } => Self::CustomEmoji(RichTextCustomEmoji { id, alternative_text }),
            RawRichText::DateTime {
                text,
                unix_time,
                date_time_format: format,
            } => Self::DateTime(RichTextDateTime {
                text: Box::new(Self::from(*text)),
                unix_time,
                format,
            }),
            RawRichText::EmailAddress {
                text,
                email_address: value,
            } => Self::EmailAddress(RichTextValue {
                text: Box::new(Self::from(*text)),
                value,
            }),
            RawRichText::Hashtag { text, hashtag: value } => Self::Hashtag(RichTextValue {
                text: Box::new(Self::from(*text)),
                value,
            }),
            RawRichText::Italic { text } => Self::Italic(Box::new(Self::from(*text))),
            RawRichText::Marked { text } => Self::Marked(Box::new(Self::from(*text))),
            RawRichText::MathematicalExpression { expression } => Self::MathematicalExpression(expression),
            RawRichText::Mention { text, username: value } => Self::Mention(RichTextValue {
                text: Box::new(Self::from(*text)),
                value,
            }),
            RawRichText::PhoneNumber {
                text,
                phone_number: value,
            } => Self::PhoneNumber(RichTextValue {
                text: Box::new(Self::from(*text)),
                value,
            }),
            RawRichText::PlainText(value) => Self::PlainText(value),
            RawRichText::Reference { name } => Self::Reference(name),
            RawRichText::ReferenceLink {
                text,
                reference_name: value,
            } => Self::ReferenceLink(RichTextValue {
                text: Box::new(Self::from(*text)),
                value,
            }),
            RawRichText::Spoiler { text } => Self::Spoiler(Box::new(Self::from(*text))),
            RawRichText::Strikethrough { text } => Self::Strikethrough(Box::new(Self::from(*text))),
            RawRichText::Subscript { text } => Self::Subscript(Box::new(Self::from(*text))),
            RawRichText::Superscript { text } => Self::Superscript(Box::new(Self::from(*text))),
            RawRichText::TextMention { text, user } => Self::TextMention(RichTextTextMention {
                text: Box::new(Self::from(*text)),
                user,
            }),
            RawRichText::Underline { text } => Self::Underline(Box::new(Self::from(*text))),
            RawRichText::Url { text, url: value } => Self::Url(RichTextValue {
                text: Box::new(Self::from(*text)),
                value,
            }),
        }
    }
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
enum RawRichText {
    Anchor {
        name: String,
    },
    AnchorLink {
        text: Box<RawRichText>,
        anchor_name: String,
    },
    BankCardNumber {
        text: Box<RawRichText>,
        bank_card_number: String,
    },
    Bold {
        text: Box<RawRichText>,
    },
    BotCommand {
        text: Box<RawRichText>,
        bot_command: String,
    },
    Cashtag {
        text: Box<RawRichText>,
        cashtag: String,
    },
    Code {
        text: Box<RawRichText>,
    },
    CustomEmoji {
        custom_emoji_id: String,
        alternative_text: String,
    },
    DateTime {
        text: Box<RawRichText>,
        unix_time: Integer,
        date_time_format: String,
    },
    EmailAddress {
        text: Box<RawRichText>,
        email_address: String,
    },
    Hashtag {
        text: Box<RawRichText>,
        hashtag: String,
    },
    Italic {
        text: Box<RawRichText>,
    },
    Marked {
        text: Box<RawRichText>,
    },
    MathematicalExpression {
        expression: String,
    },
    Mention {
        text: Box<RawRichText>,
        username: String,
    },
    PhoneNumber {
        text: Box<RawRichText>,
        phone_number: String,
    },
    Reference {
        name: String,
    },
    ReferenceLink {
        text: Box<RawRichText>,
        reference_name: String,
    },
    Spoiler {
        text: Box<RawRichText>,
    },
    Strikethrough {
        text: Box<RawRichText>,
    },
    Subscript {
        text: Box<RawRichText>,
    },
    Superscript {
        text: Box<RawRichText>,
    },
    TextMention {
        text: Box<RawRichText>,
        user: User,
    },
    Underline {
        text: Box<RawRichText>,
    },
    Url {
        text: Box<RawRichText>,
        url: String,
    },
    #[serde(untagged)]
    Array(Vec<RawRichText>),
    #[serde(untagged)]
    PlainText(String),
}

impl From<RichText> for RawRichText {
    fn from(value: RichText) -> Self {
        match value {
            RichText::Anchor(name) => Self::Anchor { name },
            RichText::AnchorLink(RichTextValue {
                text,
                value: anchor_name,
            }) => Self::AnchorLink {
                text: Box::new(Self::from(*text)),
                anchor_name,
            },
            RichText::BankCardNumber(RichTextValue {
                text,
                value: bank_card_number,
            }) => Self::BankCardNumber {
                text: Box::new(Self::from(*text)),
                bank_card_number,
            },
            RichText::Bold(text) => Self::Bold {
                text: Box::new(Self::from(*text)),
            },
            RichText::BotCommand(RichTextValue {
                text,
                value: bot_command,
            }) => Self::BotCommand {
                text: Box::new(Self::from(*text)),
                bot_command,
            },
            RichText::Cashtag(RichTextValue { text, value: cashtag }) => Self::Cashtag {
                text: Box::new(Self::from(*text)),
                cashtag,
            },
            RichText::Code(text) => Self::Code {
                text: Box::new(Self::from(*text)),
            },
            RichText::CustomEmoji(RichTextCustomEmoji {
                id: custom_emoji_id,
                alternative_text,
            }) => Self::CustomEmoji {
                custom_emoji_id,
                alternative_text,
            },
            RichText::DateTime(RichTextDateTime {
                text,
                unix_time,
                format: date_time_format,
            }) => Self::DateTime {
                text: Box::new(Self::from(*text)),
                unix_time,
                date_time_format,
            },
            RichText::EmailAddress(RichTextValue {
                text,
                value: email_address,
            }) => Self::EmailAddress {
                text: Box::new(Self::from(*text)),
                email_address,
            },
            RichText::Hashtag(RichTextValue { text, value: hashtag }) => Self::Hashtag {
                text: Box::new(Self::from(*text)),
                hashtag,
            },
            RichText::Italic(text) => Self::Italic {
                text: Box::new(Self::from(*text)),
            },
            RichText::Marked(text) => Self::Marked {
                text: Box::new(Self::from(*text)),
            },
            RichText::MathematicalExpression(expression) => Self::MathematicalExpression { expression },
            RichText::Mention(RichTextValue { text, value: username }) => Self::Mention {
                text: Box::new(Self::from(*text)),
                username,
            },
            RichText::PhoneNumber(RichTextValue {
                text,
                value: phone_number,
            }) => Self::PhoneNumber {
                text: Box::new(Self::from(*text)),
                phone_number,
            },
            RichText::Reference(name) => Self::Reference { name },
            RichText::ReferenceLink(RichTextValue {
                text,
                value: reference_name,
            }) => Self::ReferenceLink {
                text: Box::new(Self::from(*text)),
                reference_name,
            },
            RichText::Spoiler(text) => Self::Spoiler {
                text: Box::new(Self::from(*text)),
            },
            RichText::Strikethrough(text) => Self::Strikethrough {
                text: Box::new(Self::from(*text)),
            },
            RichText::Subscript(text) => Self::Subscript {
                text: Box::new(Self::from(*text)),
            },
            RichText::Superscript(text) => Self::Superscript {
                text: Box::new(Self::from(*text)),
            },
            RichText::TextMention(RichTextTextMention { text, user }) => Self::TextMention {
                text: Box::new(Self::from(*text)),
                user,
            },
            RichText::Underline(text) => Self::Underline {
                text: Box::new(Self::from(*text)),
            },
            RichText::Url(RichTextValue { text, value: url }) => Self::Url {
                text: Box::new(Self::from(*text)),
                url,
            },
            RichText::Array(value) => Self::Array(value.into_iter().map(Self::from).collect::<Vec<Self>>()),
            RichText::PlainText(value) => Self::PlainText(value),
        }
    }
}
