use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};
use serde_json::Error as JsonError;

use crate::{
    api::{Method, Payload},
    types::{Chat, Float, Integer, LocationAddress, ReactionType},
};

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

/// Describes a list of clickable areas on a story media.
pub struct StoryAreas {
    items: Vec<StoryArea>,
}

impl StoryAreas {
    pub(crate) fn serialize(&self) -> Result<String, StoryAreasError> {
        serde_json::to_string(&self.items).map_err(StoryAreasError::Serialize)
    }
}

impl<T> From<T> for StoryAreas
where
    T: IntoIterator<Item = StoryArea>,
{
    fn from(value: T) -> Self {
        Self {
            items: value.into_iter().collect(),
        }
    }
}

/// Represents a story areas error
#[derive(Debug)]
pub enum StoryAreasError {
    /// Can not serialize to JSON
    Serialize(JsonError),
}

impl fmt::Display for StoryAreasError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Serialize(err) => write!(out, "can not serialize: {err}"),
        }
    }
}

impl Error for StoryAreasError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(match self {
            Self::Serialize(err) => err,
        })
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

/// Reposts a story on behalf of a business account from another business account.
///
/// Both business accounts must be managed by the same bot,
/// and the story on the source account must have been posted (or reposted) by the bot.
///
/// Requires the `can_manage_stories` business bot right for both business accounts.
#[derive(Clone, Debug, Serialize)]
pub struct RepostStory {
    active_period: Integer,
    business_connection_id: String,
    from_chat_id: Integer,
    from_story_id: Integer,
    post_to_chat_page: Option<bool>,
    protect_content: Option<bool>,
}

impl RepostStory {
    /// Creates a new `RepostStory`.
    ///
    /// # Arguments
    ///
    /// * `active_period` - Period after which the story is moved to the archive, in seconds; must be one of 6 * 3600, 12 * 3600, 86400, or 2 * 86400.
    /// * `business_connection_id` - Unique identifier of the business connection.
    /// * `from_chat_id` - Unique identifier of the chat which posted the story that should be reposted.
    /// * `from_story_id` - Unique identifier of the story that should be reposted.
    pub fn new<T>(
        active_period: Integer,
        business_connection_id: T,
        from_chat_id: Integer,
        from_story_id: Integer,
    ) -> Self
    where
        T: Into<String>,
    {
        Self {
            active_period,
            business_connection_id: business_connection_id.into(),
            from_chat_id,
            from_story_id,
            post_to_chat_page: None,
            protect_content: None,
        }
    }
    /// Sets a new value for the `post_to_chat_page` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether to keep the story accessible after it expires.
    pub fn with_post_to_chat_page(mut self, value: bool) -> Self {
        self.post_to_chat_page = Some(value);
        self
    }
    /// Sets a new value for the `protect_content` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the content of the story must be protected from forwarding and screenshotting.
    pub fn with_protect_content(mut self, value: bool) -> Self {
        self.protect_content = Some(value);
        self
    }
}

impl Method for RepostStory {
    type Response = Story;

    fn into_payload(self) -> Payload {
        Payload::json("repostStory", self)
    }
}
