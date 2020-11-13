use crate::{
    methods::Method,
    request::Request,
    types::{ChatId, EditMessageResult, Float, InlineKeyboardMarkup, Integer},
};
use serde::Serialize;

/// Edit live location messages sent by the bot or via the bot (for inline bots)
///
/// A location can be edited until its live_period expires or editing
/// is explicitly disabled by a call to stopMessageLiveLocation
#[derive(Clone, Debug, Serialize)]
pub struct EditMessageLiveLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    chat_id: Option<ChatId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_message_id: Option<String>,
    latitude: Float,
    longitude: Float,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_accuracy: Option<Float>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heading: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proximity_alert_radius: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl EditMessageLiveLocation {
    /// Creates a new EditMessageLiveLocation
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    /// * latitude - Latitude of new location
    /// * longitude Longitude of new location
    pub fn new<C: Into<ChatId>>(chat_id: C, message_id: Integer, latitude: Float, longitude: Float) -> Self {
        EditMessageLiveLocation {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            inline_message_id: None,
            latitude,
            longitude,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
        }
    }

    /// Creates a new EditMessageLiveLocation
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    /// * latitude - Latitude of new location
    /// * longitude Longitude of new location
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S, latitude: Float, longitude: Float) -> Self {
        EditMessageLiveLocation {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
            latitude,
            longitude,
            horizontal_accuracy: None,
            heading: None,
            proximity_alert_radius: None,
            reply_markup: None,
        }
    }

    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub fn horizontal_accuracy(mut self, horizontal_accuracy: Float) -> Self {
        self.horizontal_accuracy = Some(horizontal_accuracy);
        self
    }

    /// Direction in which the user is moving, in degrees
    ///
    /// Must be between 1 and 360 if specified
    pub fn heading(mut self, heading: Integer) -> Self {
        self.heading = Some(heading);
        self
    }

    /// Maximum distance for proximity alerts about approaching another chat member, in meters
    ///
    /// Must be between 1 and 100000 if specified
    pub fn proximity_alert_radius(mut self, proximity_alert_radius: Integer) -> Self {
        self.proximity_alert_radius = Some(proximity_alert_radius);
        self
    }

    /// New inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for EditMessageLiveLocation {
    type Response = EditMessageResult;

    fn into_request(self) -> Request {
        Request::json("editMessageLiveLocation", self)
    }
}

/// Stop updating a live location message
/// sent by the bot or via the bot (for inline bots)
/// before live_period expires
#[derive(Clone, Debug, Serialize)]
pub struct StopMessageLiveLocation {
    chat_id: Option<ChatId>,
    message_id: Option<Integer>,
    inline_message_id: Option<String>,
    reply_markup: Option<InlineKeyboardMarkup>,
}

impl StopMessageLiveLocation {
    /// Creates a new StopMessageLiveLocation
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * message_id - Identifier of the sent message
    pub fn new<C: Into<ChatId>>(chat_id: C, message_id: Integer) -> Self {
        StopMessageLiveLocation {
            chat_id: Some(chat_id.into()),
            message_id: Some(message_id),
            inline_message_id: None,
            reply_markup: None,
        }
    }

    /// Creates a new StopMessageLiveLocation
    ///
    /// # Arguments
    ///
    /// * inline_message_id - Identifier of the inline message
    pub fn with_inline_message_id<S: Into<String>>(inline_message_id: S) -> Self {
        StopMessageLiveLocation {
            chat_id: None,
            message_id: None,
            inline_message_id: Some(inline_message_id.into()),
            reply_markup: None,
        }
    }

    /// New inline keyboard
    pub fn reply_markup<I: Into<InlineKeyboardMarkup>>(mut self, reply_markup: I) -> Self {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

impl Method for StopMessageLiveLocation {
    type Response = EditMessageResult;

    fn into_request(self) -> Request {
        Request::json("stopMessageLiveLocation", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        request::{RequestBody, RequestMethod},
        types::InlineKeyboardButton,
    };
    use serde_json::Value;

    #[allow(clippy::float_cmp)]
    #[test]
    fn edit_message_live_location() {
        let request = EditMessageLiveLocation::new(1, 2, 3.0, 4.0)
            .horizontal_accuracy(2.6)
            .heading(100)
            .proximity_alert_radius(200)
            .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/editMessageLiveLocation"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["message_id"], 2);
            assert_eq!(data["latitude"], 3.0);
            assert_eq!(data["longitude"], 4.0);
            assert_eq!(data["horizontal_accuracy"], 2.6);
            assert_eq!(data["heading"], 100);
            assert_eq!(data["proximity_alert_radius"], 200);
            assert_eq!(data["reply_markup"]["inline_keyboard"][0][0]["text"], "text");
        } else {
            panic!("Unexpected request body");
        }

        let request = EditMessageLiveLocation::with_inline_message_id("msg-id", 3.0, 4.0).into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/editMessageLiveLocation"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(
                data,
                serde_json::json!({
                    "inline_message_id": "msg-id",
                    "latitude": 3.0,
                    "longitude": 4.0
                })
            );
        } else {
            panic!("Unexpected request body");
        }
    }

    #[test]
    fn stop_message_live_location() {
        let request = StopMessageLiveLocation::new(1, 2)
            .reply_markup(vec![vec![InlineKeyboardButton::with_url("text", "url")]])
            .into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/stopMessageLiveLocation"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["message_id"], 2);
            assert_eq!(data["reply_markup"]["inline_keyboard"][0][0]["text"], "text");
        } else {
            panic!("Unexpected request body");
        }

        let request = StopMessageLiveLocation::with_inline_message_id("msg-id").into_request();
        assert_eq!(request.get_method(), RequestMethod::Post);
        assert_eq!(
            request.build_url("base-url", "token"),
            "base-url/bottoken/stopMessageLiveLocation"
        );
        if let RequestBody::Json(data) = request.into_body() {
            let data: Value = serde_json::from_str(&data.unwrap()).unwrap();
            assert_eq!(data["inline_message_id"], "msg-id");
        } else {
            panic!("Unexpected request body");
        }
    }
}
