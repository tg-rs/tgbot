use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent, Integer, ParseMode, TextEntities, TextEntity};

use super::raw::{
    RawInlineQueryResult,
    RawInlineQueryResultData,
    RawInlineQueryResultDataError::{self, MissingField},
    RawInlineQueryResultType,
};

#[cfg(test)]
mod tests;

/// Represents a link to a photo
///
/// By default, a photo will be sent by the user with optional caption.
/// Alternatively, you can use [`Self::with_input_message_content`]
/// to send a message with the specified content instead of the photo.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultPhoto {
    id: String,
    photo_url: String,
    thumbnail_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

impl InlineQueryResultPhoto {
    /// Creates a new InlineQueryResultPhoto
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier of the result; 1-64 bytes
    /// * photo_url - A valid URL of the photo; must be in jpeg format; size must not exceed 5MB
    /// * thumbnail_url - URL of the thumbnail of the photo
    pub fn new<A, B, C>(id: A, photo_url: B, thumbnail_url: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            id: id.into(),
            photo_url: photo_url.into(),
            thumbnail_url: thumbnail_url.into(),
            caption: None,
            caption_entities: None,
            description: None,
            input_message_content: None,
            photo_height: None,
            photo_width: None,
            parse_mode: None,
            reply_markup: None,
            title: None,
        }
    }

    /// Sets a new caption
    ///
    /// # Arguments
    ///
    /// * value - Caption of the photo to be sent, 0-1024 characters
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.caption = Some(value.into());
        self
    }

    /// Sets a new caption entities
    ///
    /// # Arguments
    ///
    /// * value - List of special entities that appear in the caption
    ///
    /// Parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.caption_entities = Some(value.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode
    ///
    /// # Arguments
    ///
    /// * value - Parse mode
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self.caption_entities = None;
        self
    }

    /// Sets a new description
    ///
    /// # Arguments
    ///
    /// * value - Short description of the result
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.description = Some(value.into());
        self
    }

    /// Sets a new input message content
    ///
    /// # Arguments
    ///
    /// * value - Content of the message to be sent instead of the photo
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        self.input_message_content = Some(value.into());
        self
    }

    /// Sets a new photo height
    ///
    /// # Arguments
    ///
    /// * value - Height of the photo
    pub fn with_photo_height(mut self, value: Integer) -> Self {
        self.photo_height = Some(value);
        self
    }

    /// Sets a new photo width
    ///
    /// # Arguments
    ///
    /// * value - Width of the photo
    pub fn with_photo_width(mut self, value: Integer) -> Self {
        self.photo_width = Some(value);
        self
    }

    /// Sets a new reply markup
    ///
    /// # Arguments
    ///
    /// * value - Inline keyboard attached to the message
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }

    /// Sets a new title
    ///
    /// # Arguments
    ///
    /// * value - Title of the result
    pub fn with_title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.title = Some(value.into());
        self
    }
}

/// Represents a link to a photo stored on the Telegram servers
///
/// By default, this photo will be sent by the user with an optional caption.
/// Alternatively, you can use [`Self::with_input_message_content`] to send
/// a message with the specified content instead of the photo.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultCachedPhoto {
    id: String,
    photo_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

impl InlineQueryResultCachedPhoto {
    /// Creates a new InlineQueryResultCachedPhoto
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier of the result; 1-64 bytes
    /// * photo_file_id - A valid file identifier of the photo
    pub fn new<A, B>(id: A, photo_file_id: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            id: id.into(),
            photo_file_id: photo_file_id.into(),
            caption: None,
            caption_entities: None,
            description: None,
            input_message_content: None,
            parse_mode: None,
            reply_markup: None,
            title: None,
        }
    }

    /// Sets a new caption
    ///
    /// # Arguments
    ///
    /// * value - Caption of the photo to be sent; 0-1024 characters
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.caption = Some(value.into());
        self
    }

    /// Sets a new caption entities
    ///
    /// # Arguments
    ///
    /// * value - List of special entities that appear in the caption
    ///
    /// Parse mode will be set to [`None`] when this method is called
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.caption_entities = Some(value.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode
    ///
    /// # Arguments
    ///
    /// * value - Parse mode
    ///
    /// Caption entities will be set to [`None`] when this method is called
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self.caption_entities = None;
        self
    }

    /// Sets a new description
    ///
    /// # Arguments
    ///
    /// * value - Short description of the result
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.description = Some(value.into());
        self
    }

    /// Sets a new input message content
    ///
    /// # Arguments
    ///
    /// * value - Content of the message to be sent instead of the photo
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        self.input_message_content = Some(value.into());
        self
    }

    /// Sets a new reply markup
    ///
    /// # Arguments
    ///
    /// * value - Inline keyboard attached to the message
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }

    /// Sets a new title
    ///
    /// # Arguments
    ///
    /// * value - Title for the result
    pub fn with_title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.title = Some(value.into());
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultPhoto {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            description: value.data.description,
            id: value.id,
            input_message_content: value.data.input_message_content,
            parse_mode: value.data.parse_mode,
            photo_height: value.data.photo_height,
            photo_width: value.data.photo_width,
            photo_url: value.data.photo_url.ok_or(MissingField("photo_url"))?,
            reply_markup: value.data.reply_markup,
            thumbnail_url: value.data.thumbnail_url.ok_or(MissingField("thumbnail_url"))?,
            title: value.data.title,
        })
    }
}

impl From<InlineQueryResultPhoto> for RawInlineQueryResult {
    fn from(value: InlineQueryResultPhoto) -> Self {
        Self {
            data: RawInlineQueryResultData {
                caption: value.caption,
                caption_entities: value.caption_entities,
                description: value.description,
                input_message_content: value.input_message_content,
                parse_mode: value.parse_mode,
                photo_height: value.photo_height,
                photo_url: Some(value.photo_url),
                photo_width: value.photo_width,
                reply_markup: value.reply_markup,
                thumbnail_url: Some(value.thumbnail_url),
                title: value.title,
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::Photo,
        }
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultCachedPhoto {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            description: value.data.description,
            id: value.id,
            input_message_content: value.data.input_message_content,
            parse_mode: value.data.parse_mode,
            photo_file_id: value.data.photo_file_id.ok_or(MissingField("photo_file_id"))?,
            title: value.data.title,
            reply_markup: value.data.reply_markup,
        })
    }
}

impl From<InlineQueryResultCachedPhoto> for RawInlineQueryResult {
    fn from(value: InlineQueryResultCachedPhoto) -> Self {
        Self {
            data: RawInlineQueryResultData {
                caption: value.caption,
                caption_entities: value.caption_entities,
                description: value.description,
                input_message_content: value.input_message_content,
                parse_mode: value.parse_mode,
                photo_file_id: Some(value.photo_file_id),
                reply_markup: value.reply_markup,
                title: value.title,
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::CachedPhoto,
        }
    }
}
