use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{ChatId, Float, Integer, Message, ReplyMarkup, ReplyParameters},
};

#[cfg(test)]
mod tests;

/// Represents a point on a map.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Location {
    /// Latitude as defined by sender.
    pub latitude: Float,
    /// Longitude as defined by sender.
    pub longitude: Float,
    /// The direction in which user is moving; in degrees; 1-360.
    ///
    /// For active live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub heading: Option<Integer>,
    /// The radius of uncertainty for the location, measured in meters; 0-1500.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_accuracy: Option<Float>,
    /// Time relative to the message sending date,
    /// during which the location can be updated, in seconds.
    ///
    /// For active live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_period: Option<Integer>,
    /// Maximum distance for proximity alerts about
    /// approaching another chat member; in meters.
    ///
    /// For sent live locations only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_alert_radius: Option<Integer>,
}

impl Location {
    /// Creates a new `Location`.
    ///
    /// # Arguments
    ///
    /// * `latitude` - Latitude.
    /// * `longitude` - Longitude.
    pub fn new(latitude: Float, longitude: Float) -> Self {
        Self {
            latitude,
            longitude,
            heading: None,
            horizontal_accuracy: None,
            live_period: None,
            proximity_alert_radius: None,
        }
    }

    /// Sets a new heading.
    ///
    /// # Arguments
    ///
    /// * `value` - A direction in which the user is moving; in degrees; 1-360.
    pub fn with_heading(mut self, value: Integer) -> Self {
        self.heading = Some(value);
        self
    }

    /// Sets a new horizontal accuracy.
    ///
    /// # Arguments
    ///
    /// * `value` - A radius of uncertainty for the location; in meters; 0-1500.
    pub fn with_horizontal_accuracy(mut self, value: Float) -> Self {
        self.horizontal_accuracy = Some(value);
        self
    }

    /// Sets a new live period.
    ///
    /// # Arguments
    ///
    /// * `value` - Period in seconds for which the location can be updated; 60-86400.
    pub fn with_live_period(mut self, value: Integer) -> Self {
        self.live_period = Some(value);
        self
    }

    /// Sets a new proximity alert radius.
    ///
    /// # Arguments
    ///
    /// * `value` - A maximum distance for proximity alerts
    ///             about approaching another chat member; in meters; 1-100000.
    pub fn with_proximity_alert_radius(mut self, value: Integer) -> Self {
        self.proximity_alert_radius = Some(value);
        self
    }
}

/// Sends a point on a map.
#[derive(Clone, Debug, Serialize)]
pub struct SendLocation {
    chat_id: ChatId,
    latitude: Float,
    longitude: Float,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heading: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_accuracy: Option<Float>,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_period: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proximity_alert_radius: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_parameters: Option<ReplyParameters>,
}

impl SendLocation {
    /// Creates a new `SendLocation`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `latitude` - Latitude.
    /// * `longitude` - Longitude.
    pub fn new<T>(chat_id: T, latitude: Float, longitude: Float) -> Self
    where
        T: Into<ChatId>,
    {
        Self {
            chat_id: chat_id.into(),
            latitude,
            longitude,
            disable_notification: None,
            heading: None,
            horizontal_accuracy: None,
            live_period: None,
            message_thread_id: None,
            protect_content: None,
            proximity_alert_radius: None,
            reply_markup: None,
            reply_parameters: None,
        }
    }

    /// Sets a new value for a `disable_notification` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to send the message silently or not;
    ///             a user will receive a notification without sound.
    pub fn with_disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    /// Sets a new heading.
    ///
    /// # Arguments
    ///
    /// * `value` - A direction in which the user is moving; in degrees; 1-360.
    pub fn with_heading(mut self, value: Integer) -> Self {
        self.heading = Some(value);
        self
    }

    /// Sets a new horizontal accuracy.
    ///
    /// # Arguments
    ///
    /// * `value` - A radius of uncertainty for the location; in meters; 0-1500.
    pub fn with_horizontal_accuracy(mut self, value: Float) -> Self {
        self.horizontal_accuracy = Some(value);
        self
    }

    /// Sets a new live period.
    ///
    /// # Arguments
    ///
    /// * `value` - Period in seconds for which the location can be updated; 60-86400.
    pub fn with_live_period(mut self, value: Integer) -> Self {
        self.live_period = Some(value);
        self
    }

    /// Sets a new message thread ID.
    ///
    /// # Arguments
    ///
    /// * `value` - Unique identifier of the target message thread;
    ///             supergroups only.
    pub fn with_message_thread_id(mut self, value: Integer) -> Self {
        self.message_thread_id = Some(value);
        self
    }

    /// Sets a new proximity alert radius.
    ///
    /// # Arguments
    ///
    /// * `value` - A maximum distance for proximity alerts
    ///           about approaching another chat member; in meters; 1-100000.
    pub fn with_proximity_alert_radius(mut self, value: Integer) -> Self {
        self.proximity_alert_radius = Some(value);
        self
    }

    /// Sets a new value for a `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether to protect the contents
    ///             of the sent message from forwarding and saving.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.protect_content = Some(value);
        self
    }

    /// Sets a new reply markup.
    ///
    /// # Arguments
    ///
    /// * `value` - Reply markup.
    pub fn with_reply_markup<T>(mut self, value: T) -> Self
    where
        T: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }

    /// Sets new reply parameters.
    ///
    /// # Arguments
    ///
    /// * `value` - Description of the message to reply to.
    pub fn with_reply_parameters(mut self, value: ReplyParameters) -> Self {
        self.reply_parameters = Some(value);
        self
    }
}

impl Method for SendLocation {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("sendLocation", self)
    }
}
