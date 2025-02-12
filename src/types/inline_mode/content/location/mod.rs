use serde::{Deserialize, Serialize};

use crate::types::{Float, Integer, Location};

#[cfg(test)]
mod tests;

/// Represents a location message to be sent as the result of an inline query.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputMessageContentLocation {
    latitude: Float,
    longitude: Float,
    heading: Option<Integer>,
    horizontal_accuracy: Option<Float>,
    live_period: Option<Integer>,
    proximity_alert_radius: Option<Integer>,
}

impl InputMessageContentLocation {
    /// Creates a new `InputMessageContentLocation`.
    ///
    /// # Arguments
    ///
    /// * `latitude` - Latitude in degrees.
    /// * `longitude` - Longitude in degrees.
    pub fn new(latitude: Float, longitude: Float) -> Self {
        Self {
            latitude,
            longitude,
            heading: None,
            horizontal_accuracy: None,
            live_period: None,
            proximity_alert_radius: None,
        }
    }

    /// Sets a new heading.
    ///
    /// # Arguments
    ///
    /// * `value` - A direction in which the user is moving; in degrees; 1-360.
    pub fn with_heading(mut self, value: Integer) -> Self {
        self.heading = Some(value);
        self
    }

    /// Sets a new horizontal accuracy.
    ///
    /// # Arguments
    ///
    /// * `value` - A radius of uncertainty for the location; in meters; 0-1500.
    pub fn with_horizontal_accuracy(mut self, value: Float) -> Self {
        self.horizontal_accuracy = Some(value);
        self
    }

    /// Sets a new live period.
    ///
    /// # Arguments
    ///
    /// * `value` - Period in seconds during which the location can be updated,
    ///             should be between 60 and 86400,
    ///             or 0x7FFFFFFF for live locations that can be edited indefinitely.
    pub fn with_live_period(mut self, value: Integer) -> Self {
        self.live_period = Some(value);
        self
    }

    /// Sets a new proximity alert radius.
    ///
    /// # Arguments
    ///
    /// * `value` - A maximum distance for proximity alerts
    ///             about approaching another chat member; in meters; 1-100000.
    pub fn with_proximity_alert_radius(mut self, value: Integer) -> Self {
        self.proximity_alert_radius = Some(value);
        self
    }
}

impl From<Location> for InputMessageContentLocation {
    fn from(value: Location) -> Self {
        Self {
            latitude: value.latitude,
            longitude: value.longitude,
            heading: value.heading,
            horizontal_accuracy: value.horizontal_accuracy,
            live_period: value.live_period,
            proximity_alert_radius: value.proximity_alert_radius,
        }
    }
}
