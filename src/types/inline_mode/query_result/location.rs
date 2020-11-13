use crate::types::{
    inline_mode::message_content::InputMessageContent,
    primitive::{Float, Integer},
    reply_markup::InlineKeyboardMarkup,
};
use serde::Serialize;

/// Location on a map
///
/// By default, the location will be sent by the user
/// Alternatively, you can use input_message_content
/// to send a message with the specified content instead of the location
#[derive(Clone, Debug, Serialize)]
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
    thumb_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    thumb_height: Option<Integer>,
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
            thumb_url: None,
            thumb_width: None,
            thumb_height: None,
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
