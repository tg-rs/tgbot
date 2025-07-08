use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

use crate::types::Integer;

/// Represents an accent color.
///
/// See the [documentation][1] for more information.
///
/// [1]: https://core.telegram.org/bots/api#accent-colors
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize)]
#[serde(into = "Integer", try_from = "Integer")]
pub enum AccentColor {
    /// Color ID: 5.
    Blue,
    /// Color ID: 4.
    Cyan,
    /// Color ID: 3.
    Green,
    /// Color ID: 1.
    Orange,
    /// Color ID: 6.
    Pink,
    /// Color ID: 2.
    Purple,
    /// Color ID: 0.
    Red,
    /// Color ID: 7-20.
    Unknown(Integer),
}

impl TryFrom<Integer> for AccentColor {
    type Error = AccentColorError;

    fn try_from(value: Integer) -> Result<Self, Self::Error> {
        use self::AccentColor::*;
        Ok(match value {
            0 => Red,
            1 => Orange,
            2 => Purple,
            3 => Green,
            4 => Cyan,
            5 => Blue,
            6 => Pink,
            7..=20 => Unknown(value),
            _ => return Err(AccentColorError::UnknownId(value)),
        })
    }
}

impl From<AccentColor> for Integer {
    fn from(value: AccentColor) -> Self {
        use self::AccentColor::*;
        match value {
            Red => 0,
            Orange => 1,
            Purple => 2,
            Green => 3,
            Cyan => 4,
            Blue => 5,
            Pink => 6,
            Unknown(value) => value,
        }
    }
}

/// Represents an accent color error.
#[derive(Debug)]
pub enum AccentColorError {
    /// Got an unknown color ID.
    UnknownId(Integer),
}

impl Error for AccentColorError {}

impl fmt::Display for AccentColorError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AccentColorError::UnknownId(value) => write!(out, "unknown accent color ID: {value}"),
        }
    }
}

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

/// Represents a profile accent color.
///
/// See the [documentation][1] for more information.
///
/// [1]: https://core.telegram.org/bots/api#profile-accent-colors
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Hash, Serialize)]
#[serde(into = "Integer", try_from = "Integer")]
pub struct ProfileAccentColor(Integer);

impl TryFrom<Integer> for ProfileAccentColor {
    type Error = ProfileAccentColorError;

    fn try_from(value: Integer) -> Result<Self, Self::Error> {
        if !(0..=15).contains(&value) {
            Err(ProfileAccentColorError::UnknownId(value))
        } else {
            Ok(Self(value))
        }
    }
}

impl From<ProfileAccentColor> for Integer {
    fn from(value: ProfileAccentColor) -> Self {
        value.0
    }
}

/// Represents a profile accent color error.
#[derive(Debug)]
pub enum ProfileAccentColorError {
    /// Got an unknown color ID.
    UnknownId(Integer),
}

impl Error for ProfileAccentColorError {}

impl fmt::Display for ProfileAccentColorError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProfileAccentColorError::UnknownId(value) => write!(out, "unknown profile accent color ID: {value}"),
        }
    }
}
