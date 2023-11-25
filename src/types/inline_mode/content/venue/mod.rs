use serde::{Deserialize, Serialize};

use crate::types::{Float, Venue};

#[cfg(test)]
mod tests;

/// Represents a venue message to be sent as the result of an inline query.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputMessageContentVenue {
    address: String,
    latitude: Float,
    longitude: Float,
    title: String,
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
            address: address.into(),
            latitude,
            longitude,
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

impl From<Venue> for InputMessageContentVenue {
    fn from(value: Venue) -> Self {
        Self {
            address: value.address,
            latitude: value.location.latitude,
            longitude: value.location.longitude,
            title: value.title,
            foursquare_id: value.foursquare_id,
            foursquare_type: value.foursquare_type,
            google_place_id: value.google_place_id,
            google_place_type: value.google_place_type,
        }
    }
}
