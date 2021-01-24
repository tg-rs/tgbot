use crate::types::{Integer, ParseMode, TextEntity};
use serde::Serialize;

/// Video to be sent
#[derive(Clone, Default, Debug, Serialize)]
pub struct InputMediaVideo {
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<Vec<TextEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    supports_streaming: Option<bool>,
}

impl InputMediaVideo {
    /// Caption of the video to be sent, 0-1024 characters
    pub fn caption<S: Into<String>>(mut self, caption: S) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// List of special entities that appear in the caption
    ///
    /// Parse mode will be set to None when this method is called
    pub fn caption_entities(mut self, caption_entities: Vec<TextEntity>) -> Self {
        self.caption_entities = Some(caption_entities);
        self.parse_mode = None;
        self
    }

    /// Sets parse mode
    ///
    /// Caption entities will be set to None when this method is called
    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self.caption_entities = None;
        self
    }

    /// Set width
    pub fn width(mut self, width: Integer) -> Self {
        self.width = Some(width);
        self
    }

    /// Set height
    pub fn height(mut self, height: Integer) -> Self {
        self.height = Some(height);
        self
    }

    /// Set duration
    pub fn duration(mut self, duration: Integer) -> Self {
        self.duration = Some(duration);
        self
    }

    /// Pass True, if the uploaded video is suitable for streaming
    pub fn supports_streaming(mut self, supports_streaming: bool) -> Self {
        self.supports_streaming = Some(supports_streaming);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_value(
                InputMediaVideo::default()
                    .caption("caption")
                    .parse_mode(ParseMode::Markdown)
                    .width(200)
                    .height(200)
                    .duration(100)
                    .supports_streaming(true)
            )
            .unwrap(),
            serde_json::json!({
                "caption": "caption",
                "parse_mode": "Markdown",
                "width": 200,
                "height": 200,
                "duration": 100,
                "supports_streaming": true
            })
        );

        assert_eq!(
            serde_json::to_value(InputMediaVideo::default()).unwrap(),
            serde_json::json!({})
        );
    }

    #[test]
    fn caption_entities_vs_parse_mode() {
        let mut method = InputMediaVideo::default();
        method = method.parse_mode(ParseMode::Markdown);
        assert_eq!(method.parse_mode.unwrap(), ParseMode::Markdown);
        assert!(method.caption_entities.is_none());
        method = method.caption_entities(vec![TextEntity::bold(0..10)]);
        assert!(method.caption_entities.is_some());
        assert!(method.parse_mode.is_none());
    }
}
