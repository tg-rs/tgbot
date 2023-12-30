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

/// Represents a link to an mp3 audio file.
///
/// By default, this audio file will be sent by the user.
/// Alternatively, you can use [`Self::with_input_message_content`] to send
/// a message with the specified content instead of the audio.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultAudio {
    audio_url: String,
    id: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl InlineQueryResultAudio {
    /// Creates a new `InlineQueryResultAudio`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier; 1-64 bytes.
    /// * `audio_url` - A valid URL of the audio file.
    /// * `title` - Title.
    pub fn new<A, B, C>(audio_url: A, id: B, title: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            audio_url: audio_url.into(),
            id: id.into(),
            title: title.into(),
            audio_duration: None,
            caption: None,
            caption_entities: None,
            input_message_content: None,
            parse_mode: None,
            performer: None,
            reply_markup: None,
        }
    }

    /// Sets a new audio duration.
    ///
    /// # Arguments
    ///
    /// * `value` - Audio duration in seconds.
    pub fn with_audio_duration(mut self, value: Integer) -> Self {
        self.audio_duration = Some(value);
        self
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
    /// * `value` - Content of the message to be sent instead of the audio.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        self.input_message_content = Some(value.into());
        self
    }

    /// Sets a new performer.
    ///
    /// # Arguments
    ///
    /// * `value` - Performer.
    pub fn with_performer<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.performer = Some(value.into());
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

/// Represents a link to an mp3 audio file stored on the Telegram servers.
///
/// By default, this audio file will be sent by the user.
/// Alternatively, you can use [Self::with_input_message_content]
/// to send a message with the specified content instead of the audio.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultCachedAudio {
    id: String,
    audio_file_id: String,
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
}

impl InlineQueryResultCachedAudio {
    /// Creates a new `InlineQueryResultCachedAudio`.
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier; 1-64 bytes.
    /// * `audio_file_id` - A valid file identifier for the audio file.
    pub fn new<A, B>(audio_file_id: A, id: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            audio_file_id: audio_file_id.into(),
            id: id.into(),
            caption: None,
            caption_entities: None,
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
    /// * value - Content of the message to be sent instead of the audio.
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
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultAudio {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            audio_duration: value.data.audio_duration,
            audio_url: value.data.audio_url.ok_or(MissingField("audio_url"))?,
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            id: value.id,
            input_message_content: value.data.input_message_content,
            parse_mode: value.data.parse_mode,
            performer: value.data.performer,
            reply_markup: value.data.reply_markup,
            title: value.data.title.ok_or(MissingField("title"))?,
        })
    }
}

impl From<InlineQueryResultAudio> for RawInlineQueryResult {
    fn from(value: InlineQueryResultAudio) -> Self {
        Self {
            data: RawInlineQueryResultData {
                audio_duration: value.audio_duration,
                audio_url: Some(value.audio_url),
                caption: value.caption,
                caption_entities: value.caption_entities,
                input_message_content: value.input_message_content,
                parse_mode: value.parse_mode,
                performer: value.performer,
                reply_markup: value.reply_markup,
                title: Some(value.title),
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::Audio,
        }
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultCachedAudio {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            audio_file_id: value.data.audio_file_id.ok_or(MissingField("audio_file_id"))?,
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            id: value.id,
            input_message_content: value.data.input_message_content,
            parse_mode: value.data.parse_mode,
            reply_markup: value.data.reply_markup,
        })
    }
}

impl From<InlineQueryResultCachedAudio> for RawInlineQueryResult {
    fn from(value: InlineQueryResultCachedAudio) -> Self {
        Self {
            data: RawInlineQueryResultData {
                audio_file_id: Some(value.audio_file_id),
                caption: value.caption,
                caption_entities: value.caption_entities,
                input_message_content: value.input_message_content,
                parse_mode: value.parse_mode,
                reply_markup: value.reply_markup,
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::CachedAudio,
        }
    }
}
