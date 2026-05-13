use serde::{Deserialize, Serialize};

use crate::types::{ParseMode, TextEntities, TextEntity};

/// Represents a live photo to be sent.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct InputMediaLivePhoto {
    caption: Option<String>,
    parse_mode: Option<ParseMode>,
    caption_entities: Option<TextEntities>,
    show_caption_above_media: Option<bool>,
    has_spoiler: Option<bool>,
}

impl InputMediaLivePhoto {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption of the live photo to be sent;
    ///  0-1024 characters after entities parsing.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.caption = Some(value.into());
        self
    }

    /// Sets a new parse mode.
    ///
    /// # Arguments
    ///
    /// * `value` - Mode for parsing entities in the live photo caption.
    ///   Caption entities will be set to [`None`] when this method is called.
    pub fn with_parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self.caption_entities = None;
        self
    }

    /// Sets a new list of caption entities.
    ///
    /// # Arguments
    ///
    /// * `value` - List of special entities that appear in the caption;
    ///  Parse mode will be set to [`None`] when this method is called.
    pub fn with_caption_entities<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = TextEntity>,
    {
        self.caption_entities = Some(value.into_iter().collect());
        self.parse_mode = None;
        self
    }

    /// Sets a new value for the `show_caption_above_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the caption must be shown above the message media.
    pub fn with_show_caption_above_media(mut self, value: bool) -> Self {
        self.show_caption_above_media = Some(value);
        self
    }

    /// Sets a new value for the `has_spoiler` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the live photo needs to be covered with a spoiler animation.
    pub fn with_has_spoiler(mut self, value: bool) -> Self {
        self.has_spoiler = Some(value);
        self
    }
}
