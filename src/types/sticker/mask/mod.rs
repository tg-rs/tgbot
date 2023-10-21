use serde::{Deserialize, Serialize};

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
