use serde::{Deserialize, Serialize};

use crate::types::Float;

/// Represents a position on faces where a mask should be placed by default.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct MaskPosition {
    /// The part of the face relative
    /// to which the mask should be placed.
    pub point: MaskPositionPoint,
    /// Mask scaling coefficient.
    ///
    /// For example, 2.0 means double size.
    pub scale: Float,
    /// Shift by X-axis measured in widths
    /// of the mask scaled to the face size,
    /// from left to right.
    ///
    /// For example, choosing -1.0
    /// will place mask just
    /// to the left of the default mask position.
    pub x_shift: Float,
    /// Shift by Y-axis measured
    /// in heights of the mask scaled to the face size,
    /// from top to bottom.
    ///
    /// For example, 1.0 will place
    /// the mask just below the default mask position.
    pub y_shift: Float,
}

impl MaskPosition {
    /// Creates a new `MaskPosition`.
    ///
    /// # Arguments
    ///
    /// * `point` - The part of the face.
    /// * `scale` - Mask scaling coefficient.
    /// * `x_shift` - Shift by X-axis.
    /// * `y_shift` - Shift by Y-axis.
    pub fn new(point: MaskPositionPoint, scale: Float, x_shift: Float, y_shift: Float) -> Self {
        Self {
            point,
            scale,
            x_shift,
            y_shift,
        }
    }
}

/// Represents a part of the face relative to which the mask should be placed.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum MaskPositionPoint {
    /// A chin.
    Chin,
    /// An eyes.
    Eyes,
    /// A forehead.
    Forehead,
    /// A mouth.
    Mouth,
}
