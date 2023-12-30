use serde::{Deserialize, Serialize};

use super::raw::{
    RawInlineQueryResult,
    RawInlineQueryResultData,
    RawInlineQueryResultDataError::{self, MissingField},
    RawInlineQueryResultType,
};
use crate::types::{InlineKeyboardMarkup, InputMessageContent, Integer, ParseMode, TextEntities, TextEntity};

#[cfg(test)]
mod tests;

/// Represents a link to a file.
///
/// By default, this file will be sent by the user with an optional caption.
/// Alternatively, you can use [`Self::with_input_message_content`] to send a message
/// with the specified content instead of the file.
/// Currently, only .PDF and .ZIP files can be sent using this method.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultDocument {
    document_url: String,
    id: String,
    mime_type: String,
    title: String,
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
    thumbnail_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_width: Option<Integer>,
}

impl InlineQueryResultDocument {
    /// Creates a new `InlineQueryResultDocument`.
    ///
    /// # Arguments
    ///
    /// * `document_url` - A valid URL for the file.
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `mime_type` - MIME type of the content of the file,
    ///                 either “application/pdf” or “application/zip”.
    /// * `title` - Title of the result.
    pub fn new<A, B, C, D>(document_url: A, id: B, mime_type: C, title: D) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
        D: Into<String>,
    {
        Self {
            document_url: document_url.into(),
            id: id.into(),
            mime_type: mime_type.into(),
            title: title.into(),
            caption: None,
            caption_entities: None,
            description: None,
            input_message_content: None,
            parse_mode: None,
            reply_markup: None,
            thumbnail_height: None,
            thumbnail_url: None,
            thumbnail_width: None,
        }
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.caption = Some(value.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.caption_entities = Some(value.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self.caption_entities = None;
        self
    }

    /// Sets a new description.
    ///
    /// # Arguments
    ///
    /// * `value` - Short description of the result.
    pub fn with_description<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.description = Some(value.into());
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the file.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        self.input_message_content = Some(value.into());
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }

    /// Sets a new thumbnail height.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail height.
    pub fn with_thumbnail_height(mut self, value: Integer) -> Self {
        self.thumbnail_height = Some(value);
        self
    }

    /// Sets a new thumbnail width.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail width.
    pub fn with_thumbnail_width(mut self, value: Integer) -> Self {
        self.thumbnail_width = Some(value);
        self
    }

    /// Sets a new thumbnail URL.
    ///
    /// # Arguments
    ///
    /// * `value` - URL of the thumbnail (jpeg only) for the file.
    pub fn with_thumbnail_url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.thumbnail_url = Some(value.into());
        self
    }
}

/// Represents a link to a file stored on the Telegram servers.
///
/// By default, this file will be sent by the user with an optional caption.
/// Alternatively, you can use [Self::with_input_message_content]
/// to send a message with the specified content instead of the file.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultCachedDocument {
    id: String,
    title: String,
    document_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedDocument {
    /// Creates a new `InlineQueryResultCachedDocument`.
    ///
    /// # Arguments
    ///
    /// * `document_file_id` - A valid file identifier of the file.
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `title` - Title of the result.
    pub fn new<A, B, C>(document_file_id: A, id: B, title: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            document_file_id: document_file_id.into(),
            id: id.into(),
            title: title.into(),
            caption: None,
            caption_entities: None,
            description: None,
            input_message_content: None,
            parse_mode: None,
            reply_markup: None,
        }
    }

    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption; 0-1024 characters.
    pub fn with_caption<T>(mut self, caption: T) -> Self
    where
        T: Into<String>,
    {
        self.caption = Some(caption.into());
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - The list of special entities that appear in the caption.
    ///
    /// Caption parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.caption_entities = Some(value.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets a new caption parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Caption entities will be set to [`None`] when this method is called.
    pub fn with_caption_parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self.caption_entities = None;
        self
    }

    /// Sets a new description.
    ///
    /// # Arguments
    ///
    /// * value - Short description of the result.
    pub fn with_description<T>(mut self, description: T) -> Self
    where
        T: Into<String>,
    {
        self.description = Some(description.into());
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the file.
    pub fn with_input_message_content<T>(mut self, input_message_content: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        self.input_message_content = Some(input_message_content.into());
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultDocument {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            description: value.data.description,
            document_url: value.data.document_url.ok_or(MissingField("document_url"))?,
            id: value.id,
            input_message_content: value.data.input_message_content,
            mime_type: value.data.mime_type.ok_or(MissingField("mime_type"))?,
            parse_mode: value.data.parse_mode,
            reply_markup: value.data.reply_markup,
            thumbnail_height: value.data.thumbnail_height,
            thumbnail_url: value.data.thumbnail_url,
            thumbnail_width: value.data.thumbnail_width,
            title: value.data.title.ok_or(MissingField("title"))?,
        })
    }
}

impl From<InlineQueryResultDocument> for RawInlineQueryResult {
    fn from(value: InlineQueryResultDocument) -> Self {
        Self {
            data: RawInlineQueryResultData {
                caption: value.caption,
                caption_entities: value.caption_entities,
                description: value.description,
                document_url: Some(value.document_url),
                input_message_content: value.input_message_content,
                mime_type: Some(value.mime_type),
                parse_mode: value.parse_mode,
                reply_markup: value.reply_markup,
                thumbnail_height: value.thumbnail_height,
                thumbnail_url: value.thumbnail_url,
                thumbnail_width: value.thumbnail_width,
                title: Some(value.title),
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::Document,
        }
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultCachedDocument {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            description: value.data.description,
            document_file_id: value.data.document_file_id.ok_or(MissingField("document_file_id"))?,
            id: value.id,
            input_message_content: value.data.input_message_content,
            parse_mode: value.data.parse_mode,
            reply_markup: value.data.reply_markup,
            title: value.data.title.ok_or(MissingField("title"))?,
        })
    }
}

impl From<InlineQueryResultCachedDocument> for RawInlineQueryResult {
    fn from(value: InlineQueryResultCachedDocument) -> Self {
        Self {
            data: RawInlineQueryResultData {
                caption: value.caption,
                caption_entities: value.caption_entities,
                description: value.description,
                document_file_id: Some(value.document_file_id),
                input_message_content: value.input_message_content,
                parse_mode: value.parse_mode,
                reply_markup: value.reply_markup,
                title: Some(value.title),
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::CachedDocument,
        }
    }
}
