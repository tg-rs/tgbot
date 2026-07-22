use serde::Serialize;
use serde_json::Error as JsonError;

use crate::{
    api::{Form, WriteForm},
    types::{
        Contact,
        Float,
        InputRichMessage,
        InputRichMessageData,
        Integer,
        LabeledPrice,
        LinkPreviewOptions,
        Location,
        ParseMode,
        Text,
        TextEntities,
        TextEntity,
        Venue,
    },
};

/// Represents a content of a message to be sent as a result of an inline query.
#[derive(Debug)]
pub struct InputMessageContent {
    data: InputMessageContentData,
    input_rich_message: Option<InputRichMessage>,
}

impl InputMessageContent {
    fn new(data: InputMessageContentData) -> Self {
        Self {
            data,
            input_rich_message: None,
        }
    }

    fn with_input_rich_message(mut self, value: InputRichMessage) -> Self {
        self.input_rich_message = Some(value);
        self
    }
}

impl WriteForm for InputMessageContent {
    type Output = InputMessageContentData;

    fn write(self, form: &mut Form) -> Self::Output {
        let Self {
            mut data,
            input_rich_message,
        } = self;
        data.rich_message = input_rich_message.map(|x| x.write(form));
        data
    }
}

impl From<Contact> for InputMessageContent {
    fn from(value: Contact) -> Self {
        Self::new(InputMessageContentData {
            first_name: Some(value.first_name),
            phone_number: Some(value.phone_number),
            last_name: value.last_name,
            vcard: value.vcard,
            ..Default::default()
        })
    }
}

impl From<InputMessageContentContact> for InputMessageContent {
    fn from(value: InputMessageContentContact) -> Self {
        Self::new(value.data)
    }
}

impl From<InputMessageContentInvoice> for InputMessageContent {
    fn from(value: InputMessageContentInvoice) -> Self {
        Self::new(value.data)
    }
}

impl From<InputMessageContentText> for InputMessageContent {
    fn from(value: InputMessageContentText) -> Self {
        Self::new(value.data)
    }
}

impl From<InputMessageContentVenue> for InputMessageContent {
    fn from(value: InputMessageContentVenue) -> Self {
        Self::new(value.data)
    }
}

impl From<Location> for InputMessageContent {
    fn from(value: Location) -> Self {
        Self::new(InputMessageContentData {
            latitude: Some(value.latitude),
            longitude: Some(value.longitude),
            heading: value.heading,
            horizontal_accuracy: value.horizontal_accuracy,
            live_period: value.live_period,
            proximity_alert_radius: value.proximity_alert_radius,
            ..Default::default()
        })
    }
}

impl<T> From<T> for InputMessageContent
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self::new(InputMessageContentText::from(value).data)
    }
}

impl From<Text> for InputMessageContent {
    fn from(value: Text) -> Self {
        Self::new(InputMessageContentText::from(value).data)
    }
}

impl From<InputRichMessage> for InputMessageContent {
    fn from(value: InputRichMessage) -> Self {
        Self::new(Default::default()).with_input_rich_message(value)
    }
}

impl From<Venue> for InputMessageContent {
    fn from(value: Venue) -> Self {
        Self::new(InputMessageContentData {
            address: Some(value.address),
            latitude: Some(value.location.latitude),
            longitude: Some(value.location.longitude),
            title: Some(value.title),
            foursquare_id: value.foursquare_id,
            foursquare_type: value.foursquare_type,
            google_place_id: value.google_place_id,
            google_place_type: value.google_place_type,
            ..Default::default()
        })
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub(crate) struct InputMessageContentData {
    address: Option<String>,
    currency: Option<String>,
    description: Option<String>,
    entities: Option<TextEntities>,
    first_name: Option<String>,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
    heading: Option<Integer>,
    horizontal_accuracy: Option<Float>,
    is_flexible: Option<bool>,
    last_name: Option<String>,
    latitude: Option<Float>,
    link_preview_options: Option<LinkPreviewOptions>,
    live_period: Option<Integer>,
    longitude: Option<Float>,
    max_tip_amount: Option<Integer>,
    message_text: Option<String>,
    need_email: Option<bool>,
    need_name: Option<bool>,
    need_phone_number: Option<bool>,
    need_shipping_address: Option<bool>,
    parse_mode: Option<ParseMode>,
    payload: Option<String>,
    phone_number: Option<String>,
    photo_height: Option<Integer>,
    photo_size: Option<Integer>,
    photo_url: Option<String>,
    photo_width: Option<Integer>,
    prices: Option<Vec<LabeledPrice>>,
    provider_data: Option<String>,
    provider_token: Option<String>,
    proximity_alert_radius: Option<Integer>,
    rich_message: Option<InputRichMessageData>,
    send_email_to_provider: Option<bool>,
    send_phone_number_to_provider: Option<bool>,
    suggested_tip_amounts: Option<Vec<Integer>>,
    title: Option<String>,
    vcard: Option<String>,
}

/// Represents a contact message to be sent as the result of an inline query.
#[derive(Debug)]
pub struct InputMessageContentContact {
    data: InputMessageContentData,
}

impl InputMessageContentContact {
    /// Creates a new `InputMessageContentContact`.
    ///
    /// # Arguments
    ///
    /// * `first_name` - The first name of the contact.
    /// * `phone_numer` - The phone number of the contact.
    pub fn new<A, B>(first_name: A, phone_number: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            data: InputMessageContentData {
                phone_number: Some(phone_number.into()),
                first_name: Some(first_name.into()),
                ..Default::default()
            },
        }
    }

    /// Sets a new last name.
    ///
    /// # Arguments
    ///
    /// * `value` - The last name of the contact.
    pub fn with_last_name<T>(mut self, last_name: T) -> Self
    where
        T: Into<String>,
    {
        self.data.last_name = Some(last_name.into());
        self
    }

    /// Sets a new vCard.
    ///
    /// # Arguments
    ///
    /// * `value` - Additional data about the contact in the form of a vCard; 0-2048 bytes.
    pub fn with_vcard<T>(mut self, vcard: T) -> Self
    where
        T: Into<String>,
    {
        self.data.vcard = Some(vcard.into());
        self
    }
}

/// Represents an invoice message to be sent as the result of an inline query.
#[derive(Debug)]
pub struct InputMessageContentInvoice {
    data: InputMessageContentData,
}

impl InputMessageContentInvoice {
    /// Creates a new `InputMessageContentInvoice`.
    ///
    /// # Arguments
    ///
    /// * `currency` - Three-letter ISO 4217 currency code.
    /// * `description` - Product description; 1-255 characters.
    /// * `payload` - Bot-defined invoice payload; 1-128 bytes;
    ///   this will not be displayed to the user,
    ///   use for your internal processes.
    /// * `prices` - Price breakdown (e.g. product price, tax, discount,
    ///   delivery cost, delivery tax, bonus, etc.).
    /// * `title` - Product name; 1-32 characters.
    pub fn new<A, B, C, D, E>(currency: A, description: B, payload: C, prices: D, title: E) -> Self
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<String>,
        D: IntoIterator<Item = LabeledPrice>,
        E: Into<String>,
    {
        Self {
            data: InputMessageContentData {
                currency: Some(currency.into()),
                description: Some(description.into()),
                payload: Some(payload.into()),
                prices: Some(prices.into_iter().collect()),
                title: Some(title.into()),
                ..Default::default()
            },
        }
    }

    /// Sets a new value for the `is_flexible` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether the final price depends on the shipping method.
    pub fn with_is_flexible(mut self, value: bool) -> Self {
        self.data.is_flexible = Some(value);
        self
    }

    /// Sets a new max tip amount.
    ///
    /// # Arguments
    ///
    /// * `value` - Maximum accepted amount for tips in the smallest units of the currency; default - 0.
    ///
    /// For example, for a maximum tip of US$ 1.45 pass max_tip_amount = 145.
    /// See the exp parameter in [currencies.json], it shows the number
    /// of digits past the decimal point for each currency
    /// (2 for the majority of currencies).
    ///
    /// [currencies.json]: https://core.telegram.org/bots/payments/currencies.json
    pub fn with_max_tip_amount(mut self, value: Integer) -> Self {
        self.data.max_tip_amount = Some(value);
        self
    }

    /// Sets a new value for the `need_email` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether an email address of a user is required to complete the order.
    pub fn with_need_email(mut self, value: bool) -> Self {
        self.data.need_email = Some(value);
        self
    }

    /// Sets a new value for the `need_name` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether a full name of a user is required to complete the order.
    pub fn with_need_name(mut self, value: bool) -> Self {
        self.data.need_name = Some(value);
        self
    }

    /// Sets a new value for the `need_phone_number` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether a phone number of a user is required to complete the order.
    pub fn with_need_phone_number(mut self, value: bool) -> Self {
        self.data.need_phone_number = Some(value);
        self
    }

    /// Sets a new value for the `need_shipping_address` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether a shipping address of a user
    ///   is required to complete the order.
    pub fn with_need_shipping_address(mut self, value: bool) -> Self {
        self.data.need_shipping_address = Some(value);
        self
    }

    /// Sets a new photo height.
    ///
    /// # Arguments
    ///
    /// * `value` - Photo height.
    pub fn with_photo_height(mut self, value: Integer) -> Self {
        self.data.photo_height = Some(value);
        self
    }

    /// Sets a new photo size.
    ///
    /// # Arguments
    ///
    /// * `value` - Photo size in bytes.
    pub fn with_photo_size(mut self, value: Integer) -> Self {
        self.data.photo_size = Some(value);
        self
    }

    /// Sets a new photo width.
    ///
    /// # Arguments
    ///
    /// * `value` - Photo width.
    pub fn with_photo_width(mut self, value: Integer) -> Self {
        self.data.photo_width = Some(value);
        self
    }

    /// Sets a new photo URL.
    ///
    /// # Arguments
    ///
    /// * `value` - URL of the product photo for the invoice.
    ///
    /// Can be a photo of the goods or a marketing image for a service.
    /// People like it better when they see what they are paying for.
    pub fn with_photo_url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.photo_url = Some(value.into());
        self
    }

    /// Sets a new provider data.
    ///
    /// # Arguments
    ///
    /// * `value` - An object for data about the invoice,
    ///   which will be shared with the payment provider.
    ///
    /// A detailed description of the required fields should be provided by the payment provider.
    pub fn with_provider_data<T>(mut self, value: &T) -> Result<Self, JsonError>
    where
        T: Serialize,
    {
        self.data.provider_data = Some(serde_json::to_string(value)?);
        Ok(self)
    }

    /// Sets a new provider token.
    ///
    /// # Arguments
    ///
    /// * `value` - Payment provider token, obtained via @BotFather;
    ///   pass an empty string for payments in Telegram Stars.
    pub fn with_provider_token<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.data.provider_token = Some(value.into());
        self
    }

    /// Sets a new value for the `send_email_to_provider` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether an email address of a user should be sent to provider.
    pub fn with_send_email_to_provider(mut self, value: bool) -> Self {
        self.data.send_email_to_provider = Some(value);
        self
    }

    /// Sets a new value for the `send_phone_number_to_provider` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Indicates whether a phone number of a user should be sent to provider.
    pub fn with_send_phone_number_to_provider(mut self, value: bool) -> Self {
        self.data.send_phone_number_to_provider = Some(value);
        self
    }

    /// Sets a new suggested tip amounts.
    ///
    /// # Arguments
    ///
    /// * `value` - Array of suggested amounts of tip in the smallest units of the currency.
    ///
    /// At most 4 suggested tip amounts can be specified.
    /// The suggested tip amounts must be positive, passed
    /// in a strictly increased order and must not exceed `max_tip_amount`.
    pub fn with_suggested_tip_amounts<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = Integer>,
    {
        self.data.suggested_tip_amounts = Some(value.into_iter().collect());
        self
    }
}

/// Represents a text message to be sent as the result of an inline query.
#[derive(Debug)]
pub struct InputMessageContentText {
    data: InputMessageContentData,
}

impl InputMessageContentText {
    /// Creates a new `InputMessageContentText`.
    ///
    /// # Arguments
    ///
    /// * `value` - Text; 1-4096 characters.
    pub fn new<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            data: InputMessageContentData {
                message_text: Some(value.into()),
                ..Default::default()
            },
        }
    }

    /// Sets a new list of entities.
    ///
    /// # Arguments
    ///
    /// * `value` - List of special entities that appear in the text.
    ///
    /// Parse mode will be set to [`None`] when this method is called.
    pub fn with_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.data.entities = Some(value.into_iter().collect());
        self.data.parse_mode = None;
        self
    }

    /// Sets a new link preview options.
    ///
    /// # Arguments
    ///
    /// * `value` - Link preview generation options for the message.
    pub fn with_link_preview_options(mut self, value: LinkPreviewOptions) -> Self {
        self.data.link_preview_options = Some(value);
        self
    }

    /// Sets a new parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Parse mode.
    ///
    /// Entities will be set to [`None`] when this method is called.
    pub fn with_parse_mode(mut self, value: ParseMode) -> Self {
        self.data.parse_mode = Some(value);
        self.data.entities = None;
        self
    }
}

impl<T> From<T> for InputMessageContentText
where
    T: Into<String>,
{
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl From<Text> for InputMessageContentText {
    fn from(value: Text) -> Self {
        let mut result = Self::new(value.data);
        if let Some(entities) = value.entities {
            result = result.with_entities(entities);
        }
        result
    }
}

/// Represents a venue message to be sent as the result of an inline query.
#[derive(Debug)]
pub struct InputMessageContentVenue {
    data: InputMessageContentData,
}

impl InputMessageContentVenue {
    /// Creates a new `InputMessageContentVenue`.
    ///
    /// # Arguments
    ///
    /// * `address` - Address of the venue.
    /// * `latitude` - Latitude of the venue in degrees.
    /// * `longitude` - Longitude of the venue in degrees.
    /// * `title` - Name of the venue.
    pub fn new<A, B>(address: A, latitude: Float, longitude: Float, title: B) -> Self
    where
        A: Into<String>,
        B: Into<String>,
    {
        Self {
            data: InputMessageContentData {
                address: Some(address.into()),
                latitude: Some(latitude),
                longitude: Some(longitude),
                title: Some(title.into()),
                ..Default::default()
            },
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
        self.data.foursquare_id = Some(value.into());
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
        self.data.foursquare_type = Some(value.into());
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
        self.data.google_place_id = Some(value.into());
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
        self.data.google_place_type = Some(value.into());
        self
    }
}
