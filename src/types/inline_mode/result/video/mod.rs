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

/// Link to a page containing an embedded video player or a video file
///
/// By default, this video file will be sent by the user with an optional caption
/// Alternatively, you can use input_message_content to send a message with
/// the specified content instead of the video
/// If an InlineQueryResultVideo message contains an embedded video (e.g., YouTube),
/// you must replace its content using input_message_content
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultVideo {
    id: String,
    video_url: String,
    mime_type: String,
    thumbnail_url: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    video_duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultVideo {
    /// Creates a new InlineQueryResultVideo with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * video_url - A valid URL for the embedded video player or video file
    /// * mime_type - Mime type of the content of video url, “text/html” or “video/mp4”
    /// * thumbnail_url - URL of the thumbnail (jpeg only) for the video
    /// * title - Title for the result
    pub fn new<A, B, C, D, E>(id: A, video_url: B, mime_type: C, thumbnail_url: D, title: E) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
        D: Into<String>,
        E: Into<String>,
    {
        InlineQueryResultVideo {
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
            input_message_content: None,
        }
    }

    /// Caption of the video to be sent, 0-1024 characters
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

    /// Video width
    pub fn video_width(mut self, video_width: Integer) -> Self {
        self.video_width = Some(video_width);
        self
    }

    /// Video height
    pub fn video_height(mut self, video_height: Integer) -> Self {
        self.video_height = Some(video_height);
        self
    }

    /// Video duration in seconds
    pub fn video_duration(mut self, video_duration: Integer) -> Self {
        self.video_duration = Some(video_duration);
        self
    }

    /// Short description of the result
    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// Content of the message to be sent instead of the video
    ///
    /// This field is required if InlineQueryResultVideo is used
    /// to send an HTML-page as a result (e.g., a YouTube video)
    pub fn input_message_content<C: Into<InputMessageContent>>(mut self, input_message_content: C) -> Self {
        self.input_message_content = Some(input_message_content.into());
        self
    }
}

/// Link to a video file stored on the Telegram servers
///
/// By default, this video file will be sent by the user with an optional caption
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the video
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultCachedVideo {
    id: String,
    video_file_id: String,
    title: String,
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

impl InlineQueryResultCachedVideo {
    /// Creates a new InlineQueryResultCachedVideo with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * video_file_id - A valid file identifier of the video
    /// * title - Title for the result
    pub fn new<I, F, T>(id: I, video_file_id: F, title: T) -> Self
    where
        I: Into<String>,
        F: Into<String>,
        T: Into<String>,
    {
        InlineQueryResultCachedVideo {
            id: id.into(),
            video_file_id: video_file_id.into(),
            title: title.into(),
            description: None,
            caption: None,
            caption_entities: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    /// Short description of the result
    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Caption of the video to be sent, 0-1024 characters
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

    /// Content of the message to be sent instead of the video
    pub fn input_message_content<C: Into<InputMessageContent>>(mut self, input_message_content: C) -> Self {
        self.input_message_content = Some(input_message_content.into());
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultVideo {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            video_url: value.data.video_url.ok_or(MissingField("video_url"))?,
            mime_type: value.data.mime_type.ok_or(MissingField("mime_type"))?,
            thumbnail_url: value.data.thumbnail_url.ok_or(MissingField("thumbnail_url"))?,
            title: value.data.title.ok_or(MissingField("title"))?,
            caption: value.data.caption,
            parse_mode: value.data.parse_mode,
            video_width: value.data.video_width,
            video_height: value.data.video_height,
            video_duration: value.data.video_duration,
            description: value.data.description,
            caption_entities: value.data.caption_entities,
            reply_markup: value.data.reply_markup,
            input_message_content: value.data.input_message_content,
        })
    }
}

impl From<InlineQueryResultVideo> for RawInlineQueryResult {
    fn from(value: InlineQueryResultVideo) -> Self {
        Self {
            data: RawInlineQueryResultData {
                video_url: Some(value.video_url),
                mime_type: Some(value.mime_type),
                thumbnail_url: Some(value.thumbnail_url),
                title: Some(value.title),
                caption: value.caption,
                parse_mode: value.parse_mode,
                video_width: value.video_width,
                video_height: value.video_height,
                video_duration: value.video_duration,
                description: value.description,
                caption_entities: value.caption_entities,
                reply_markup: value.reply_markup,
                input_message_content: value.input_message_content,
                ..Default::default()
            },
            id: value.id,
            kind: RawInlineQueryResultKind::Video,
        }
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultCachedVideo {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            video_file_id: value.data.video_file_id.ok_or(MissingField("video_file_id"))?,
            title: value.data.title.ok_or(MissingField("title"))?,
            description: value.data.description,
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            parse_mode: value.data.parse_mode,
            reply_markup: value.data.reply_markup,
            input_message_content: value.data.input_message_content,
        })
    }
}

impl From<InlineQueryResultCachedVideo> for RawInlineQueryResult {
    fn from(value: InlineQueryResultCachedVideo) -> Self {
        Self {
            data: RawInlineQueryResultData {
                video_file_id: Some(value.video_file_id),
                title: Some(value.title),
                description: value.description,
                caption: value.caption,
                caption_entities: value.caption_entities,
                parse_mode: value.parse_mode,
                reply_markup: value.reply_markup,
                input_message_content: value.input_message_content,
                ..Default::default()
            },
            id: value.id,
            kind: RawInlineQueryResultKind::CachedVideo,
        }
    }
}
