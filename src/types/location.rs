use crate::types::{
    primitive::{Float, Integer},
    user::User,
};
use serde::Deserialize;

/// Point on the map
#[derive(Clone, Copy, Debug, Deserialize)]
pub struct Location {
    /// Longitude as defined by sender
    pub longitude: Float,
    /// Latitude as defined by sender
    pub latitude: Float,
    /// The radius of uncertainty for the location, measured in meters; 0-1500
    pub horizontal_accuracy: Option<Float>,
    /// Time relative to the message sending date,
    /// during which the location can be updated, in seconds
    ///
    /// For active live locations only
    pub live_period: Option<Integer>,
    /// The direction in which user is moving, in degrees; 1-360
    ///
    /// For active live locations only
    pub heading: Option<Integer>,
    /// Maximum distance for proximity alerts about
    /// approaching another chat member, in meters
    ///
    /// For sent live locations only
    pub proximity_alert_radius: Option<Integer>,
}

/// Represents the content of a service message,
/// sent whenever a user in the chat triggers
/// a proximity alert set by another user
#[derive(Clone, Debug, Deserialize)]
pub struct ProximityAlertTriggered {
    /// User that triggered the alert
    pub traveler: User,
    /// User that set the alert
    pub watcher: User,
    /// The distance between the users
    pub distance: Integer,
}

#[cfg(test)]
mod tests {
    #![allow(clippy::float_cmp)]
    use super::*;

    #[test]
    fn deserialize_location_full() {
        let data: Location = serde_json::from_value(serde_json::json!({
            "longitude": 2.5,
            "latitude": 2.6,
            "horizontal_accuracy": 0.4,
            "live_period": 1,
            "heading": 45,
            "proximity_alert_radius": 20
        }))
        .unwrap();
        assert_eq!(data.longitude, 2.5);
        assert_eq!(data.latitude, 2.6);
        assert_eq!(data.horizontal_accuracy.unwrap(), 0.4);
        assert_eq!(data.live_period.unwrap(), 1);
        assert_eq!(data.heading.unwrap(), 45);
        assert_eq!(data.proximity_alert_radius.unwrap(), 20);
    }

    #[test]
    fn deserialize_location_partial() {
        let data: Location = serde_json::from_value(serde_json::json!({
            "longitude": 2.5,
            "latitude": 2.6,
        }))
        .unwrap();
        assert_eq!(data.longitude, 2.5);
        assert_eq!(data.latitude, 2.6);
        assert!(data.horizontal_accuracy.is_none());
        assert!(data.live_period.is_none());
        assert!(data.heading.is_none());
        assert!(data.proximity_alert_radius.is_none());
    }

    #[test]
    fn deserialize_proximity_alert_triggered() {
        let data: ProximityAlertTriggered = serde_json::from_value(serde_json::json!({
            "traveler": {
                "id": 1,
                "first_name": "1",
                "is_bot": false
            },
            "watcher": {
                "id": 2,
                "first_name": "2",
                "is_bot": false
            },
            "distance": 10
        }))
        .unwrap();
        assert_eq!(data.traveler.id, 1);
        assert_eq!(data.watcher.id, 2);
        assert_eq!(data.distance, 10);
    }
}
