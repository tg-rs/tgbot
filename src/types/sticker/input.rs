use serde::Serialize;

use crate::{
    api::{Form, WriteForm},
    types::{InputFile, MaskPosition, StickerFormat},
};

/// Represents a metadata of a sticker to be added to a sticker set.
#[derive(Debug)]
pub struct InputSticker {
    data: InputStickerData,
    sticker: InputFile,
}

impl InputSticker {
    /// Creates a new `InputSticker`.
    ///
    /// # Arguments
    ///
    /// * `sticker` - The added sticker.
    /// * `emoji_list` - List of 1-20 emoji associated with the sticker.
    /// * `format` - Format of the sticker.
    pub fn new<A, B, C>(sticker: A, emoji_list: B, format: StickerFormat) -> Self
    where
        A: Into<InputFile>,
        B: IntoIterator<Item = C>,
        C: Into<String>,
    {
        Self {
            data: InputStickerData {
                emoji_list: Some(emoji_list.into_iter().map(Into::into).collect()),
                format: Some(format),
                ..Default::default()
            },
            sticker: sticker.into(),
        }
    }

    /// Sets a new list of keywords.
    ///
    /// # Arguments
    ///
    /// * `value` - List of 0-20 search keywords for the sticker
    ///   with total length of up to 64 characters;
    ///   for “regular” and “custom_emoji” stickers only.
    pub fn with_keywords<A, B>(mut self, value: A) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<String>,
    {
        self.data.keywords = Some(value.into_iter().map(Into::into).collect());
        self
    }

    /// Sets a new mask position.
    ///
    /// # Arguments
    ///
    /// * `value` - Position where the mask should be placed on faces;
    ///   for “mask” stickers only.
    pub fn with_mask_position(mut self, value: MaskPosition) -> Self {
        self.data.mask_position = Some(value);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub(crate) struct InputStickerData {
    emoji_list: Option<Vec<String>>,
    format: Option<StickerFormat>,
    keywords: Option<Vec<String>>,
    mask_position: Option<MaskPosition>,
    sticker: Option<String>,
}

impl WriteForm for InputSticker {
    type Output = InputStickerData;

    fn write(self, form: &mut Form) -> Self::Output {
        let Self { sticker, mut data } = self;
        data.sticker = Some(sticker.write(form));
        data
    }
}
