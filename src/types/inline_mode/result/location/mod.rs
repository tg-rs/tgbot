use serde::{Deserialize, Serialize};

use crate::types::{Float, InlineKeyboardMarkup, InputMessageContent, Integer};

use super::raw::{
    RawInlineQueryResult,
    RawInlineQueryResultData,
    RawInlineQueryResultDataError::{self, MissingField},
    RawInlineQueryResultType,
};

#[cfg(test)]
mod tests;

/// Represents a location on a map
///
/// By default, the location will be sent by the user.
/// Alternatively, you can use [`Self::with_input_message_content`]
/// to send a message with the specified content instead of the location.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct InlineQueryResultLocation {
    id: String,
    latitude: Float,
    longitude: Float,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    heading: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_accuracy: Option<Float>,
    #[serde(skip_serializing_if = "Option::is_none")]
    input_message_content: Option<InputMessageContent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_period: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proximity_alert_radius: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumbnail_width: Option<Integer>,
}

impl InlineQueryResultLocation {
    /// Creates a new InlineQueryResultLocation
    ///
    /// # Arguments
    ///
    /// * id - Unique identifier for this result, 1-64 bytes
    /// * latitude - Location latitude in degrees
    /// * longitude - Location longitude in degrees
    /// * title - Location title
    pub fn new<A, B>(id: A, latitude: Float, longitude: Float, title: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
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

    /// Sets a new heading
    ///
    /// # Arguments
    ///
    /// * value - Direction in which the user is moving, in degrees; 1-360
    pub fn with_heading(mut self, value: Integer) -> Self {
        self.heading = Some(value);
        self
    }

    /// Sets a new horizontal accuracy
    ///
    /// # Arguments
    ///
    /// * value - The radius of uncertainty for the location, measured in meters; 0-1500
    pub fn with_horizontal_accuracy(mut self, value: Float) -> Self {
        self.horizontal_accuracy = Some(value);
        self
    }

    /// Sets a new input message content
    ///
    /// # Arguments
    ///
    /// * value - Content of the message to be sent instead of the location
    pub fn with_input_message_content<T>(mut self, value: T) -> Self
    where
        T: Into<InputMessageContent>,
    {
        self.input_message_content = Some(value.into());
        self
    }

    /// Sets a new live period
    ///
    /// # Arguments
    ///
    /// * value - Period in seconds for which the location can be updated; 60-86400
    pub fn with_live_period(mut self, value: Integer) -> Self {
        self.live_period = Some(value);
        self
    }

    /// Sets a new proximity alert radius
    ///
    /// # Arguments
    ///
    /// * value - A maximum distance for proximity alerts about
    ///           approaching another chat member, in meters; 1-100000
    pub fn with_proximity_alert_radius(mut self, value: Integer) -> Self {
        self.proximity_alert_radius = Some(value);
        self
    }

    /// Sets a new reply markup
    ///
    /// # Arguments
    ///
    /// * value - Inline keyboard attached to the message
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<InlineKeyboardMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }

    /// Sets a new thumbnail height
    ///
    /// # Arguments
    ///
    /// * value - Thumbnail height
    pub fn with_thumbnail_height(mut self, value: Integer) -> Self {
        self.thumbnail_height = Some(value);
        self
    }

    /// Sets a new thumbnail URL
    ///
    /// # Arguments
    ///
    /// * value - URL of the thumbnail for the result
    pub fn with_thumbnail_url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.thumbnail_url = Some(value.into());
        self
    }

    /// Sets a new thumbnail width
    ///
    /// # Arguments
    ///
    /// * value - Thumbnail width
    pub fn with_thumbnail_width(mut self, value: Integer) -> Self {
        self.thumbnail_width = Some(value);
        self
    }
}

impl TryFrom<RawInlineQueryResult> for InlineQueryResultLocation {
    type Error = RawInlineQueryResultDataError;

    fn try_from(value: RawInlineQueryResult) -> Result<Self, Self::Error> {
        Ok(Self {
            heading: value.data.heading,
            horizontal_accuracy: value.data.horizontal_accuracy,
            id: value.id,
            input_message_content: value.data.input_message_content,
            latitude: value.data.latitude.ok_or(MissingField("latitude"))?,
            live_period: value.data.live_period,
            longitude: value.data.longitude.ok_or(MissingField("longitude"))?,
            proximity_alert_radius: value.data.proximity_alert_radius,
            reply_markup: value.data.reply_markup,
            thumbnail_height: value.data.thumbnail_height,
            thumbnail_url: value.data.thumbnail_url,
            thumbnail_width: value.data.thumbnail_width,
            title: value.data.title.ok_or(MissingField("title"))?,
        })
    }
}

impl From<InlineQueryResultLocation> for RawInlineQueryResult {
    fn from(value: InlineQueryResultLocation) -> Self {
        Self {
            data: RawInlineQueryResultData {
                horizontal_accuracy: value.horizontal_accuracy,
                latitude: Some(value.latitude),
                live_period: value.live_period,
                longitude: Some(value.longitude),
                heading: value.heading,
                input_message_content: value.input_message_content,
                proximity_alert_radius: value.proximity_alert_radius,
                reply_markup: value.reply_markup,
                thumbnail_height: value.thumbnail_height,
                thumbnail_url: value.thumbnail_url,
                thumbnail_width: value.thumbnail_width,
                title: Some(value.title),
                ..Default::default()
            },
            id: value.id,
            result_type: RawInlineQueryResultType::Location,
        }
    }
}
