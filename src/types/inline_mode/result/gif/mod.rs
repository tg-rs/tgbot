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

/// Represents a link to an animated GIF file.
///
/// By default, this animated GIF file
/// will be sent by the user with optional caption.
/// Alternatively, you can use [Self::with_input_message_content]
/// to send a message with the specified content instead of the animation.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultGif {
    gif_url: String,
    id: String,
    thumbnail_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gif_duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gif_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gif_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_mime_type: Option<String>,
}

impl InlineQueryResultGif {
    /// Creates a new `InlineQueryResultGif`.
    ///
    /// # Arguments
    ///
    /// * `gif_url` - A valid URL for the GIF file; file size must not exceed 1MB.
    /// * `id` - Unique identifier for this result; 1-64 bytes.
    /// * `thumbnail_url` - URL of the static thumbnail for the result (JPEG or GIF).
    pub fn new<A, B, C>(gif_url: A, id: B, thumbnail_url: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            gif_url: gif_url.into(),
            id: id.into(),
            thumbnail_url: thumbnail_url.into(),
            caption: None,
            caption_entities: None,
            gif_duration: None,
            gif_height: None,
            gif_width: None,
            input_message_content: None,
            parse_mode: None,
            reply_markup: None,
            title: None,
            thumbnail_mime_type: None,
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

    /// Sets a new GIF duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Duration of the GIF.
    pub fn with_gif_duration(mut self, value: Integer) -> Self {
        self.gif_duration = Some(value);
        self
    }

    /// Sets a new GIF height.
    ///
    /// # Arguments
    ///
    /// * `value` - Height of the GIF.
    pub fn with_gif_height(mut self, value: Integer) -> Self {
        self.gif_height = Some(value);
        self
    }

    /// Sets a new GIF width.
    ///
    /// # Arguments
    ///
    /// * `value` - Width of the GIF.
    pub fn with_gif_width(mut self, value: Integer) -> Self {
        self.gif_width = Some(value);
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the GIF animation.
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

    /// Sets a new title.
    ///
    /// # Arguments
    ///
    /// * `value` - Title for the result.
    pub fn with_title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.title = Some(value.into());
        self
    }

    /// Sets a new MIME type.
    ///
    /// # Arguments
    ///
    /// * `value` - MIME type of the thumbnail; default - “image/jpeg”.
    ///
    /// Must be one of “image/jpeg”, “image/gif”, or “video/mp4”.
    pub fn with_thumbnail_mime_type<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.thumbnail_mime_type = Some(value.into());
        self
    }
}

/// Link to an animated GIF file stored on the Telegram servers.
///
/// By default, this animated GIF file will be sent by the user with an optional caption.
/// Alternatively, you can use [`Self::with_input_message_content`] to send
/// a message with specified content instead of the animation.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultCachedGif {
    gif_file_id: String,
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

impl InlineQueryResultCachedGif {
    /// Creates a new `InlineQueryResultCachedGif`.
    ///
    /// # Arguments
    ///
    /// * `gif_file_id` - A valid file identifier for the GIF file.
    /// * `id` - Unique identifier for this result; 1-64 bytes.
    pub fn new<A, B>(gif_file_id: A, id: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            gif_file_id: gif_file_id.into(),
            id: id.into(),
            caption: None,
            caption_entities: None,
            input_message_content: None,
            parse_mode: None,
            reply_markup: None,
            title: None,
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

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the GIF animation.
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

    /// Sets a new title.
    ///
    /// # Arguments
    ///
    /// * `value` - Title for the result.
    pub fn with_title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.title = Some(value.into());
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultGif {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            gif_duration: value.data.gif_duration,
            gif_height: value.data.gif_height,
            gif_url: value.data.gif_url.ok_or(MissingField("gif_url"))?,
            gif_width: value.data.gif_width,
            id: value.id,
            input_message_content: value.data.input_message_content,
            parse_mode: value.data.parse_mode,
            reply_markup: value.data.reply_markup,
            thumbnail_mime_type: value.data.thumbnail_mime_type,
            thumbnail_url: value.data.thumbnail_url.ok_or(MissingField("thumbnail_url"))?,
            title: value.data.title,
        })
    }
}

impl From<InlineQueryResultGif> for RawInlineQueryResult {
    fn from(value: InlineQueryResultGif) -> Self {
        Self {
            data: RawInlineQueryResultData {
                caption: value.caption,
                caption_entities: value.caption_entities,
                gif_duration: value.gif_duration,
                gif_height: value.gif_height,
                gif_url: Some(value.gif_url),
                gif_width: value.gif_width,
                input_message_content: value.input_message_content,
                parse_mode: value.parse_mode,
                reply_markup: value.reply_markup,
                thumbnail_mime_type: value.thumbnail_mime_type,
                thumbnail_url: Some(value.thumbnail_url),
                title: value.title,
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::Gif,
        }
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultCachedGif {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            gif_file_id: value.data.gif_file_id.ok_or(MissingField("gif_file_id"))?,
            id: value.id,
            input_message_content: value.data.input_message_content,
            parse_mode: value.data.parse_mode,
            reply_markup: value.data.reply_markup,
            title: value.data.title,
        })
    }
}

impl From<InlineQueryResultCachedGif> for RawInlineQueryResult {
    fn from(value: InlineQueryResultCachedGif) -> Self {
        Self {
            data: RawInlineQueryResultData {
                caption: value.caption,
                caption_entities: value.caption_entities,
                gif_file_id: Some(value.gif_file_id),
                input_message_content: value.input_message_content,
                parse_mode: value.parse_mode,
                reply_markup: value.reply_markup,
                title: value.title,
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::CachedGif,
        }
    }
}
