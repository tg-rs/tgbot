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

/// Link to a file
///
/// By default, this file will be sent by the user with an optional caption
/// Alternatively, you can use input_message_content to send a message
/// with the specified content instead of the file
/// Currently, only .PDF and .ZIP files can be sent using this method
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultDocument {
    id: String,
    title: String,
    document_url: String,
    mime_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb_height: Option<Integer>,
}

impl InlineQueryResultDocument {
    /// Creates a new InlineQueryResultDocument with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * title - Title for the result
    /// * document_url - A valid URL for the file
    /// * mime_type - Mime type of the content of the file, either “application/pdf” or “application/zip”
    pub fn new<I, T, U, M>(id: I, title: T, document_url: U, mime_type: M) -> Self
    where
        I: Into<String>,
        T: Into<String>,
        U: Into<String>,
        M: Into<String>,
    {
        InlineQueryResultDocument {
            id: id.into(),
            title: title.into(),
            caption: None,
            caption_entities: None,
            parse_mode: None,
            document_url: document_url.into(),
            mime_type: mime_type.into(),
            description: None,
            reply_markup: None,
            input_message_content: None,
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
        }
    }

    /// Caption of the document to be sent, 0-1024 characters
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

    /// Content of the message to be sent instead of the file
    pub fn input_message_content<C: Into<InputMessageContent>>(mut self, input_message_content: C) -> Self {
        self.input_message_content = Some(input_message_content.into());
        self
    }

    /// URL of the thumbnail (jpeg only) for the file
    pub fn thumb_url<S: Into<String>>(mut self, thumb_url: S) -> Self {
        self.thumb_url = Some(thumb_url.into());
        self
    }

    /// Thumbnail width
    pub fn thumb_width(mut self, thumb_width: Integer) -> Self {
        self.thumb_width = Some(thumb_width);
        self
    }

    /// Thumbnail height
    pub fn thumb_height(mut self, thumb_height: Integer) -> Self {
        self.thumb_height = Some(thumb_height);
        self
    }
}

/// Link to a file stored on the Telegram servers
///
/// By default, this file will be sent by the user with an optional caption
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the file
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultCachedDocument {
    id: String,
    title: String,
    document_file_id: String,
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

impl InlineQueryResultCachedDocument {
    /// Creates a new InlineQueryResultCachedDocument with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * title - Title for the result
    /// * document_file_id - A valid file identifier for the file
    pub fn new<I, T, F>(id: I, title: T, document_file_id: F) -> Self
    where
        I: Into<String>,
        T: Into<String>,
        F: Into<String>,
    {
        InlineQueryResultCachedDocument {
            id: id.into(),
            title: title.into(),
            document_file_id: document_file_id.into(),
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

    /// Caption of the document to be sent, 0-1024 characters
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

    /// Content of the message to be sent instead of the file
    pub fn input_message_content<C: Into<InputMessageContent>>(mut self, input_message_content: C) -> Self {
        self.input_message_content = Some(input_message_content.into());
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultDocument {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            title: value.data.title.ok_or(MissingField("title"))?,
            document_url: value.data.document_url.ok_or(MissingField("document_url"))?,
            mime_type: value.data.mime_type.ok_or(MissingField("mime_type"))?,
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            parse_mode: value.data.parse_mode,
            description: value.data.description,
            reply_markup: value.data.reply_markup,
            input_message_content: value.data.input_message_content,
            thumb_url: value.data.thumb_url,
            thumb_width: value.data.thumb_width,
            thumb_height: value.data.thumb_height,
        })
    }
}

impl From<InlineQueryResultDocument> for RawInlineQueryResult {
    fn from(value: InlineQueryResultDocument) -> Self {
        Self {
            data: RawInlineQueryResultData {
                title: Some(value.title),
                document_url: Some(value.document_url),
                mime_type: Some(value.mime_type),
                caption: value.caption,
                caption_entities: value.caption_entities,
                parse_mode: value.parse_mode,
                description: value.description,
                reply_markup: value.reply_markup,
                input_message_content: value.input_message_content,
                thumb_url: value.thumb_url,
                thumb_width: value.thumb_width,
                thumb_height: value.thumb_height,
                ..Default::default()
            },
            id: value.id,
            kind: RawInlineQueryResultKind::Document,
        }
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultCachedDocument {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            title: value.data.title.ok_or(MissingField("title"))?,
            document_file_id: value.data.document_file_id.ok_or(MissingField("document_file_id"))?,
            description: value.data.description,
            caption: value.data.caption,
            caption_entities: value.data.caption_entities,
            parse_mode: value.data.parse_mode,
            reply_markup: value.data.reply_markup,
            input_message_content: value.data.input_message_content,
        })
    }
}

impl From<InlineQueryResultCachedDocument> for RawInlineQueryResult {
    fn from(value: InlineQueryResultCachedDocument) -> Self {
        Self {
            data: RawInlineQueryResultData {
                title: Some(value.title),
                document_file_id: Some(value.document_file_id),
                description: value.description,
                caption: value.caption,
                caption_entities: value.caption_entities,
                parse_mode: value.parse_mode,
                reply_markup: value.reply_markup,
                input_message_content: value.input_message_content,
                ..Default::default()
            },
            id: value.id,
            kind: RawInlineQueryResultKind::CachedDocument,
        }
    }
}
