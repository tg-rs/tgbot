use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Represents the options used for link preview generation.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Default, Deserialize, PartialEq, PartialOrd, Serialize)]
pub struct LinkPreviewOptions {
    /// Whether the link preview is disabled.
    pub is_disabled: Option<bool>,
    /// Whether the media in the link preview is suppposed to be enlarged.
    pub prefer_large_media: Option<bool>,
    /// Whether the media in the link preview is suppposed to be shrunk.
    pub prefer_small_media: Option<bool>,
    /// Whether the link preview must be shown above the message text.
    pub show_above_text: Option<bool>,
    /// URL to use for the link preview.
    pub url: Option<String>,
}

impl LinkPreviewOptions {
    /// Sets a new value for the `is_disabled` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the link preview is disabled.
    pub fn with_is_disabled(mut self, value: bool) -> Self {
        self.is_disabled = Some(value);
        self
    }

    /// Sets a new value for the `prefer_large_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the media in the link preview is suppposed to be enlarged;
    ///   ignored if the URL isn't explicitly specified or media size change isn't supported for the preview.
    pub fn with_prefer_large_media(mut self, value: bool) -> Self {
        self.prefer_large_media = Some(value);
        self
    }

    /// Sets a new value for the `prefer_small_media` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the media in the link preview is suppposed to be shrunk;
    ///   ignored if the URL isn't explicitly specified or media size change isn't supported for the preview.
    pub fn with_prefer_small_media(mut self, value: bool) -> Self {
        self.prefer_small_media = Some(value);
        self
    }

    /// Sets a new value for the `show_above_text` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the link preview must be shown above the message text;
    ///   otherwise, the link preview will be shown below the message text.
    pub fn with_show_above_text(mut self, value: bool) -> Self {
        self.show_above_text = Some(value);
        self
    }

    /// Sets a new URL.
    ///
    /// # Arguments
    ///
    /// * `value` - URL to use for the link preview.
    ///
    /// If empty, then the first URL found in the message text will be used.
    pub fn with_url<T>(mut self, value: T) -> Self
    where
        T: Into<String>,
    {
        self.url = Some(value.into());
        self
    }
}
