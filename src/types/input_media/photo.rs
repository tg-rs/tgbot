use crate::types::{ParseMode, TextEntity};
use serde::Serialize;

/// Photo to be sent
#[derive(Clone, Default, Debug, Serialize)]
pub struct InputMediaPhoto {
    #[serde(skip_serializing_if = "Option::is_none")]
    caption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    caption_entities: Option<Vec<TextEntity>>,
}

impl InputMediaPhoto {
    /// Caption of the photo to be sent, 0-1024 characters
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize() {
        assert_eq!(
            serde_json::to_value(
                InputMediaPhoto::default()
                    .caption("caption")
                    .parse_mode(ParseMode::Markdown)
            )
            .unwrap(),
            serde_json::json!({
                "caption": "caption",
                "parse_mode": "Markdown"
            })
        );

        assert_eq!(
            serde_json::to_value(InputMediaPhoto::default()).unwrap(),
            serde_json::json!({})
        );
    }

    #[test]
    fn caption_entities_vs_parse_mode() {
        let mut method = InputMediaPhoto::default();
        method = method.parse_mode(ParseMode::Markdown);
        assert_eq!(method.parse_mode.unwrap(), ParseMode::Markdown);
        assert!(method.caption_entities.is_none());
        method = method.caption_entities(vec![TextEntity::bold(0..10)]);
        assert!(method.caption_entities.is_some());
        assert!(method.parse_mode.is_none());
    }
}
