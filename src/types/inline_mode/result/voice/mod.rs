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

/// Represents a link to a voice recording in an OGG container encoded with OPUS.
///
/// By default, this voice recording will be sent by the user.
/// Alternatively, you can use [`Self::with_input_message_content`] to send
/// a message with the specified content instead of the the voice message.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultVoice {
    id: String,
    title: String,
    voice_url: String,
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
    voice_duration: Option<Integer>,
}

impl InlineQueryResultVoice {
    /// Creates a new `InlineQueryResultVoice`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `title` - Recording title.
    /// * `voice_url` - A valid URL of the voice recording.
    pub fn new<A, B, C>(id: A, title: B, voice_url: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            id: id.into(),
            title: title.into(),
            voice_url: voice_url.into(),
            caption: None,
            caption_entities: None,
            input_message_content: None,
            parse_mode: None,
            reply_markup: None,
            voice_duration: None,
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
    /// * `value` - Content of the message to be sent instead of the voice recording.
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

    /// Sets a new voice duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Recording duration in seconds.
    pub fn with_voice_duration(mut self, value: Integer) -> Self {
        self.voice_duration = Some(value);
        self
    }
}

/// Represents a link to a voice message stored on the Telegram servers.
///
/// By default, this voice message will be sent by the user.
/// Alternatively, you can use [`Self::with_input_message_content`]
/// to send a message with the specified content instead of the voice message.
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
    /// Creates a new `InlineQueryResultCachedVoice`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `title` - Title of the result.
    /// * `voice_file_id` - A valid file identifier of the voice message.
    pub fn new<A, B, C>(id: A, title: B, voice_file_id: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
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

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the voice message.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        self.input_message_content = Some(value.into());
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultVoice {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            input_message_content: value.data.input_message_content,
            parse_mode: value.data.parse_mode,
            reply_markup: value.data.reply_markup,
            title: value.data.title.ok_or(MissingField("title"))?,
            voice_duration: value.data.voice_duration,
            voice_url: value.data.voice_url.ok_or(MissingField("voice_url"))?,
        })
    }
}

impl From<InlineQueryResultVoice> for RawInlineQueryResult {
    fn from(value: InlineQueryResultVoice) -> Self {
        Self {
            data: RawInlineQueryResultData {
                caption: value.caption,
                caption_entities: value.caption_entities,
                input_message_content: value.input_message_content,
                parse_mode: value.parse_mode,
                reply_markup: value.reply_markup,
                title: Some(value.title),
                voice_duration: value.voice_duration,
                voice_url: Some(value.voice_url),
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::Voice,
        }
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultCachedVoice {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            id: value.id,
            input_message_content: value.data.input_message_content,
            parse_mode: value.data.parse_mode,
            reply_markup: value.data.reply_markup,
            title: value.data.title.ok_or(MissingField("title"))?,
            voice_file_id: value.data.voice_file_id.ok_or(MissingField("voice_file_id"))?,
        })
    }
}

impl From<InlineQueryResultCachedVoice> for RawInlineQueryResult {
    fn from(value: InlineQueryResultCachedVoice) -> Self {
        Self {
            data: RawInlineQueryResultData {
                caption: value.caption,
                caption_entities: value.caption_entities,
                input_message_content: value.input_message_content,
                parse_mode: value.parse_mode,
                reply_markup: value.reply_markup,
                title: Some(value.title),
                voice_file_id: Some(value.voice_file_id),
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::CachedVoice,
        }
    }
}
