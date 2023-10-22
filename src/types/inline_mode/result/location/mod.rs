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

/// Location on a map
///
/// By default, the location will be sent by the user
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the location
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultLocation {
    id: String,
    latitude: Float,
    longitude: Float,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_accuracy: Option<Float>,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_period: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heading: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proximity_alert_radius: Option<Integer>,
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

impl InlineQueryResultLocation {
    /// Creates a new InlineQueryResultLocation with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * latitude - Location latitude in degrees
    /// * longitude - Location longitude in degrees
    /// * title - Location title
    pub fn new<I, T>(id: I, latitude: Float, longitude: Float, title: T) -> Self
    where
        I: Into<String>,
        T: Into<String>,
    {
        InlineQueryResultLocation {
            id: id.into(),
            latitude,
            longitude,
            title: title.into(),
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
            input_message_content: None,
            thumbnail_url: None,
            thumbnail_width: None,
            thumbnail_height: None,
        }
    }

    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub fn horizontal_accuracy(mut self, horizontal_accuracy: Float) -> Self {
        self.horizontal_accuracy = Some(horizontal_accuracy);
        self
    }

    /// Period in seconds for which the location can be updated, should be between 60 and 86400
    pub fn live_period(mut self, live_period: Integer) -> Self {
        self.live_period = Some(live_period);
        self
    }

    /// For live locations, a direction in which the user is moving, in degrees
    ///
    /// Must be between 1 and 360 if specified
    pub fn heading(mut self, heading: Integer) -> Self {
        self.heading = Some(heading);
        self
    }

    /// For live locations, a maximum distance for proximity alerts about
    /// approaching another chat member, in meters
    ///
    /// Must be between 1 and 100000 if specified
    pub fn proximity_alert_radius(mut self, proximity_alert_radius: Integer) -> Self {
        self.proximity_alert_radius = Some(proximity_alert_radius);
        self
    }

    /// Inline keyboard attached to the message
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }

    /// Content of the message to be sent instead of the location
    pub fn input_message_content<C: Into<InputMessageContent>>(mut self, input_message_content: C) -> Self {
        self.input_message_content = Some(input_message_content.into());
        self
    }

    /// Url of the thumbnail for the result
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

impl TryFrom<RawInlineQueryResult> for InlineQueryResultLocation {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.id,
            latitude: value.data.latitude.ok_or(MissingField("latitude"))?,
            longitude: value.data.longitude.ok_or(MissingField("longitude"))?,
            title: value.data.title.ok_or(MissingField("title"))?,
            horizontal_accuracy: value.data.horizontal_accuracy,
            live_period: value.data.live_period,
            heading: value.data.heading,
            proximity_alert_radius: value.data.proximity_alert_radius,
            reply_markup: value.data.reply_markup,
            input_message_content: value.data.input_message_content,
            thumbnail_url: value.data.thumbnail_url,
            thumbnail_width: value.data.thumbnail_width,
            thumbnail_height: value.data.thumbnail_height,
        })
    }
}

impl From<InlineQueryResultLocation> for RawInlineQueryResult {
    fn from(value: InlineQueryResultLocation) -> Self {
        Self {
            data: RawInlineQueryResultData {
                latitude: Some(value.latitude),
                longitude: Some(value.longitude),
                title: Some(value.title),
                horizontal_accuracy: value.horizontal_accuracy,
                live_period: value.live_period,
                heading: value.heading,
                proximity_alert_radius: value.proximity_alert_radius,
                reply_markup: value.reply_markup,
                input_message_content: value.input_message_content,
                thumbnail_url: value.thumbnail_url,
                thumbnail_width: value.thumbnail_width,
                thumbnail_height: value.thumbnail_height,
                ..Default::default()
            },
            id: value.id,
            kind: RawInlineQueryResultKind::Location,
        }
    }
}
