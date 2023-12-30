use serde::{Deserialize, Serialize};

use super::raw::{
    RawInlineQueryResult,
    RawInlineQueryResultData,
    RawInlineQueryResultDataError::{self, MissingField},
    RawInlineQueryResultType,
};
use crate::types::{InlineKeyboardMarkup, InputMessageContent, Integer};

#[cfg(test)]
mod tests;

/// Represents a contact with a phone number.
///
/// By default, this contact will be sent by the user.
/// Alternatively, you can use [Self::with_input_message_content]
/// to send a message with the specified content instead of the contact.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultContact {
    first_name: String,
    id: String,
    phone_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<String>,
}

impl InlineQueryResultContact {
    /// Creates a new `InlineQueryResultContact`.
    ///
    /// # Arguments
    ///
    /// * `first_name` - Contact's first name.
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `phone_number` - Contact's phone number.
    pub fn new<A, B, C>(first_name: A, id: B, phone_number: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            first_name: first_name.into(),
            id: id.into(),
            phone_number: phone_number.into(),
            input_message_content: None,
            last_name: None,
            reply_markup: None,
            thumbnail_height: None,
            thumbnail_url: None,
            thumbnail_width: None,
            vcard: None,
        }
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the contact.
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        self.input_message_content = Some(value.into());
        self
    }

    /// Sets a new last name.
    ///
    /// # Arguments
    ///
    /// * `value` - Last name.
    pub fn with_last_name<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.last_name = Some(value.into());
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

    /// Sets a new thumbnail height.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail height.
    pub fn with_thumbnail_height(mut self, value: Integer) -> Self {
        self.thumbnail_height = Some(value);
        self
    }

    /// Sets a new thumbnail URL.
    ///
    /// # Arguments
    ///
    /// * `value` - URL of the thumbnail for the result.
    pub fn with_thumbnail_url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.thumbnail_url = Some(value.into());
        self
    }

    /// Sets a new thumbnail width.
    ///
    /// # Arguments
    ///
    /// * `value` - Thumbnail width.
    pub fn with_thumbnail_width(mut self, value: Integer) -> Self {
        self.thumbnail_width = Some(value);
        self
    }

    /// Sets a new vCard.
    ///
    /// # Arguments
    ///
    /// * `value` - Additional data about the contact in the form of a vCard; 0-2048 bytes.
    pub fn with_vcard<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.vcard = Some(value.into());
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultContact {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            first_name: value.data.first_name.ok_or(MissingField("first_name"))?,
            id: value.id,
            input_message_content: value.data.input_message_content,
            last_name: value.data.last_name,
            phone_number: value.data.phone_number.ok_or(MissingField("phone_number"))?,
            reply_markup: value.data.reply_markup,
            thumbnail_height: value.data.thumbnail_height,
            thumbnail_url: value.data.thumbnail_url,
            thumbnail_width: value.data.thumbnail_width,
            vcard: value.data.vcard,
        })
    }
}

impl From<InlineQueryResultContact> for RawInlineQueryResult {
    fn from(value: InlineQueryResultContact) -> Self {
        Self {
            data: RawInlineQueryResultData {
                first_name: Some(value.first_name),
                input_message_content: value.input_message_content,
                last_name: value.last_name,
                phone_number: Some(value.phone_number),
                reply_markup: value.reply_markup,
                thumbnail_height: value.thumbnail_height,
                thumbnail_url: value.thumbnail_url,
                thumbnail_width: value.thumbnail_width,
                vcard: value.vcard,
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::Contact,
        }
    }
}
