use crate::types::location::Location;
use serde::Deserialize;

/// Venue
#[derive(Clone, Debug, Deserialize)]
pub struct Venue {
    /// Venue location
    pub location: Location,
    /// Name of the venue
    pub title: String,
    /// Address of the venue
    pub address: String,
    /// Foursquare identifier of the venue
    pub foursquare_id: Option<String>,
    /// Foursquare type of the venue
    /// For example: “arts_entertainment/default”,
    /// “arts_entertainment/aquarium” or “food/icecream”
    pub foursquare_type: Option<String>,
    /// Google Places identifier of the venue
    pub google_place_id: Option<String>,
    /// Google Places type of the venue
    ///
    /// https://developers.google.com/places/web-service/supported_types
    pub google_place_type: Option<String>,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]
    use super::*;

    #[test]
    fn deserialize_full() {
        let data: Venue = serde_json::from_value(serde_json::json!({
            "location": {
                "latitude": 1.1,
                "longitude": 2.0
            },
            "title": "venue title",
            "address": "venue address",
            "foursquare_id": "f-id",
            "foursquare_type": "f-type",
            "google_place_id": "g-id",
            "google_place_type": "g-type"
        }))
        .unwrap();

        assert_eq!(data.location.latitude, 1.1);
        assert_eq!(data.location.longitude, 2.0);
        assert_eq!(data.title, "venue title");
        assert_eq!(data.address, "venue address");
        assert_eq!(data.foursquare_id.unwrap(), "f-id");
        assert_eq!(data.foursquare_type.unwrap(), "f-type");
        assert_eq!(data.google_place_id.unwrap(), "g-id");
        assert_eq!(data.google_place_type.unwrap(), "g-type");
    }

    #[test]
    fn deserialize_partial() {
        let data: Venue = serde_json::from_value(serde_json::json!({
            "location": {
                "latitude": 1.1,
                "longitude": 2.0
            },
            "title": "venue title",
            "address": "venue address"
        }))
        .unwrap();

        assert_eq!(data.location.latitude, 1.1);
        assert_eq!(data.location.longitude, 2.0);
        assert_eq!(data.title, "venue title");
        assert_eq!(data.address, "venue address");
        assert!(data.foursquare_id.is_none());
        assert!(data.foursquare_type.is_none());
        assert!(data.google_place_id.is_none());
        assert!(data.google_place_type.is_none());
    }
}
