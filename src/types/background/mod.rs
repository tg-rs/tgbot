use serde::{Deserialize, Serialize};

use crate::types::{Document, Integer};

#[cfg(test)]
mod tests;

/// Describes the way a background is filled based on the selected colors.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum BackgroundFill {
    /// The background is a freeform gradient that rotates after every message in the chat.
    FreeformGradient {
        /// A list of the 3 or 4 base colors that are used to generate the freeform gradient in the RGB24 format.
        colors: Vec<Integer>,
    },
    /// The background is a gradient fill.
    Gradient {
        /// Bottom color of the gradient in the RGB24 format.
        bottom_color: Integer,
        /// Clockwise rotation angle of the background fill in degrees; 0-359.
        rotation_angle: Integer,
        /// Top color of the gradient in the RGB24 format.
        top_color: Integer,
    },
    /// The background is filled using the selected color.
    Solid {
        /// The color of the background fill in the RGB24 format.
        color: Integer,
    },
}

/// This object describes the type of a background.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum BackgroundType {
    /// The background is taken directly from a built-in chat theme.
    ChatTheme {
        /// Name of the chat theme, which is usually an emoji.
        theme_name: String,
    },
    /// The background is automatically filled based on the selected colors.
    Fill {
        /// Dimming of the background in dark themes, as a percentage; 0-100.
        dark_theme_dimming: Integer,
        /// The background fill.
        fill: BackgroundFill,
    },
    /// The background is a PNG or TGV (gzipped subset of SVG with MIME type “application/x-tgwallpattern”)
    /// pattern to be combined with the background fill chosen by the user.
    Pattern {
        /// Document with the pattern.
        document: Document,
        ///The background fill that is combined with the pattern.
        fill: BackgroundFill,
        /// Intensity of the pattern when it is shown above the filled background; 0-100.
        intensity: Integer,
        ///  Whether the background fill must be applied only to the pattern itself. All other pixels are black in this case. For dark themes only.
        is_inverted: Option<bool>,
        /// Whether the background moves slightly when the device is tilted.
        is_moving: Option<bool>,
    },
    /// The background is a wallpaper in the JPEG format.
    Wallpaper {
        /// Dimming of the background in dark themes, as a percentage; 0-100.
        dark_theme_dimming: Integer,
        /// Document with the wallpaper.
        document: Document,
        /// Whether the wallpaper is downscaled to fit in a 450x450 square and then box-blurred with radius 12.
        is_blurred: Option<bool>,
        /// Whether the background moves slightly when the device is tilted.
        is_moving: Option<bool>,
    },
}
