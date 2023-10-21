use std::{fmt, fmt::Formatter};

use serde::Serialize;
use serde_json::Error as JsonError;

use crate::{
    api::Form,
    types::{InputFile, InputFileKind, MaskPosition},
};

#[cfg(test)]
mod tests;

/// A metadata of a sticker to be added to a sticker set
#[derive(Debug)]
pub struct InputSticker {
    emoji_list: Vec<String>,
    keywords: Option<Vec<String>>,
    mask_position: Option<MaskPosition>,
    sticker: InputFile,
}

impl InputSticker {
    /// Creates a new InputStickerMetadata
    ///
    /// # Arguments
    ///
    /// * sticker - The added sticker
    /// * emoji_list - List of 1-20 emoji associated with the sticker
    pub fn new<A, B, C>(sticker: A, emoji_list: B) -> Self
    where
        A: Into<InputFile>,
        B: IntoIterator<Item = C>,
        C: Into<String>,
    {
        Self {
            emoji_list: emoji_list.into_iter().map(Into::into).collect(),
            keywords: None,
            mask_position: None,
            sticker: sticker.into(),
        }
    }

    /// List of 0-20 search keywords for the sticker with total length of up to 64 characters
    ///
    /// For “regular” and “custom_emoji” stickers only.
    pub fn with_keywords<A, B>(mut self, value: A) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<String>,
    {
        self.keywords = Some(value.into_iter().map(Into::into).collect());
        self
    }

    /// Position where the mask should be placed on faces. For “mask” stickers only.
    pub fn with_mask_position(mut self, value: MaskPosition) -> Self {
        self.mask_position = Some(value);
        self
    }
}

impl TryFrom<InputSticker> for Form {
    type Error = InputStickerError;

    fn try_from(value: InputSticker) -> Result<Self, Self::Error> {
        let InputSticker {
            sticker,
            emoji_list,
            mask_position,
            keywords,
        } = value;
        let mut form = Form::new();
        let sticker = match sticker.kind {
            InputFileKind::Id(x) | InputFileKind::Url(x) => x,
            _ => {
                let name = "tgbot_input_sticker";
                form.insert_field(name, sticker);
                format!("attach://{}", name)
            }
        };
        form.insert_field(
            "sticker",
            serde_json::to_string(&InputStickerMetadata {
                sticker,
                emoji_list,
                mask_position,
                keywords,
            })
            .map_err(InputStickerError::Serialize)?,
        );
        Ok(form)
    }
}

/// A collection of stickers to be added to a sticker set
#[derive(Debug, Default)]
pub struct InputStickers {
    form: Form,
    metadata: Vec<InputStickerMetadata>,
}

impl InputStickers {
    /// Add a sticker to the collection
    pub fn with(mut self, value: InputSticker) -> Self {
        let InputSticker {
            sticker,
            emoji_list,
            mask_position,
            keywords,
        } = value;
        let sticker = match sticker.kind {
            InputFileKind::Id(x) | InputFileKind::Url(x) => x,
            _ => {
                let idx = self.metadata.len() + 1;
                let name = format!("tgbot_input_sticker_{}", idx);
                self.form.insert_field(&name, sticker);
                format!("attach://{}", name)
            }
        };
        self.metadata.push(InputStickerMetadata {
            sticker,
            emoji_list,
            mask_position,
            keywords,
        });
        self
    }
}

impl TryFrom<InputStickers> for Form {
    type Error = InputStickerError;

    fn try_from(mut value: InputStickers) -> Result<Self, Self::Error> {
        let stickers = serde_json::to_string(&value.metadata).map_err(InputStickerError::Serialize)?;
        value.form.insert_field("stickers", stickers);
        Ok(value.form)
    }
}

/// An InputStickers error
#[derive(Debug)]
pub enum InputStickerError {
    /// Could not serialize a list of input stickers
    Serialize(JsonError),
}

impl fmt::Display for InputStickerError {
    fn fmt(&self, out: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serialize(err) => write!(out, "could not serialize a list of input stickers: {}", err),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
struct InputStickerMetadata {
    sticker: String,
    emoji_list: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    mask_position: Option<MaskPosition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    keywords: Option<Vec<String>>,
}
