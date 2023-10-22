use serde::{Deserialize, Serialize};

use crate::types::{Float, InlineKeyboardMarkup, InputMessageContent, Integer};

use super::raw::{
    RawInlineQueryResult,
    RawInlineQueryResultData,
    RawInlineQueryResultDataError::{self, MissingField},
    RawInlineQueryResultKind,
};

#[cfg(test)]
mod tests;

/// Venue
///
/// By default, the venue will be sent by the user
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the venue
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultVenue {
    id: String,
    latitude: Float,
    longitude: Float,
    title: String,
    address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_place_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_place_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_height: Option<Integer>,
}

impl InlineQueryResultVenue {
    /// Creates a new InlineQueryResultVenue with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * latitude - Latitude of the venue location in degrees
    /// * longitude - Longitude of the venue location in degrees
    /// * title - Title of the venue
    /// * address - Address of the venue
    pub fn new<I, T, A>(id: I, latitude: Float, longitude: Float, title: T, address: A) -> Self
    where
        I: Into<String>,
        T: Into<String>,
        A: Into<String>,
    {
        InlineQueryResultVenue {
            id: id.into(),
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            reply_markup: None,
            input_message_content: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
        }
    }

    /// Foursquare identifier of the venue if known
    pub fn foursquare_id<S: Into<String>>(mut self, foursquare_id: S) -> Self {
        self.foursquare_id = Some(foursquare_id.into());
        self
    }

    /// Foursquare type of the venue, if known
    ///
    /// For example, “arts_entertainment/default”, “arts_entertainment/aquarium” or “food/ice-cream”
    pub fn foursquare_type<S: Into<String>>(mut self, foursquare_type: S) -> Self {
        self.foursquare_type = Some(foursquare_type.into());
        self
    }

    /// Google Places identifier of the venue
    pub fn google_place_id<S: Into<String>>(mut self, google_place_id: S) -> Self {
        self.google_place_id = Some(google_place_id.into());
        self
    }

    /// Google Places type of the venue
    ///
    /// <https://developers.google.com/places/web-service/supported_types>
    pub fn google_place_type<S: Into<String>>(mut self, google_place_type: S) -> Self {
        self.google_place_type = Some(google_place_type.into());
        self
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// Content of the message to be sent instead of the venue
    pub fn input_message_content<C: Into<InputMessageContent>>(mut self, input_message_content: C) -> Self {
        self.input_message_content = Some(input_message_content.into());
        self
    }

    /// URL of the thumbnail for the result
    pub fn thumbnail_url<S: Into<String>>(mut self, thumbnail_url: S) -> Self {
        self.thumbnail_url = Some(thumbnail_url.into());
        self
    }

    /// Thumbnail width
    pub fn thumbnail_width(mut self, thumbnail_width: Integer) -> Self {
        self.thumbnail_width = Some(thumbnail_width);
        self
    }

    /// Thumbnail height
    pub fn thumbnail_height(mut self, thumbnail_height: Integer) -> Self {
        self.thumbnail_height = Some(thumbnail_height);
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultVenue {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            latitude: value.data.latitude.ok_or(MissingField("latitude"))?,
            longitude: value.data.longitude.ok_or(MissingField("longitude"))?,
            title: value.data.title.ok_or(MissingField("title"))?,
            address: value.data.address.ok_or(MissingField("address"))?,
            foursquare_id: value.data.foursquare_id,
            foursquare_type: value.data.foursquare_type,
            google_place_id: value.data.google_place_id,
            google_place_type: value.data.google_place_type,
            reply_markup: value.data.reply_markup,
            input_message_content: value.data.input_message_content,
            thumbnail_url: value.data.thumbnail_url,
            thumbnail_width: value.data.thumbnail_width,
            thumbnail_height: value.data.thumbnail_height,
        })
    }
}

impl From<InlineQueryResultVenue> for RawInlineQueryResult {
    fn from(value: InlineQueryResultVenue) -> Self {
        Self {
            data: RawInlineQueryResultData {
                latitude: Some(value.latitude),
                longitude: Some(value.longitude),
                title: Some(value.title),
                address: Some(value.address),
                foursquare_id: value.foursquare_id,
                foursquare_type: value.foursquare_type,
                google_place_id: value.google_place_id,
                google_place_type: value.google_place_type,
                reply_markup: value.reply_markup,
                input_message_content: value.input_message_content,
                thumbnail_url: value.thumbnail_url,
                thumbnail_width: value.thumbnail_width,
                thumbnail_height: value.thumbnail_height,
                ..Default::default()
            },
            id: value.id,
            kind: RawInlineQueryResultKind::Venue,
        }
    }
}
