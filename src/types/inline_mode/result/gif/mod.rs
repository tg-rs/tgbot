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

/// Link to an animated GIF file
///
/// By default, this animated GIF file
/// will be sent by the user with optional caption
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the animation
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultGif {
    id: String,
    gif_url: String,
    thumbnail_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_mime_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gif_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gif_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    gif_duration: Option<Integer>,
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

impl InlineQueryResultGif {
    /// Creates a new InlineQueryResultGif with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * gif_url - A valid URL for the GIF file. File size must not exceed 1MB
    /// * thumbnail_url - URL of the static thumbnail for the result (jpeg or gif)
    pub fn new<I, U, T>(id: I, gif_url: U, thumbnail_url: T) -> Self
    where
        I: Into<String>,
        U: Into<String>,
        T: Into<String>,
    {
        InlineQueryResultGif {
            id: id.into(),
            gif_url: gif_url.into(),
            gif_width: None,
            gif_height: None,
            gif_duration: None,
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

    /// Width of the GIF
    pub fn gif_width(mut self, gif_width: Integer) -> Self {
        self.gif_width = Some(gif_width);
        self
    }

    /// Height of the GIF
    pub fn gif_height(mut self, gif_height: Integer) -> Self {
        self.gif_height = Some(gif_height);
        self
    }

    /// Duration of the GIF
    pub fn gif_duration(mut self, gif_duration: Integer) -> Self {
        self.gif_duration = Some(gif_duration);
        self
    }

    /// Title for the result
    pub fn title<S: Into<String>>(mut self, title: S) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Caption of the GIF file to be sent, 0-1024 characters
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

    /// Content of the message to be sent instead of the GIF animation
    pub fn input_message_content<C: Into<InputMessageContent>>(mut self, input_message_content: C) -> Self {
        self.input_message_content = Some(input_message_content.into());
        self
    }
}

/// Link to an animated GIF file stored on the Telegram servers
///
/// By default, this animated GIF file will be sent by the user with an optional caption
/// Alternatively, you can use input_message_content to send
/// a message with specified content instead of the animation
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultCachedGif {
    id: String,
    gif_file_id: String,
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

impl InlineQueryResultCachedGif {
    /// Creates a new InlineQueryResultCachedGif with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * gif_file_id - A valid file identifier for the GIF file
    pub fn new<I, F>(id: I, gif_file_id: F) -> Self
    where
        I: Into<String>,
        F: Into<String>,
    {
        InlineQueryResultCachedGif {
            id: id.into(),
            gif_file_id: gif_file_id.into(),
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

    /// Caption of the GIF file to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// List of special entities that appear in the caption,
    /// which can be specified instead of parse_mode
    pub fn caption_entities<T>(mut self, caption_entities: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.caption_entities = Some(caption_entities.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Parse mode
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

    /// Content of the message to be sent instead of the GIF animation
    pub fn input_message_content<C: Into<InputMessageContent>>(mut self, input_message_content: C) -> Self {
        self.input_message_content = Some(input_message_content.into());
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultGif {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            gif_url: value.data.gif_url.ok_or(MissingField("gif_url"))?,
            thumbnail_url: value.data.thumbnail_url.ok_or(MissingField("thumbnail_url"))?,
            thumbnail_mime_type: value.data.thumbnail_mime_type,
            gif_width: value.data.gif_width,
            gif_height: value.data.gif_height,
            gif_duration: value.data.gif_duration,
            title: value.data.title,
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            parse_mode: value.data.parse_mode,
            reply_markup: value.data.reply_markup,
            input_message_content: value.data.input_message_content,
        })
    }
}

impl From<InlineQueryResultGif> for RawInlineQueryResult {
    fn from(value: InlineQueryResultGif) -> Self {
        Self {
            data: RawInlineQueryResultData {
                gif_url: Some(value.gif_url),
                thumbnail_url: Some(value.thumbnail_url),
                thumbnail_mime_type: value.thumbnail_mime_type,
                gif_width: value.gif_width,
                gif_height: value.gif_height,
                gif_duration: value.gif_duration,
                title: value.title,
                caption: value.caption,
                caption_entities: value.caption_entities,
                parse_mode: value.parse_mode,
                reply_markup: value.reply_markup,
                input_message_content: value.input_message_content,
                ..Default::default()
            },
            id: value.id,
            kind: RawInlineQueryResultKind::Gif,
        }
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultCachedGif {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            gif_file_id: value.data.gif_file_id.ok_or(MissingField("gif_file_id"))?,
            title: value.data.title,
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            parse_mode: value.data.parse_mode,
            reply_markup: value.data.reply_markup,
            input_message_content: value.data.input_message_content,
        })
    }
}

impl From<InlineQueryResultCachedGif> for RawInlineQueryResult {
    fn from(value: InlineQueryResultCachedGif) -> Self {
        Self {
            data: RawInlineQueryResultData {
                gif_file_id: Some(value.gif_file_id),
                title: value.title,
                caption: value.caption,
                caption_entities: value.caption_entities,
                parse_mode: value.parse_mode,
                reply_markup: value.reply_markup,
                input_message_content: value.input_message_content,
                ..Default::default()
            },
            id: value.id,
            kind: RawInlineQueryResultKind::CachedGif,
        }
    }
}
