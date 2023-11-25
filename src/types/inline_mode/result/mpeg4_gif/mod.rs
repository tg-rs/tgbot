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

/// Represents a link to a video animation (H.264/MPEG-4 AVC video without sound).
///
/// By default, this animated MPEG-4 file will be sent by the user with optional caption.
/// Alternatively, you can use [`Self::with_input_message_content`]
/// to send a message with the specified content instead of the animation.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultMpeg4Gif {
    id: String,
    mpeg4_url: String,
    thumbnail_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mpeg4_duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mpeg4_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mpeg4_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
}

impl InlineQueryResultMpeg4Gif {
    /// Creates a new `InlineQueryResultMpeg4Gif`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for the result; 1-64 bytes.
    /// * `mpeg4_url` - A valid URL for the MP4 file; file size must not exceed 1MB.
    /// * `thumbnail_url` - URL of the static thumbnail (jpeg or gif) for the result.
    pub fn new<A, B, C>(id: A, mpeg4_url: B, thumbnail_url: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            id: id.into(),
            mpeg4_url: mpeg4_url.into(),
            thumbnail_url: thumbnail_url.into(),
            caption: None,
            caption_entities: None,
            input_message_content: None,
            mpeg4_duration: None,
            mpeg4_height: None,
            mpeg4_width: None,
            parse_mode: None,
            reply_markup: None,
            thumbnail_mime_type: None,
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
    /// * `value` - Content of the message to be sent instead of the video animation.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        self.input_message_content = Some(value.into());
        self
    }

    /// Sets a new MPEG4 duration.
    ///
    /// # Arguments
    ///
    /// * `value` - MPEG4 duration.
    pub fn with_mpeg4_duration(mut self, value: Integer) -> Self {
        self.mpeg4_duration = Some(value);
        self
    }

    /// Sets a new MPEG4 height.
    ///
    /// # Arguments
    ///
    /// * `value` - MPEG4 height.
    pub fn with_mpeg4_height(mut self, value: Integer) -> Self {
        self.mpeg4_height = Some(value);
        self
    }

    /// Sets a new MPEG4 width.
    ///
    /// # Arguments
    ///
    /// * `value` - MPEG4 width.
    pub fn with_mpeg4_width(mut self, value: Integer) -> Self {
        self.mpeg4_width = Some(value);
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

    /// Sets a new thumbnail MIME type.
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

    /// Sets a new title.
    ///
    /// # Arguments
    ///
    /// * `value` - Title of the result.
    pub fn with_title<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.title = Some(value.into());
        self
    }
}

/// Represents a link to a video animation
/// (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers.
///
/// By default, this animated MPEG-4 file will be sent by the user with an optional caption.
/// Alternatively, you can use [`Self::with_input_message_content`]
/// to send a message with the specified content
/// instead of the animation.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultCachedMpeg4Gif {
    id: String,
    mpeg4_file_id: String,
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

impl InlineQueryResultCachedMpeg4Gif {
    /// Creates a new `InlineQueryResultCachedMpeg4Gif`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `mpeg4_file_id` - A valid file identifier for the MP4 file.
    pub fn new<A, B>(id: A, mpeg4_file_id: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            id: id.into(),
            mpeg4_file_id: mpeg4_file_id.into(),
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

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the video animation.
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

    /// Sets a new title.
    ///
    /// # Arguments
    ///
    /// * `value` - Title for the result.
    pub fn with_title<T>(mut self, title: T) -> Self
    where
        T: Into<String>,
    {
        self.title = Some(title.into());
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultMpeg4Gif {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            input_message_content: value.data.input_message_content,
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            mpeg4_url: value.data.mpeg4_url.ok_or(MissingField("mpeg4_url"))?,
            mpeg4_width: value.data.mpeg4_width,
            mpeg4_height: value.data.mpeg4_height,
            mpeg4_duration: value.data.mpeg4_duration,
            parse_mode: value.data.parse_mode,
            reply_markup: value.data.reply_markup,
            thumbnail_mime_type: value.data.thumbnail_mime_type,
            thumbnail_url: value.data.thumbnail_url.ok_or(MissingField("thumbnail_url"))?,
            title: value.data.title,
        })
    }
}

impl From<InlineQueryResultMpeg4Gif> for RawInlineQueryResult {
    fn from(value: InlineQueryResultMpeg4Gif) -> Self {
        Self {
            data: RawInlineQueryResultData {
                caption: value.caption,
                caption_entities: value.caption_entities,
                input_message_content: value.input_message_content,
                mpeg4_duration: value.mpeg4_duration,
                mpeg4_height: value.mpeg4_height,
                mpeg4_width: value.mpeg4_width,
                mpeg4_url: Some(value.mpeg4_url),
                parse_mode: value.parse_mode,
                reply_markup: value.reply_markup,
                thumbnail_url: Some(value.thumbnail_url),
                thumbnail_mime_type: value.thumbnail_mime_type,
                title: value.title,
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::Mpeg4Gif,
        }
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultCachedMpeg4Gif {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            input_message_content: value.data.input_message_content,
            mpeg4_file_id: value.data.mpeg4_file_id.ok_or(MissingField("mpeg4_file_id"))?,
            parse_mode: value.data.parse_mode,
            reply_markup: value.data.reply_markup,
            title: value.data.title,
        })
    }
}

impl From<InlineQueryResultCachedMpeg4Gif> for RawInlineQueryResult {
    fn from(value: InlineQueryResultCachedMpeg4Gif) -> Self {
        Self {
            data: RawInlineQueryResultData {
                caption: value.caption,
                caption_entities: value.caption_entities,
                input_message_content: value.input_message_content,
                mpeg4_file_id: Some(value.mpeg4_file_id),
                parse_mode: value.parse_mode,
                reply_markup: value.reply_markup,
                title: value.title,
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::CachedMpeg4Gif,
        }
    }
}
