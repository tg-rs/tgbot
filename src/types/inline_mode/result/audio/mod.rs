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

/// Link to an mp3 audio file
///
/// By default, this audio file will be sent by the user
/// Alternatively, you can use input_message_content to send
/// a message with the specified content instead of the audio
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultAudio {
    id: String,
    audio_url: String,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    performer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
}

impl InlineQueryResultAudio {
    /// Creates a new InlineQueryResultAudio with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * audio_url - A valid URL for the audio file
    /// * title - Title
    pub fn new<I, U, T>(id: I, audio_url: U, title: T) -> Self
    where
        I: Into<String>,
        U: Into<String>,
        T: Into<String>,
    {
        InlineQueryResultAudio {
            id: id.into(),
            audio_url: audio_url.into(),
            title: title.into(),
            caption: None,
            caption_entities: None,
            parse_mode: None,
            performer: None,
            audio_duration: None,
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

    /// Performer
    pub fn performer<S: Into<String>>(mut self, performer: S) -> Self {
        self.performer = Some(performer.into());
        self
    }

    /// Audio duration in seconds
    pub fn audio_duration(mut self, audio_duration: Integer) -> Self {
        self.audio_duration = Some(audio_duration);
        self
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// Content of the message to be sent instead of the audio
    pub fn input_message_content<C: Into<InputMessageContent>>(mut self, input_message_content: C) -> Self {
        self.input_message_content = Some(input_message_content.into());
        self
    }
}

/// Link to an mp3 audio file stored on the Telegram servers
///
/// By default, this audio file will be sent by the user
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the audio
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultCachedAudio {
    id: String,
    audio_file_id: String,
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

impl InlineQueryResultCachedAudio {
    /// Creates a new InlineQueryResultCachedAudio with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * audio_file_id - A valid file identifier for the audio file
    pub fn new<I, F>(id: I, audio_file_id: F) -> Self
    where
        I: Into<String>,
        F: Into<String>,
    {
        InlineQueryResultCachedAudio {
            id: id.into(),
            audio_file_id: audio_file_id.into(),
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

    /// Content of the message to be sent instead of the audio
    pub fn input_message_content<C: Into<InputMessageContent>>(mut self, input_message_content: C) -> Self {
        self.input_message_content = Some(input_message_content.into());
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultAudio {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            audio_url: value.data.audio_url.ok_or(MissingField("audio_url"))?,
            title: value.data.title.ok_or(MissingField("title"))?,
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            parse_mode: value.data.parse_mode,
            performer: value.data.performer,
            audio_duration: value.data.audio_duration,
            reply_markup: value.data.reply_markup,
            input_message_content: value.data.input_message_content,
        })
    }
}

impl From<InlineQueryResultAudio> for RawInlineQueryResult {
    fn from(value: InlineQueryResultAudio) -> Self {
        Self {
            data: RawInlineQueryResultData {
                audio_url: Some(value.audio_url),
                title: Some(value.title),
                caption: value.caption,
                caption_entities: value.caption_entities,
                parse_mode: value.parse_mode,
                performer: value.performer,
                audio_duration: value.audio_duration,
                reply_markup: value.reply_markup,
                input_message_content: value.input_message_content,
                ..Default::default()
            },
            id: value.id,
            kind: RawInlineQueryResultKind::Audio,
        }
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultCachedAudio {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            audio_file_id: value.data.audio_file_id.ok_or(MissingField("audio_file_id"))?,
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            parse_mode: value.data.parse_mode,
            reply_markup: value.data.reply_markup,
            input_message_content: value.data.input_message_content,
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
                parse_mode: value.parse_mode,
                reply_markup: value.reply_markup,
                input_message_content: value.input_message_content,
                ..Default::default()
            },
            id: value.id,
            kind: RawInlineQueryResultKind::CachedAudio,
        }
    }
}
