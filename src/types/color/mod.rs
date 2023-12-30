use crate::types::Integer;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Represents a color of a forum topic icon.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize)]
#[serde(into = "Integer", from = "Integer")]
pub enum ForumTopicIconColor {
    /// `#FF93B2`
    BakerMillerPink,
    /// `#FB6F5F`
    Bittersweet,
    /// `#CB86DB`
    BrightLavender,
    /// `#FFD67E`
    Jasmine,
    /// `#8EEE98`
    LightGreen,
    /// `#6FB9F0`
    VeryLightAzure,
    /// An unknown color in RGB format.
    Unknown(Integer),
}

impl From<Integer> for ForumTopicIconColor {
    fn from(value: Integer) -> Self {
        use self::ForumTopicIconColor::*;
        match value {
            16749490 => BakerMillerPink,
            16478047 => Bittersweet,
            13338331 => BrightLavender,
            16766590 => Jasmine,
            9367192 => LightGreen,
            7322096 => VeryLightAzure,
            value => Unknown(value),
        }
    }
}

impl From<ForumTopicIconColor> for Integer {
    fn from(value: ForumTopicIconColor) -> Self {
        use self::ForumTopicIconColor::*;
        match value {
            BakerMillerPink => 16749490,
            Bittersweet => 16478047,
            BrightLavender => 13338331,
            Jasmine => 16766590,
            LightGreen => 9367192,
            VeryLightAzure => 7322096,
            Unknown(value) => value,
        }
    }
}
