use serde::{Deserialize, Serialize};

use super::raw::{
    RawInlineQueryResult,
    RawInlineQueryResultData,
    RawInlineQueryResultDataError::{self, MissingField},
    RawInlineQueryResultType,
};
use crate::types::{Float, InlineKeyboardMarkup, InputMessageContent, Integer};

#[cfg(test)]
mod tests;

/// Represents a venue.
///
/// By default, the venue will be sent by the user.
/// Alternatively, you can use [`Self::with_input_message_content`]
/// to send a message with the specified content instead of the venue.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultVenue {
    address: String,
    id: String,
    latitude: Float,
    longitude: Float,
    title: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
    input_message_content: Option<InputMessageContent>,
    reply_markup: Option<InlineKeyboardMarkup>,
    thumbnail_height: Option<Integer>,
    thumbnail_url: Option<String>,
    thumbnail_width: Option<Integer>,
}

impl InlineQueryResultVenue {
    /// Creates a new `InlineQueryResultVenue`.
    ///
    /// # Arguments
    ///
    /// * `address` - Address of the venue.
    /// * `id` - Unique identifier of the result; 1-64 bytes.
    /// * `latitude` - Latitude of the venue location in degrees.
    /// * `longitude` - Longitude of the venue location in degrees.
    /// * `title` - Title of the venue.
    pub fn new<A, B, C>(address: A, id: B, latitude: Float, longitude: Float, title: C) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            address: address.into(),
            id: id.into(),
            latitude,
            longitude,
            title: title.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            input_message_content: None,
            reply_markup: None,
            thumbnail_height: None,
            thumbnail_url: None,
            thumbnail_width: None,
        }
    }

    /// Sets a new foursquare ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Foursquare identifier of the venue.
    pub fn with_foursquare_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.foursquare_id = Some(value.into());
        self
    }

    /// Sets a new foursquare type.
    ///
    /// # Arguments
    ///
    /// * `value` - Foursquare type of the venue.
    ///
    /// For example, “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/ice-cream”.
    pub fn with_foursquare_type<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.foursquare_type = Some(value.into());
        self
    }

    /// Sets a new Google Places ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Google Places identifier of the venue.
    pub fn with_google_place_id<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.google_place_id = Some(value.into());
        self
    }

    /// Sets a new Google Places type.
    ///
    /// # Arguments
    ///
    /// * `value` - Google Places type of the venue.
    ///
    /// <https://developers.google.com/places/web-service/supported_types>
    pub fn with_google_place_type<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.google_place_type = Some(value.into());
        self
    }

    /// Sets a new input message content.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the message to be sent instead of the venue.
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
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultVenue {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            address: value.data.address.ok_or(MissingField("address"))?,
            foursquare_id: value.data.foursquare_id,
            foursquare_type: value.data.foursquare_type,
            google_place_id: value.data.google_place_id,
            google_place_type: value.data.google_place_type,
            id: value.id,
            input_message_content: value.data.input_message_content,
            latitude: value.data.latitude.ok_or(MissingField("latitude"))?,
            longitude: value.data.longitude.ok_or(MissingField("longitude"))?,
            reply_markup: value.data.reply_markup,
            thumbnail_height: value.data.thumbnail_height,
            thumbnail_url: value.data.thumbnail_url,
            thumbnail_width: value.data.thumbnail_width,
            title: value.data.title.ok_or(MissingField("title"))?,
        })
    }
}

impl From<InlineQueryResultVenue> for RawInlineQueryResult {
    fn from(value: InlineQueryResultVenue) -> Self {
        Self {
            data: RawInlineQueryResultData {
                address: Some(value.address),
                foursquare_id: value.foursquare_id,
                foursquare_type: value.foursquare_type,
                google_place_id: value.google_place_id,
                google_place_type: value.google_place_type,
                input_message_content: value.input_message_content,
                latitude: Some(value.latitude),
                longitude: Some(value.longitude),
                reply_markup: value.reply_markup,
                thumbnail_height: value.thumbnail_height,
                thumbnail_url: value.thumbnail_url,
                thumbnail_width: value.thumbnail_width,
                title: Some(value.title),
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::Venue,
        }
    }
}
