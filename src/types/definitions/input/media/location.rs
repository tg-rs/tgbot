use serde::{Deserialize, Serialize};

use crate::types::Float;

/// Represents a location to be sent.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputMediaLocation {
    latitude: Float,
    longitude: Float,
    horizontal_accuracy: Option<Float>,
}

impl InputMediaLocation {
    /// Creates a new `InputMediaLocation`.
    ///
    /// # Arguments
    ///
    /// * `latitude` - Latitude of the location.
    /// * `longitude` - Longitude of the location.
    pub fn new(latitude: Float, longitude: Float) -> Self {
        Self {
            latitude,
            longitude,
            horizontal_accuracy: None,
        }
    }

    /// Sets a new horizontal accuracy.
    ///
    /// # Arguments
    ///
    /// * `value` - The radius of uncertainty for the location, measured in meters; 0-1500
    pub fn with_horizontal_accuracy(mut self, value: Float) -> Self {
        self.horizontal_accuracy = Some(value);
        self
    }
}
