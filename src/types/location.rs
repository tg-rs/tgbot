use crate::types::primitive::{Float, Integer};
use serde::Deserialize;

/// Point on the map
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: Float,
    /// Latitude as defined by sender
    pub latitude: Float,
    /// Time relative to the message sending date,
    /// during which the location can be updated, in seconds
    ///
    /// For active live locations only
    pub live_period: Option<Integer>,
    /// The direction in which user is moving, in degrees; 1-360
    ///
    /// For active live locations only
    pub heading: Option<Integer>,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]
    use super::*;

    #[test]
    fn deserialize_full() {
        let data: Location = serde_json::from_value(serde_json::json!({
            "longitude": 2.5,
            "latitude": 2.6,
            "live_period": 1,
            "heading": 45
        }))
        .unwrap();
        assert_eq!(data.longitude, 2.5);
        assert_eq!(data.latitude, 2.6);
        assert_eq!(data.live_period.unwrap(), 1);
        assert_eq!(data.heading.unwrap(), 45);
    }

    #[test]
    fn deserialize_partial() {
        let data: Location = serde_json::from_value(serde_json::json!({
            "longitude": 2.5,
            "latitude": 2.6,
        }))
        .unwrap();
        assert_eq!(data.longitude, 2.5);
        assert_eq!(data.latitude, 2.6);
        assert!(data.live_period.is_none());
        assert!(data.heading.is_none());
    }
}
