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

/// Represents a link to a page containing an embedded video player or a video file.
///
/// By default, this video file will be sent by the user with an optional caption.
/// Alternatively, you can use [`Self::with_input_message_content`] to send a message with
/// the specified content instead of the video.
/// If an [`InlineQueryResultVideo`] message contains an embedded video (e.g., YouTube),
/// you must replace its content using [`Self::with_input_message_content`].
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultVideo {
    id: String,
    mime_type: String,
    thumbnail_url: String,
    title: String,
    video_url: String,
    caption: Option<String>,
    caption_entities: Option<TextEntities>,
    description: Option<String>,
    input_message_content: Option<InputMessageContent>,
    parse_mode: Option<ParseMode>,
    reply_markup: Option<InlineKeyboardMarkup>,
    show_caption_above_media: Option<bool>,
    video_duration: Option<Integer>,
    video_height: Option<Integer>,
    video_width: Option<Integer>,
}

impl InlineQueryResultVideo {
    /// Creates a new `InlineQueryResultVideo`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `mime_type` - MIME type of the content of video url: “text/html” or “video/mp4”.
    /// * `thumbnail_url` - URL of the thumbnail for the video; JPEG only.
    /// * `title` - Title of the result.
    /// * `video_url` - A valid URL of the embedded video player or video file.
    pub fn new<A, B, C, D, E>(id: A, mime_type: B, thumbnail_url: C, title: D, video_url: E) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
        D: Into<String>,
        E: Into<String>,
    {
        Self {
            id: id.into(),
            video_url: video_url.into(),
            mime_type: mime_type.into(),
            thumbnail_url: thumbnail_url.into(),
            title: title.into(),
            caption: None,
            parse_mode: None,
            video_width: None,
            video_height: None,
            video_duration: None,
            description: None,
            caption_entities: None,
            reply_markup: None,
            show_caption_above_media: None,
            input_message_content: None,
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
    /// * `value` - Content of the message to be sent instead of the video.
    ///
    /// This field is required if [`InlineQueryResultVideo`] is used
    /// to send an HTML-page as a result (e.g., a YouTube video).
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        self.input_message_content = Some(value.into());
        self
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.show_caption_above_media = Some(value);
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

    /// Sets a new video duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Video duration in seconds.
    pub fn with_video_duration(mut self, value: Integer) -> Self {
        self.video_duration = Some(value);
        self
    }

    /// Sets a new video height.
    ///
    /// # Arguments
    ///
    /// * `value` - Video height.
    pub fn with_video_height(mut self, value: Integer) -> Self {
        self.video_height = Some(value);
        self
    }

    /// Sets a new vide width.
    ///
    /// # Arguments
    ///
    /// * `value` - Video width.
    pub fn with_video_width(mut self, value: Integer) -> Self {
        self.video_width = Some(value);
        self
    }
}

/// Represents a link to a video file stored on the Telegram servers.
///
/// By default, this video file will be sent by the user with an optional caption.
/// Alternatively, you can use [`Self::with_input_message_content`]
/// to send a message with the specified content instead of the video.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultCachedVideo {
    id: String,
    video_file_id: String,
    title: String,
    description: Option<String>,
    caption: Option<String>,
    caption_entities: Option<TextEntities>,
    parse_mode: Option<ParseMode>,
    reply_markup: Option<InlineKeyboardMarkup>,
    show_caption_above_media: Option<bool>,
    input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultCachedVideo {
    /// Creates a new `InlineQueryResultCachedVideo`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `title` - Title of the result.
    /// * `video_file_id` - A valid file identifier of the video.
    pub fn new<A, B, C>(id: A, title: B, video_file_id: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            id: id.into(),
            title: title.into(),
            video_file_id: video_file_id.into(),
            caption: None,
            caption_entities: None,
            description: None,
            input_message_content: None,
            parse_mode: None,
            reply_markup: None,
            show_caption_above_media: None,
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
    /// * `value` - Content of the message to be sent instead of the video.
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

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.show_caption_above_media = Some(value);
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultVideo {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            description: value.data.description,
            id: value.id,
            input_message_content: value.data.input_message_content,
            mime_type: value.data.mime_type.ok_or(MissingField("mime_type"))?,
            parse_mode: value.data.parse_mode,
            reply_markup: value.data.reply_markup,
            show_caption_above_media: value.data.show_caption_above_media,
            thumbnail_url: value.data.thumbnail_url.ok_or(MissingField("thumbnail_url"))?,
            title: value.data.title.ok_or(MissingField("title"))?,
            video_duration: value.data.video_duration,
            video_height: value.data.video_height,
            video_url: value.data.video_url.ok_or(MissingField("video_url"))?,
            video_width: value.data.video_width,
        })
    }
}

impl From<InlineQueryResultVideo> for RawInlineQueryResult {
    fn from(value: InlineQueryResultVideo) -> Self {
        Self {
            data: RawInlineQueryResultData {
                caption: value.caption,
                caption_entities: value.caption_entities,
                description: value.description,
                input_message_content: value.input_message_content,
                mime_type: Some(value.mime_type),
                parse_mode: value.parse_mode,
                reply_markup: value.reply_markup,
                show_caption_above_media: value.show_caption_above_media,
                thumbnail_url: Some(value.thumbnail_url),
                title: Some(value.title),
                video_duration: value.video_duration,
                video_height: value.video_height,
                video_url: Some(value.video_url),
                video_width: value.video_width,
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::Video,
        }
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultCachedVideo {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            description: value.data.description,
            input_message_content: value.data.input_message_content,
            parse_mode: value.data.parse_mode,
            reply_markup: value.data.reply_markup,
            show_caption_above_media: value.data.show_caption_above_media,
            title: value.data.title.ok_or(MissingField("title"))?,
            video_file_id: value.data.video_file_id.ok_or(MissingField("video_file_id"))?,
        })
    }
}

impl From<InlineQueryResultCachedVideo> for RawInlineQueryResult {
    fn from(value: InlineQueryResultCachedVideo) -> Self {
        Self {
            data: RawInlineQueryResultData {
                caption: value.caption,
                caption_entities: value.caption_entities,
                description: value.description,
                input_message_content: value.input_message_content,
                parse_mode: value.parse_mode,
                reply_markup: value.reply_markup,
                show_caption_above_media: value.show_caption_above_media,
                title: Some(value.title),
                video_file_id: Some(value.video_file_id),
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::CachedVideo,
        }
    }
}
