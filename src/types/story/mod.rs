use serde::{Deserialize, Serialize};

use crate::types::{Chat, Float, Integer, LocationAddress, ReactionType};

#[cfg(test)]
mod tests;

/// Represents a story.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Story {
    /// Chat that posted the story.
    pub chat: Chat,
    /// Unique identifier of the story in the chat.
    pub id: Integer,
}

impl Story {
    /// Creates a new `Story`.
    ///
    /// # Arguments
    ///
    /// * `chat` - Chat that posted the story.
    /// * `id` - Unique identifier of the story in the chat.
    pub fn new<T>(chat: T, id: Integer) -> Self
    where
        T: Into<Chat>,
    {
        Self { chat: chat.into(), id }
    }

    /// Sets a new chat.
    ///
    /// # Arguments
    ///
    /// * `value` - Chat that posted the story.
    pub fn with_chat<T>(mut self, value: T) -> Self
    where
        T: Into<Chat>,
    {
        self.chat = value.into();
        self
    }

    /// Sets a new ID.
    ///
    /// # Arguments
    ///
    /// `value` - Unique identifier of the story in the chat.
    pub fn with_id(mut self, value: Integer) -> Self {
        self.id = value;
        self
    }
}

/// Describes a clickable area on a story media.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct StoryArea {
    /// Type of the area.
    #[serde(rename = "type")]
    pub area_type: StoryAreaType,
    /// Position of the area
    pub position: StoryAreaPosition,
}

impl StoryArea {
    /// Creates a new `StoryArea`.
    ///
    /// # Arguments
    ///
    /// * `area_type` - Type of the area.
    /// * `position` - Position of the area.
    pub fn new<T>(area_type: T, position: StoryAreaPosition) -> Self
    where
        T: Into<StoryAreaType>,
    {
        Self {
            area_type: area_type.into(),
            position,
        }
    }
}

/// Describes the position of a clickable area within a story.
#[derive(Clone, Debug, Deserialize, derive_more::From, PartialEq, PartialOrd, Serialize)]
pub struct StoryAreaPosition {
    /// The radius of the rectangle corner rounding, as a percentage of the media width.
    pub corner_radius_percentage: Float,
    /// The height of the area's rectangle, as a percentage of the media height.
    pub height_percentage: Float,
    /// The clockwise rotation angle of the rectangle, in degrees; 0-360.
    pub rotation_angle: Float,
    /// The width of the area's rectangle, as a percentage of the media width.
    pub width_percentage: Float,
    /// The abscissa of the area's center, as a percentage of the media width.
    pub x_percentage: Float,
    /// The ordinate of the area's center, as a percentage of the media height.
    pub y_percentage: Float,
}

/// Describes the type of a clickable area on a story.
#[derive(Clone, Debug, Deserialize, derive_more::From, PartialEq, PartialOrd, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StoryAreaType {
    /// An area pointing to an HTTP or tg:// link.
    Link(StoryAreaTypeLink),
    /// An area pointing to a location.
    Location(StoryAreaTypeLocation),
    /// An area pointing to a suggested reaction.
    SuggestedReaction(StoryAreaTypeSuggestedReaction),
    /// An area pointing to a unique gift.
    UniqueGift(StoryAreaTypeUniqueGift),
    /// An area containing weather information.
    Weather(StoryAreaTypeWeather),
}

/// Describes a story area pointing to an HTTP or tg:// link.
///
/// Currently, a story can have up to 3 link areas.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct StoryAreaTypeLink {
    /// HTTP or tg:// URL to be opened when the area is clicked.
    pub url: String,
}

impl StoryAreaTypeLink {
    /// Creates a new `StoryAreaTypeLink`.
    ///
    /// # Arguments
    ///
    /// * `url` - HTTP or tg:// URL to be opened when the area is clicked.
    pub fn new<T>(url: T) -> Self
    where
        T: Into<String>,
    {
        Self { url: url.into() }
    }
}

/// Describes a story area pointing to a location.
///
/// Currently, a story can have up to 10 location areas.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct StoryAreaTypeLocation {
    /// Location latitude in degrees.
    pub latitude: Float,
    /// Location longitude in degrees.
    pub longitude: Float,
    /// Address of the location.
    pub address: Option<LocationAddress>,
}

impl StoryAreaTypeLocation {
    /// Creates a new `StoryAreaTypeLocation`.
    ///
    /// # Arguments
    ///
    /// * `latitude` - Location latitude in degrees.
    /// * `longitude` - Location longitude in degrees.
    pub fn new(latitude: Float, longitude: Float) -> Self {
        Self {
            latitude,
            longitude,
            address: None,
        }
    }

    /// Sets a new address
    ///
    /// # Arguments
    ///
    /// * `value` - Address of the location.
    pub fn with_address(mut self, value: LocationAddress) -> Self {
        self.address = Some(value);
        self
    }
}

/// Describes a story area pointing to a suggested reaction.
///
/// Currently, a story can have up to 5 suggested reaction areas.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct StoryAreaTypeSuggestedReaction {
    /// Type of the reaction.
    pub reaction_type: ReactionType,
    /// Whether the reaction area has a dark background.
    pub is_dark: Option<bool>,
    /// Whether reaction area corner is flipped.
    pub is_flipped: Option<bool>,
}

impl StoryAreaTypeSuggestedReaction {
    /// Creates a new `StoryAreaTypeSuggestedReaction`.
    ///
    /// # Arguments
    ///
    /// * `reaction_type` - Type of the reaction.
    pub fn new(reaction_type: ReactionType) -> Self {
        Self {
            reaction_type,
            is_dark: None,
            is_flipped: None,
        }
    }

    /// Sets a new value for the `is_dark` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the reaction area has a dark background.
    pub fn with_is_dark(mut self, value: bool) -> Self {
        self.is_dark = Some(value);
        self
    }

    /// Sets a new value for the `is_flipped` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether reaction area corner is flipped.
    pub fn with_is_flipped(mut self, value: bool) -> Self {
        self.is_flipped = Some(value);
        self
    }
}

/// Describes a story area pointing to a unique gift.
///
/// Currently, a story can have at most 1 unique gift area.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct StoryAreaTypeUniqueGift {
    /// Unique name of the gift.
    pub name: String,
}

impl StoryAreaTypeUniqueGift {
    /// Creates a new `StoryAreaTypeUniqueGift`.
    ///
    /// # Arguments
    ///
    /// * `name` - Unique name of the gift.
    pub fn new<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self { name: name.into() }
    }
}

/// Describes a story area containing weather information.
///
/// Currently, a story can have up to 3 weather areas.
#[derive(Clone, Debug, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct StoryAreaTypeWeather {
    /// A color of the area background in the ARGB format.
    pub background_color: Integer,
    /// Emoji representing the weather.
    pub emoji: String,
    /// Temperature, in degree Celsius
    pub temperature: Float,
}

impl StoryAreaTypeWeather {
    /// Creates a new `StoryAreaTypeWeather`.
    ///
    /// # Arguments
    ///
    /// * `background_color` - A color of the area background in the ARGB format.
    /// * `emoji` - Emoji representing the weather.
    /// * `temperature` - Temperature, in degree Celsius.
    pub fn new<T>(background_color: Integer, emoji: T, temperature: Float) -> Self
    where
        T: Into<String>,
    {
        Self {
            background_color,
            emoji: emoji.into(),
            temperature,
        }
    }
}
