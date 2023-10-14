use serde::{Deserialize, Serialize};

use crate::types::{Float, Venue};

#[cfg(test)]
mod tests;

/// Venue message to be sent as the result of an inline query
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
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
    /// “arts_entertainment/aquarium” or “food/ice-cream”
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

impl From<Venue> for InputMessageContentVenue {
    fn from(value: Venue) -> Self {
        Self {
            latitude: value.location.latitude,
            longitude: value.location.longitude,
            title: value.title,
            address: value.address,
            foursquare_id: value.foursquare_id,
            foursquare_type: value.foursquare_type,
            google_place_id: value.google_place_id,
            google_place_type: value.google_place_type,
        }
    }
}
