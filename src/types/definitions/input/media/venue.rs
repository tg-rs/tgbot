use serde::{Deserialize, Serialize};

use crate::types::Float;

/// Represents a venue to be sent.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputMediaVenue {
    address: String,
    latitude: Float,
    longitude: Float,
    title: String,
    foursquare_id: Option<String>,
    foursquare_type: Option<String>,
    google_place_id: Option<String>,
    google_place_type: Option<String>,
}

impl InputMediaVenue {
    /// Creates a new `InputMediaVenue`.
    ///
    /// # Arguments
    ///
    /// * `latitude` - Latitude of the location.
    /// * `longitude` - Longitude of the location.
    /// * `title` - Name of the venue.
    /// * `address` - Address of the venue.
    pub fn new<A, B>(latitude: Float, longitude: Float, title: A, address: B) -> Self
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
    /// * `value` - Foursquare type of the venue, if known.
    pub fn with_foursquare_type<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.foursquare_type = Some(value.into());
        self
    }

    /// Sets a new google place ID.
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

    /// Sets a new google place type.
    ///
    /// # Arguments
    ///
    /// * `value` - Google Places type of the venue.
    pub fn with_google_place_type<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.google_place_type = Some(value.into());
        self
    }
}
