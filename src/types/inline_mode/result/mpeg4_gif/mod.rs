use serde::{Deserialize, Serialize};

use crate::types::{InlineKeyboardMarkup, InputMessageContent, Integer, ParseMode, TextEntities, TextEntity};

use super::raw::{
    RawInlineQueryResult,
    RawInlineQueryResultData,
    RawInlineQueryResultDataError::{self, MissingField},
    RawInlineQueryResultKind,
};

#[cfg(test)]
mod tests;

/// Link to a video animation (H.264/MPEG-4 AVC video without sound)
///
/// By default, this animated MPEG-4 file will be sent by the user with optional caption
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the animation
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultMpeg4Gif {
    id: String,
    mpeg4_url: String,
    thumbnail_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mpeg4_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mpeg4_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mpeg4_duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
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

impl InlineQueryResultMpeg4Gif {
    /// Creates a new InlineQueryResultMpeg4Gif with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * mpeg4_url - A valid URL for the MP4 file. File size must not exceed 1MB
    /// * thumbnail_url - URL of the static thumbnail (jpeg or gif) for the result
    pub fn new<I, U, T>(id: I, mpeg4_url: U, thumbnail_url: T) -> Self
    where
        I: Into<String>,
        U: Into<String>,
        T: Into<String>,
    {
        InlineQueryResultMpeg4Gif {
            id: id.into(),
            mpeg4_url: mpeg4_url.into(),
            mpeg4_width: None,
            mpeg4_height: None,
            mpeg4_duration: None,
            thumbnail_url: thumbnail_url.into(),
            thumbnail_mime_type: None,
            title: None,
            caption: None,
            caption_entities: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    /// MIME type of the thumbnail
    ///
    /// Must be one of “image/jpeg”, “image/gif”, or “video/mp4”
    ///
    /// Defaults to “image/jpeg”
    pub fn thumbnail_mime_type<S: Into<String>>(mut self, mime_type: S) -> Self {
        self.thumbnail_mime_type = Some(mime_type.into());
        self
    }

    /// Video width
    pub fn mpeg4_width(mut self, mpeg4_width: Integer) -> Self {
        self.mpeg4_width = Some(mpeg4_width);
        self
    }

    /// Video height
    pub fn mpeg4_height(mut self, mpeg4_height: Integer) -> Self {
        self.mpeg4_height = Some(mpeg4_height);
        self
    }

    /// Video duration
    pub fn mpeg4_duration(mut self, mpeg4_duration: Integer) -> Self {
        self.mpeg4_duration = Some(mpeg4_duration);
        self
    }

    /// Title for the result
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Caption of the MPEG-4 file to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// List of special entities that appear in the caption
    ///
    /// Parse mode will be set to None when this method is called
    pub fn caption_entities<T>(mut self, caption_entities: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.caption_entities = Some(caption_entities.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets parse mode
    ///
    /// Caption entities will be set to None when this method is called
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self.caption_entities = None;
        self
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// Content of the message to be sent instead of the video animation
    pub fn input_message_content<C: Into<InputMessageContent>>(mut self, input_message_content: C) -> Self {
        self.input_message_content = Some(input_message_content.into());
        self
    }
}

/// Link to a video animation (H.264/MPEG-4 AVC video without sound) stored on the Telegram servers
///
/// By default, this animated MPEG-4 file will be sent by the user with an optional caption
/// Alternatively, you can use input_message_content
/// to send a message with the specified content
/// instead of the animation
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultCachedMpeg4Gif {
    id: String,
    mpeg4_file_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<String>,
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

impl InlineQueryResultCachedMpeg4Gif {
    /// Creates a new InlineQueryResultCachedMpeg4Gif with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * mpeg4_file_id - A valid file identifier for the MP4 file
    pub fn new<I, F>(id: I, mpeg4_file_id: F) -> Self
    where
        I: Into<String>,
        F: Into<String>,
    {
        InlineQueryResultCachedMpeg4Gif {
            id: id.into(),
            mpeg4_file_id: mpeg4_file_id.into(),
            title: None,
            caption: None,
            caption_entities: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    /// Title for the result
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Caption of the MPEG-4 file to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// List of special entities that appear in the caption
    ///
    /// Parse mode will be set to None when this method is called
    pub fn caption_entities<T>(mut self, caption_entities: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.caption_entities = Some(caption_entities.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets parse mode
    ///
    /// Caption entities will be set to None when this method is called
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self.caption_entities = None;
        self
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// Content of the message to be sent instead of the video animation
    pub fn input_message_content<C: Into<InputMessageContent>>(mut self, input_message_content: C) -> Self {
        self.input_message_content = Some(input_message_content.into());
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultMpeg4Gif {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            mpeg4_url: value.data.mpeg4_url.ok_or(MissingField("mpeg4_url"))?,
            thumbnail_url: value.data.thumbnail_url.ok_or(MissingField("thumbnail_url"))?,
            thumbnail_mime_type: value.data.thumbnail_mime_type,
            mpeg4_width: value.data.mpeg4_width,
            mpeg4_height: value.data.mpeg4_height,
            mpeg4_duration: value.data.mpeg4_duration,
            title: value.data.title,
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            parse_mode: value.data.parse_mode,
            reply_markup: value.data.reply_markup,
            input_message_content: value.data.input_message_content,
        })
    }
}

impl From<InlineQueryResultMpeg4Gif> for RawInlineQueryResult {
    fn from(value: InlineQueryResultMpeg4Gif) -> Self {
        Self {
            data: RawInlineQueryResultData {
                mpeg4_url: Some(value.mpeg4_url),
                thumbnail_url: Some(value.thumbnail_url),
                thumbnail_mime_type: value.thumbnail_mime_type,
                mpeg4_width: value.mpeg4_width,
                mpeg4_height: value.mpeg4_height,
                mpeg4_duration: value.mpeg4_duration,
                title: value.title,
                caption: value.caption,
                caption_entities: value.caption_entities,
                parse_mode: value.parse_mode,
                reply_markup: value.reply_markup,
                input_message_content: value.input_message_content,
                ..Default::default()
            },
            id: value.id,
            kind: RawInlineQueryResultKind::Mpeg4Gif,
        }
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultCachedMpeg4Gif {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            mpeg4_file_id: value.data.mpeg4_file_id.ok_or(MissingField("mpeg4_file_id"))?,
            title: value.data.title,
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            parse_mode: value.data.parse_mode,
            reply_markup: value.data.reply_markup,
            input_message_content: value.data.input_message_content,
        })
    }
}

impl From<InlineQueryResultCachedMpeg4Gif> for RawInlineQueryResult {
    fn from(value: InlineQueryResultCachedMpeg4Gif) -> Self {
        Self {
            data: RawInlineQueryResultData {
                mpeg4_file_id: Some(value.mpeg4_file_id),
                title: value.title,
                caption: value.caption,
                caption_entities: value.caption_entities,
                parse_mode: value.parse_mode,
                reply_markup: value.reply_markup,
                input_message_content: value.input_message_content,
                ..Default::default()
            },
            id: value.id,
            kind: RawInlineQueryResultKind::CachedMpeg4Gif,
        }
    }
}
