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

/// Link to a voice recording in an .ogg container encoded with OPUS
///
/// By default, this voice recording will be sent by the user
/// Alternatively, you can use input_message_content to send
/// a message with the specified content instead of the the voice message
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultVoice {
    id: String,
    voice_url: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    voice_duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultVoice {
    /// Creates a new InlineQueryResultVoice with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * voice_url - A valid URL for the voice recording
    /// * title - Recording title
    pub fn new<I, U, T>(id: I, voice_url: U, title: T) -> Self
    where
        I: Into<String>,
        U: Into<String>,
        T: Into<String>,
    {
        InlineQueryResultVoice {
            id: id.into(),
            voice_url: voice_url.into(),
            title: title.into(),
            caption: None,
            caption_entities: None,
            parse_mode: None,
            voice_duration: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    /// Caption, 0-1024 characters
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
        self
    }

    /// Recording duration in seconds
    pub fn voice_duration(mut self, voice_duration: Integer) -> Self {
        self.voice_duration = Some(voice_duration);
        self
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// Content of the message to be sent instead of the voice recording
    pub fn input_message_content<C: Into<InputMessageContent>>(mut self, input_message_content: C) -> Self {
        self.input_message_content = Some(input_message_content.into());
        self
    }
}

/// Link to a voice message stored on the Telegram servers
///
/// By default, this voice message will be sent by the user
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the voice message
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultCachedVoice {
    id: String,
    voice_file_id: String,
    title: String,
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

impl InlineQueryResultCachedVoice {
    /// Creates a new InlineQueryResultCachedVoice with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * voice_file_id - A valid file identifier for the voice message
    /// * title - Title for the result
    pub fn new<I, F, T>(id: I, voice_file_id: F, title: T) -> Self
    where
        I: Into<String>,
        F: Into<String>,
        T: Into<String>,
    {
        InlineQueryResultCachedVoice {
            id: id.into(),
            voice_file_id: voice_file_id.into(),
            title: title.into(),
            caption: None,
            caption_entities: None,
            parse_mode: None,
            reply_markup: None,
            input_message_content: None,
        }
    }

    /// Caption, 0-1024 characters
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

    /// Content of the message to be sent instead of the voice message
    pub fn input_message_content<C: Into<InputMessageContent>>(mut self, input_message_content: C) -> Self {
        self.input_message_content = Some(input_message_content.into());
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultVoice {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            voice_url: value.data.voice_url.ok_or(MissingField("voice_url"))?,
            title: value.data.title.ok_or(MissingField("title"))?,
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            parse_mode: value.data.parse_mode,
            voice_duration: value.data.voice_duration,
            reply_markup: value.data.reply_markup,
            input_message_content: value.data.input_message_content,
        })
    }
}

impl From<InlineQueryResultVoice> for RawInlineQueryResult {
    fn from(value: InlineQueryResultVoice) -> Self {
        Self {
            data: RawInlineQueryResultData {
                voice_url: Some(value.voice_url),
                title: Some(value.title),
                caption: value.caption,
                caption_entities: value.caption_entities,
                parse_mode: value.parse_mode,
                voice_duration: value.voice_duration,
                reply_markup: value.reply_markup,
                input_message_content: value.input_message_content,
                ..Default::default()
            },
            id: value.id,
            kind: RawInlineQueryResultKind::Voice,
        }
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultCachedVoice {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            voice_file_id: value.data.voice_file_id.ok_or(MissingField("voice_file_id"))?,
            title: value.data.title.ok_or(MissingField("title"))?,
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            parse_mode: value.data.parse_mode,
            reply_markup: value.data.reply_markup,
            input_message_content: value.data.input_message_content,
        })
    }
}

impl From<InlineQueryResultCachedVoice> for RawInlineQueryResult {
    fn from(value: InlineQueryResultCachedVoice) -> Self {
        Self {
            data: RawInlineQueryResultData {
                voice_file_id: Some(value.voice_file_id),
                title: Some(value.title),
                caption: value.caption,
                caption_entities: value.caption_entities,
                parse_mode: value.parse_mode,
                reply_markup: value.reply_markup,
                input_message_content: value.input_message_content,
                ..Default::default()
            },
            id: value.id,
            kind: RawInlineQueryResultKind::CachedVoice,
        }
    }
}
