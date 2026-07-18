use std::{error, fmt};

/// A form value serialization error kind.
#[derive(Clone, Copy, Debug)]
pub enum SerializeErrorKind {
    /// Can not serialize callback data.
    CallbackData,
    /// Can not serialize inline keyboard markup.
    InlineKeyboardMarkup,
    /// Can not serialize inline query result data.
    InlineQueryResultData,
    /// Can not serialize inline query results button.
    InlineQueryResultsButton,
    /// Can not serialize input rich message data.
    InputRichMessageData,
    /// Can not serialize link preview options.
    LinkPreviewOptions,
    /// Can not serialize text entities.
    TextEntities,
}

impl fmt::Display for SerializeErrorKind {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(
            out,
            "{}",
            match self {
                Self::CallbackData => "callback data",
                Self::InlineKeyboardMarkup => "inline keyboard markup",
                Self::InlineQueryResultData => "inline query result data",
                Self::InlineQueryResultsButton => "inline query results button",
                Self::InputRichMessageData => "input rich message data",
                Self::LinkPreviewOptions => "link preview options",
                Self::TextEntities => "text entities",
            }
        )
    }
}

/// A form value serialization error.
#[derive(Debug)]
pub struct SerializeError {
    /// Serialization error kind.
    pub kind: SerializeErrorKind,
    /// Source error.
    pub inner: serde_json::Error,
}

impl SerializeError {
    pub(crate) fn callback_data(inner: serde_json::Error) -> Self {
        Self {
            kind: SerializeErrorKind::CallbackData,
            inner,
        }
    }

    pub(crate) fn inline_keyboard_markup(inner: serde_json::Error) -> Self {
        Self {
            kind: SerializeErrorKind::InlineKeyboardMarkup,
            inner,
        }
    }

    pub(crate) fn inline_query_result_data(inner: serde_json::Error) -> Self {
        Self {
            kind: SerializeErrorKind::InlineQueryResultData,
            inner,
        }
    }

    pub(crate) fn inline_query_results_button(inner: serde_json::Error) -> Self {
        Self {
            kind: SerializeErrorKind::InlineQueryResultsButton,
            inner,
        }
    }

    pub(crate) fn input_rich_message_data(inner: serde_json::Error) -> Self {
        Self {
            kind: SerializeErrorKind::InputRichMessageData,
            inner,
        }
    }

    pub(crate) fn link_preview_options(inner: serde_json::Error) -> Self {
        Self {
            kind: SerializeErrorKind::LinkPreviewOptions,
            inner,
        }
    }

    pub(crate) fn text_entities(inner: serde_json::Error) -> Self {
        Self {
            kind: SerializeErrorKind::TextEntities,
            inner,
        }
    }
}

impl error::Error for SerializeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.inner)
    }
}

impl fmt::Display for SerializeError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "can not serialize {}: {}", self.kind, self.inner)
    }
}
