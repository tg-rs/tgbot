use serde::{Deserialize, Serialize};

use crate::{
    api::{Method, Payload},
    types::{ChatId, Float, Integer, Location, Message, ReplyMarkup},
};

#[cfg(test)]
mod tests;

/// Represents a venue.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct Venue {
    /// Address of the venue.
    pub address: String,
    /// Location of the venue.
    pub location: Location,
    /// Name of the venue.
    pub title: String,
    /// Foursquare identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_id: Option<String>,
    /// Foursquare type.
    ///
    /// Example: “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/ice-cream”.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub foursquare_type: Option<String>,
    /// Google Places identifier.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    /// Google Places type.
    ///
    /// <https://developers.google.com/places/web-service/supported_types>
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_type: Option<String>,
}

impl Venue {
    /// Creates a new `Venue`.
    ///
    /// # Arguments
    ///
    /// * `title` - Title of the venue.
    /// * `address` - Address of the venue.
    /// * `location` - Location of the venue.
    pub fn new<A, B>(title: A, address: B, location: Location) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            address: address.into(),
            location,
            title: title.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
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
}

/// Sends information about a venue.
#[derive(Clone, Debug, Serialize)]
pub struct SendVenue {
    chat_id: ChatId,
    latitude: Float,
    longitude: Float,
    title: String,
    address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    allow_sending_without_reply: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_notification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_place_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    google_place_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_thread_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protect_content: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
}

impl SendVenue {
    /// Creates a new `SendVenue`.
    ///
    /// # Arguments
    ///
    /// * `chat_id` - Unique identifier of the target chat.
    /// * `latitude` - Latitude of the venue.
    /// * `longitude` - Longitude of the venue.
    /// * `title` - Name of the venue.
    /// * `address` - Address of the venue.
    pub fn new<A, B, C>(chat_id: A, latitude: Float, longitude: Float, title: B, address: C) -> Self
    where
        A: Into<ChatId>,
        B: Into<String>,
        C: Into<String>,
    {
        Self {
            chat_id: chat_id.into(),
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            allow_sending_without_reply: None,
            disable_notification: None,
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
            message_thread_id: None,
            protect_content: None,
            reply_markup: None,
            reply_to_message_id: None,
        }
    }

    /// Sets a new value for an `allow_sending_without_reply` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the message should be sent even
    ///             if the specified replied-to message is not found.
    pub fn with_allow_sending_without_reply(mut self, value: bool) -> Self {
        self.allow_sending_without_reply = Some(value);
        self
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

    /// Sets a new message ID for a reply.
    ///
    /// # Arguments
    ///
    /// * `value` - ID of the original message.
    pub fn with_reply_to_message_id(mut self, value: Integer) -> Self {
        self.reply_to_message_id = Some(value);
        self
    }
}

impl Method for SendVenue {
    type Response = Message;

    fn into_payload(self) -> Payload {
        Payload::json("sendVenue", self)
    }
}
