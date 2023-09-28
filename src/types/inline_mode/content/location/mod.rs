use serde::Serialize;

use crate::types::{Float, Integer};

#[cfg(test)]
mod tests;

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
