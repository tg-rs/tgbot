use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;

use crate::types::Float;

#[cfg(test)]
mod tests;

/// Position on faces where a mask should be placed by default
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MaskPosition {
    /// The part of the face relative
    /// to which the mask should be placed
    pub point: MaskPositionPoint,
    /// Shift by X-axis measured in widths
    /// of the mask scaled to the face size,
    /// from left to right.
    /// For example, choosing -1.0
    /// will place mask just
    /// to the left of the default mask position
    pub x_shift: Float,
    /// Shift by Y-axis measured
    /// in heights of the mask scaled to the face size,
    /// from top to bottom.
    /// For example, 1.0 will place
    /// the mask just below the default mask position
    pub y_shift: Float,
    /// Mask scaling coefficient.
    /// For example, 2.0 means double size
    pub scale: Float,
}

impl MaskPosition {
    pub(crate) fn serialize(&self) -> Result<String, MaskPositionError> {
        serde_json::to_string(self).map_err(MaskPositionError::Serialize)
    }
}

/// An error occurred with mask position
#[derive(Debug)]
pub enum MaskPositionError {
    /// Failed to serialize mask position
    Serialize(JsonError),
}

impl Error for MaskPositionError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MaskPositionError::Serialize(err) => Some(err),
        }
    }
}

impl fmt::Display for MaskPositionError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MaskPositionError::Serialize(err) => write!(out, "can not serialize mask position: {}", err),
        }
    }
}

/// The part of the face relative to which the mask should be placed
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MaskPositionPoint {
    /// “forehead”
    Forehead,
    /// “eyes”
    Eyes,
    /// “mouth”
    Mouth,
    /// “chin”
    Chin,
}
