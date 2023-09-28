use crate::types::{
    parse_mode::ParseMode,
    payments::LabeledPrice,
    primitive::{Float, Integer},
    text::{TextEntities, TextEntity},
};
use serde::Serialize;
use serde_json::Error as JsonError;

/// Content of a message to be sent as a result of an inline query
#[derive(Clone, Debug, derive_more::From, Serialize)]
#[serde(untagged)]
pub enum InputMessageContent {
    /// Contact message
    Contact(InputMessageContentContact),
    /// Invoice message
    Invoice(InputMessageContentInvoice),
    /// Location message
    Location(InputMessageContentLocation),
    /// Text message
    Text(InputMessageContentText),
    /// Venue message
    Venue(InputMessageContentVenue),
}

/// Contact message to be sent as the result of an inline query
#[derive(Clone, Debug, Serialize)]
pub struct InputMessageContentContact {
    phone_number: String,
    first_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vcard: Option<String>,
}

impl InputMessageContentContact {
    /// Creates a new InputMessageContentContact with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * phone_numer - Contact's phone number
    /// * first_name - Contact's first name
    pub fn new<S: Into<String>>(phone_number: S, first_name: S) -> Self {
        InputMessageContentContact {
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            vcard: None,
        }
    }

    /// Contact's last name
    pub fn last_name<S: Into<String>>(mut self, last_name: S) -> Self {
        self.last_name = Some(last_name.into());
        self
    }

    /// Additional data about the contact in the form of a vCard, 0-2048 bytes
    pub fn vcard<S: Into<String>>(mut self, vcard: S) -> Self {
        self.vcard = Some(vcard.into());
        self
    }
}

/// Represents the content of an invoice message to be sent as the result of an inline query
#[derive(Clone, Debug, Serialize)]
pub struct InputMessageContentInvoice {
    title: String,
    description: String,
    payload: String,
    provider_token: String,
    currency: String,
    prices: Vec<LabeledPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tip_amount: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_tip_amounts: Option<Vec<Integer>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    provider_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_size: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    photo_height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_name: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_phone_number: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    need_shipping_address: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_phone_number_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_email_to_provider: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_flexible: Option<bool>,
}

impl InputMessageContentInvoice {
    /// Creates a new InputMessageContentInvoice
    ///
    /// # Arguments
    ///
    /// * title - Product name, 1-32 characters
    /// * description - Product description, 1-255 characters
    /// * payload - Bot-defined invoice payload, 1-128 bytes.
    ///             This will not be displayed to the user,
    ///             use for your internal processes.
    /// * provider_token - Payment provider token, obtained via Botfather
    /// * currency - Three-letter ISO 4217 currency code, see more on currencies
    /// * prices - Price breakdown  (e.g. product price, tax, discount,
    ///            delivery cost, delivery tax, bonus, etc.)
    pub fn new<A, B, C, D, E, F>(
        title: A,
        description: B,
        payload: C,
        provider_token: D,
        currency: E,
        prices: F,
    ) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
        D: Into<String>,
        E: Into<String>,
        F: IntoIterator<Item = LabeledPrice>,
    {
        Self {
            title: title.into(),
            description: description.into(),
            payload: payload.into(),
            provider_token: provider_token.into(),
            currency: currency.into(),
            prices: prices.into_iter().collect(),
            max_tip_amount: None,
            suggested_tip_amounts: None,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
        }
    }

    /// Sets the maximum accepted amount for tips in the smallest units of the currency
    ///
    /// For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145.
    /// See the exp parameter in [currencies.json], it shows the number
    // of digits past the decimal point for each currency
    /// (2 for the majority of currencies). Defaults to 0
    ///
    /// [currencies.json]: https://core.telegram.org/bots/payments/currencies.json
    pub fn max_tip_amount(mut self, value: Integer) -> Self {
        self.max_tip_amount = Some(value);
        self
    }

    /// Sets an array of suggested amounts of tip in the smallest units of the currency
    ///
    /// At most 4 suggested tip amounts can be specified.
    /// The suggested tip amounts must be positive, passed
    /// in a strictly increased order and must not exceed max_tip_amount.
    pub fn suggested_tip_amounts(mut self, value: impl IntoIterator<Item = Integer>) -> Self {
        self.suggested_tip_amounts = Some(value.into_iter().collect());
        self
    }

    /// An object for data about the invoice,
    /// which will be shared with the payment provider.
    /// A detailed description of the required fields should be provided by the payment provider.
    pub fn provider_data<T>(mut self, value: &T) -> Result<Self, JsonError>
    where
        T: Serialize,
    {
        self.provider_data = Some(serde_json::to_string(value)?);
        Ok(self)
    }

    /// Sets an URL of the product photo for the invoice
    ///
    /// Can be a photo of the goods or a marketing image for a service.
    /// People like it better when they see what they are paying for.
    pub fn photo_url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.photo_url = Some(value.into());
        self
    }

    /// Sets a photo size
    pub fn photo_size(mut self, value: Integer) -> Self {
        self.photo_size = Some(value);
        self
    }

    /// Sets a photo width
    pub fn photo_width(mut self, value: Integer) -> Self {
        self.photo_width = Some(value);
        self
    }

    /// Sets a photo height
    pub fn photo_height(mut self, value: Integer) -> Self {
        self.photo_height = Some(value);
        self
    }

    /// Pass True, if you require the user's full name to complete the order
    pub fn need_name(mut self, value: bool) -> Self {
        self.need_name = Some(value);
        self
    }

    /// Pass True, if you require the user's phone number to complete the order
    pub fn need_phone_number(mut self, value: bool) -> Self {
        self.need_phone_number = Some(value);
        self
    }

    /// Pass True, if you require the user's email address to complete the order
    pub fn need_email(mut self, value: bool) -> Self {
        self.need_email = Some(value);
        self
    }

    /// Pass True, if you require the user's shipping address to complete the order
    pub fn need_shipping_address(mut self, value: bool) -> Self {
        self.need_shipping_address = Some(value);
        self
    }

    /// Pass True, if user's phone number should be sent to provider
    pub fn send_phone_number_to_provider(mut self, value: bool) -> Self {
        self.send_phone_number_to_provider = Some(value);
        self
    }

    /// Pass True, if user's email address should be sent to provider
    pub fn send_email_to_provider(mut self, value: bool) -> Self {
        self.send_email_to_provider = Some(value);
        self
    }

    /// Pass True, if the final price depends on the shipping method
    pub fn is_flexible(mut self, value: bool) -> Self {
        self.is_flexible = Some(value);
        self
    }
}

/// Location message to be sent as the result of an inline query
#[derive(Clone, Debug, Serialize)]
pub struct InputMessageContentLocation {
    latitude: Float,
    longitude: Float,
    #[serde(skip_serializing_if = "Option::is_none")]
    horizontal_accuracy: Option<Float>,
    #[serde(skip_serializing_if = "Option::is_none")]
    live_period: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    heading: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    proximity_alert_radius: Option<Integer>,
}

impl InputMessageContentLocation {
    /// Creates a new InputMessageContentLocation with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * latitude - Latitude of the location in degrees
    /// * longitude - Longitude of the location in degrees
    pub fn new(latitude: Float, longitude: Float) -> Self {
        InputMessageContentLocation {
            latitude,
            longitude,
            horizontal_accuracy: None,
            live_period: None,
            heading: None,
            proximity_alert_radius: None,
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

    /// For live locations, a maximum distance for proximity alerts
    /// about approaching another chat member, in meters
    ///
    /// Must be between 1 and 100000 if specified
    pub fn proximity_alert_radius(mut self, proximity_alert_radius: Integer) -> Self {
        self.proximity_alert_radius = Some(proximity_alert_radius);
        self
    }
}

/// Text message to be sent as the result of an inline query
#[derive(Clone, Debug, Serialize)]
pub struct InputMessageContentText {
    message_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    entities: Option<TextEntities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    disable_web_page_preview: Option<bool>,
}

impl InputMessageContentText {
    /// Creates a new InputMessageContentText with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * message_text - Text of the message to be sent, 1-4096 characters
    pub fn new<S: Into<String>>(message_text: S) -> Self {
        InputMessageContentText {
            message_text: message_text.into(),
            entities: None,
            parse_mode: None,
            disable_web_page_preview: None,
        }
    }

    /// List of special entities that appear in the caption
    ///
    /// Parse mode will be set to None when this method is called
    pub fn entities<T>(mut self, entities: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.entities = Some(entities);
        self.parse_mode = None;
        self
    }

    /// Sets parse mode
    ///
    /// Caption entities will be set to None when this method is called
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self.entities = None;
        self
    }

    /// Disables link previews for links in the sent message
    pub fn disable_web_page_preview(mut self, disable_web_page_preview: bool) -> Self {
        self.disable_web_page_preview = Some(disable_web_page_preview);
        self
    }
}

/// Venue message to be sent as the result of an inline query
#[derive(Clone, Debug, Serialize)]
pub struct InputMessageContentVenue {
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
}

impl InputMessageContentVenue {
    /// Creates a new InputMessageContentVenue with empty optional parameters
    ///
    /// # Arguments
    ///
    /// * latitude - Latitude of the venue in degrees
    /// * longitude - Longitude of the venue in degrees
    /// * title - Name of the venue
    /// * address - Address of the venue
    pub fn new<S: Into<String>>(latitude: Float, longitude: Float, title: S, address: S) -> Self {
        InputMessageContentVenue {
            latitude,
            longitude,
            title: title.into(),
            address: address.into(),
            foursquare_id: None,
            foursquare_type: None,
            google_place_id: None,
            google_place_type: None,
        }
    }

    /// Foursquare identifier of the venue, if known
    pub fn foursquare_id<S: Into<String>>(mut self, foursquare_id: S) -> Self {
        self.foursquare_id = Some(foursquare_id.into());
        self
    }

    /// Foursquare type of the venue, if known
    ///
    /// For example, “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”
    pub fn foursquare_type<S: Into<String>>(mut self, foursquare_type: S) -> Self {
        self.foursquare_type = Some(foursquare_type.into());
        self
    }

    /// Google Places identifier of the venue
    pub fn google_place_id<S: Into<String>>(mut self, google_place_id: S) -> Self {
        self.google_place_id = Some(google_place_id.into());
        self
    }

    /// Google Places type of the venue.
    ///
    /// <https://developers.google.com/places/web-service/supported_types>
    pub fn google_place_type<S: Into<String>>(mut self, google_place_type: S) -> Self {
        self.google_place_type = Some(google_place_type.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_contact() {
        assert_eq!(
            serde_json::to_value(InputMessageContent::from(
                InputMessageContentContact::new("+79001231212", "Vasya")
                    .last_name("Pupkin")
                    .vcard("vcard")
            ))
            .unwrap(),
            serde_json::json!({
                "phone_number": "+79001231212",
                "first_name": "Vasya",
                "last_name": "Pupkin",
                "vcard": "vcard"
            })
        );

        assert_eq!(
            serde_json::to_value(InputMessageContent::from(InputMessageContentContact::new(
                "+79001231212",
                "Vasya"
            )))
            .unwrap(),
            serde_json::json!({
                "phone_number": "+79001231212",
                "first_name": "Vasya"
            })
        );
    }

    #[derive(Serialize)]
    struct InvoiceProviderData {
        key: String,
    }

    #[test]
    fn serialize_invoice() {
        let val = serde_json::to_value(InputMessageContent::from(
            InputMessageContentInvoice::new(
                "title",
                "description",
                "payload",
                "provider-token",
                "RUB",
                [LabeledPrice::new("item", 100)],
            )
            .max_tip_amount(1)
            .suggested_tip_amounts([2])
            .provider_data(&InvoiceProviderData {
                key: String::from("value"),
            })
            .unwrap()
            .photo_url("https://google.com/favicon.ico")
            .photo_size(100)
            .photo_width(24)
            .photo_height(24)
            .need_name(true)
            .need_email(false)
            .need_phone_number(true)
            .need_shipping_address(false)
            .send_phone_number_to_provider(true)
            .send_email_to_provider(false)
            .is_flexible(true),
        ))
        .unwrap();
        assert_eq!(val["title"], "title");
        assert_eq!(val["description"], "description");
        assert_eq!(val["payload"], "payload");
        assert_eq!(val["provider_token"], "provider-token");
        assert_eq!(val["currency"], "RUB");
        assert_eq!(val["prices"], serde_json::json!([{"label": "item", "amount": 100}]));
        assert_eq!(val["max_tip_amount"], 1);
        assert_eq!(val["suggested_tip_amounts"].as_array().unwrap(), &[2]);
        assert_eq!(val["provider_data"], r#"{"key":"value"}"#);
        assert_eq!(val["photo_url"], "https://google.com/favicon.ico");
        assert_eq!(val["photo_size"], 100);
        assert_eq!(val["photo_width"], 24);
        assert_eq!(val["photo_height"], 24);
        assert!(val["need_name"].as_bool().unwrap());
        assert!(!val["need_email"].as_bool().unwrap());
        assert!(val["need_phone_number"].as_bool().unwrap());
        assert!(!val["need_shipping_address"].as_bool().unwrap());
        assert!(val["send_phone_number_to_provider"].as_bool().unwrap());
        assert!(!val["send_email_to_provider"].as_bool().unwrap());
        assert!(val["is_flexible"].as_bool().unwrap());

        let val = serde_json::to_value(InputMessageContent::from(InputMessageContentInvoice::new(
            "title",
            "description",
            "payload",
            "provider-token",
            "RUB",
            [LabeledPrice::new("item", 100)],
        )))
        .unwrap();
        assert_eq!(val["title"], "title");
        assert_eq!(val["description"], "description");
        assert_eq!(val["payload"], "payload");
        assert_eq!(val["provider_token"], "provider-token");
        assert_eq!(val["currency"], "RUB");
        assert_eq!(val["prices"], serde_json::json!([{"label": "item", "amount": 100}]));
        assert!(val.get("max_tip_amount").is_none());
        assert!(val.get("suggested_tip_amounts").is_none());
        assert!(val.get("provider_data").is_none());
        assert!(val.get("photo_url").is_none());
        assert!(val.get("photo_size").is_none());
        assert!(val.get("photo_width").is_none());
        assert!(val.get("photo_height").is_none());
        assert!(val.get("need_name").is_none());
        assert!(val.get("need_phone_number").is_none());
        assert!(val.get("need_email").is_none());
        assert!(val.get("need_shipping_address").is_none());
        assert!(val.get("send_phone_number_to_provider").is_none());
        assert!(val.get("send_email_to_provider").is_none());
        assert!(val.get("is_flexible").is_none());
    }

    #[allow(clippy::float_cmp)]
    #[test]
    fn serialize_location() {
        let val = serde_json::to_value(InputMessageContent::from(
            InputMessageContentLocation::new(1.1, 2.1)
                .horizontal_accuracy(1.5)
                .live_period(100)
                .heading(90)
                .proximity_alert_radius(100),
        ))
        .unwrap();
        assert_eq!(val.get("latitude").unwrap().as_f64().unwrap().round(), 1.0);
        assert_eq!(val.get("longitude").unwrap().as_f64().unwrap().round(), 2.0);
        assert_eq!(val.get("horizontal_accuracy").unwrap().as_f64().unwrap(), 1.5);
        assert_eq!(val.get("live_period").unwrap().as_i64().unwrap(), 100);
        assert_eq!(val.get("heading").unwrap().as_i64().unwrap(), 90);
        assert_eq!(val.get("proximity_alert_radius").unwrap().as_i64().unwrap(), 100);

        let val = serde_json::to_value(InputMessageContent::from(InputMessageContentLocation::new(1.1, 2.1))).unwrap();
        assert_eq!(val.get("latitude").unwrap().as_f64().unwrap().round(), 1.0);
        assert_eq!(val.get("longitude").unwrap().as_f64().unwrap().round(), 2.0);
        assert!(val.get("horizontal_accuracy").is_none());
        assert!(val.get("live_period").is_none());
        assert!(val.get("heading").is_none());
        assert!(val.get("proximity_alert_radius").is_none());
    }

    #[test]
    fn serialize_text() {
        assert_eq!(
            serde_json::to_value(InputMessageContent::from(
                InputMessageContentText::new("text")
                    .entities(vec![TextEntity::bold(0..10)])
                    .parse_mode(ParseMode::Html)
                    .disable_web_page_preview(true)
            ))
            .unwrap(),
            serde_json::json!({
                "message_text": "text",
                "parse_mode": "HTML",
                "disable_web_page_preview": true
            })
        );

        assert_eq!(
            serde_json::to_value(InputMessageContent::from(
                InputMessageContentText::new("text")
                    .parse_mode(ParseMode::Markdown)
                    .entities(vec![TextEntity::bold(0..10)])
            ))
            .unwrap(),
            serde_json::json!({
                "message_text": "text",
                "entities": [
                    {
                        "type": "bold",
                        "offset": 0,
                        "length": 10
                    }
                ]
            })
        );
    }

    #[allow(clippy::float_cmp)]
    #[test]
    fn serialize_venue() {
        let val = serde_json::to_value(InputMessageContent::from(
            InputMessageContentVenue::new(1.1, 2.1, "title", "addr")
                .foursquare_id("f-id")
                .foursquare_type("f-type")
                .google_place_id("g-id")
                .google_place_type("g-type"),
        ))
        .unwrap();
        assert_eq!(val.get("latitude").unwrap().as_f64().unwrap().round(), 1.0);
        assert_eq!(val.get("longitude").unwrap().as_f64().unwrap().round(), 2.0);
        assert_eq!(val.get("title").unwrap().as_str().unwrap(), "title");
        assert_eq!(val.get("address").unwrap().as_str().unwrap(), "addr");
        assert_eq!(val.get("foursquare_id").unwrap().as_str().unwrap(), "f-id");
        assert_eq!(val.get("foursquare_type").unwrap().as_str().unwrap(), "f-type");
        assert_eq!(val.get("google_place_id").unwrap().as_str().unwrap(), "g-id");
        assert_eq!(val.get("google_place_type").unwrap().as_str().unwrap(), "g-type");

        let val = serde_json::to_value(InputMessageContent::from(InputMessageContentVenue::new(
            1.1, 2.1, "title", "addr",
        )))
        .unwrap();
        assert_eq!(val.get("latitude").unwrap().as_f64().unwrap().round(), 1.0);
        assert_eq!(val.get("longitude").unwrap().as_f64().unwrap().round(), 2.0);
        assert_eq!(val.get("title").unwrap().as_str().unwrap(), "title");
        assert_eq!(val.get("address").unwrap().as_str().unwrap(), "addr");
        assert!(val.get("foursquare_id").is_none());
        assert!(val.get("foursquare_type").is_none());
        assert!(val.get("google_place_id").is_none());
        assert!(val.get("google_place_type").is_none());
    }
}
