use serde::{Deserialize, Serialize};

use crate::{
    api::{Form, WriteForm},
    types::{
        InputMedia,
        InputMediaAnimation,
        InputMediaAudio,
        InputMediaData,
        InputMediaPhoto,
        InputMediaVideo,
        InputMediaVoiceNote,
        Integer,
        Location,
        RichBlockCaption,
        RichBlockListItemType,
        RichBlockTableCell,
        RichText,
    },
};

/// Represents a block in a rich formatted message to be sent.
#[derive(Debug)]
pub struct InputRichBlock {
    data: InputRichBlockData,
    blocks: Option<Vec<InputRichBlock>>,
    block_list_items: Option<Vec<InputRichBlockListItem>>,
    input_media: Option<InputMedia>,
}

impl InputRichBlock {
    fn new(data: InputRichBlockData) -> Self {
        Self {
            data,
            blocks: None,
            block_list_items: None,
            input_media: None,
        }
    }

    fn from_blocks<T>(data_type: InputRichBlockDataType, blocks: T) -> Self
    where
        T: IntoIterator<Item = InputRichBlock>,
    {
        Self::new(InputRichBlockData {
            data_type,
            parameters: InputRichBlockParameters { ..Default::default() },
        })
        .with_blocks(blocks)
    }

    fn with_blocks<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = InputRichBlock>,
    {
        self.blocks = Some(value.into_iter().collect());
        self
    }

    fn with_block_list_items<T>(mut self, value: T) -> Self
    where
        T: IntoIterator<Item = InputRichBlockListItem>,
    {
        self.block_list_items = Some(value.into_iter().collect());
        self
    }

    fn with_input_media<T>(mut self, value: T) -> Self
    where
        T: Into<InputMedia>,
    {
        self.input_media = Some(value.into());
        self
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the anchor.
    ///
    /// A block with an anchor, corresponding to the HTML tag `<a>` with the attribute name.
    pub fn anchor<T>(name: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(InputRichBlockData {
            data_type: InputRichBlockDataType::Anchor,
            parameters: InputRichBlockParameters {
                name: Some(name.into()),
                ..Default::default()
            },
        })
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `media` - The file.
    /// * `info` - Additional information for the file; caption is ignored.
    ///
    /// A block with an animation, corresponding to the HTML tag `<video>`.
    pub fn animation<T>(value: T) -> Self
    where
        T: Into<InputMediaAnimation>,
    {
        Self::new(InputRichBlockData {
            data_type: InputRichBlockDataType::Animation,
            parameters: InputRichBlockParameters { ..Default::default() },
        })
        .with_input_media(value.into())
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `media` - The file.
    /// * `info` - Additional information for the file; caption is ignored.
    ///
    /// A block with a music file, corresponding to the HTML tag `<audio>`.
    pub fn audio<T>(value: T) -> Self
    where
        T: Into<InputMediaAudio>,
    {
        Self::new(InputRichBlockData {
            data_type: InputRichBlockDataType::Audio,
            parameters: InputRichBlockParameters { ..Default::default() },
        })
        .with_input_media(value.into())
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `blocks` - Content of the block.
    /// * `credit` - Credit of the block.
    ///
    /// A block quotation, corresponding to the HTML tag `<blockquote>`.
    pub fn blockquote<A, B>(blocks: A, credit: Option<B>) -> Self
    where
        A: IntoIterator<Item = InputRichBlock>,
        B: Into<RichText>,
    {
        let mut result = Self::from_blocks(InputRichBlockDataType::Blockquote, blocks);
        result.data.parameters.credit = credit.map(Into::into);
        result
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `blocks` - Elements of the collage.
    /// * `caption` - Caption of the block.
    ///
    /// A collage, corresponding to the custom HTML tag `<tg-collage>`.
    pub fn collage<A, B>(blocks: A, caption: Option<B>) -> Self
    where
        A: IntoIterator<Item = InputRichBlock>,
        B: Into<RichBlockCaption>,
    {
        let mut result = Self::from_blocks(InputRichBlockDataType::Collage, blocks);
        result.data.parameters.caption = caption.map(|x| x.into().into());
        result
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `summary` - Always shown summary of the block.
    /// * `blocks` - Content of the block.
    /// * `is_open` - Whether the content of the block is visible by default.
    ///
    /// An expandable block for details disclosure,
    /// corresponding to the HTML tag `<details>`.
    pub fn details<A, B>(summary: A, blocks: B, is_open: bool) -> Self
    where
        A: Into<RichText>,
        B: IntoIterator<Item = InputRichBlock>,
    {
        let mut result = Self::from_blocks(InputRichBlockDataType::Details, blocks);
        result.data.parameters.summary = Some(summary.into());
        if is_open {
            result.data.parameters.is_open = Some(true);
        }
        result
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// A divider, corresponding to the HTML tag `<hr/>`.
    pub fn divider() -> Self {
        Self::new(InputRichBlockData {
            data_type: InputRichBlockDataType::Divider,
            parameters: InputRichBlockParameters { ..Default::default() },
        })
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `text` - Text of the block.
    ///
    /// A footer, corresponding to the HTML tag `<footer>`.
    pub fn footer<T>(text: T) -> Self
    where
        T: Into<RichText>,
    {
        Self::new(InputRichBlockData {
            data_type: InputRichBlockDataType::Footer,
            parameters: InputRichBlockParameters {
                text: Some(text.into()),
                ..Default::default()
            },
        })
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `text` - Text of the block.
    /// * `size` - Relative size of the text font; 1-6.
    ///
    /// A section heading, corresponding to the HTML tags `<h%N%>`.
    pub fn heading<T>(text: T, size: Integer) -> Self
    where
        T: Into<RichText>,
    {
        Self::new(InputRichBlockData {
            data_type: InputRichBlockDataType::Heading,
            parameters: InputRichBlockParameters {
                text: Some(text.into()),
                size: Some(size),
                ..Default::default()
            },
        })
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `items` - Items of the list.
    ///
    /// A list of blocks, corresponding to the HTML tag
    /// `<ul>` or `<ol>` with multiple nested tags `<li>`.
    pub fn list<T>(items: T) -> Self
    where
        T: IntoIterator<Item = InputRichBlockListItem>,
    {
        Self::new(InputRichBlockData {
            data_type: InputRichBlockDataType::List,
            parameters: InputRichBlockParameters { ..Default::default() },
        })
        .with_block_list_items(items)
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `location` - Location of the center of the map.
    /// * `zoom` - Map zoom level; 0-24.
    /// * `width` - Map width; 0-10000.
    /// * `height` - Map height; 0-10000.
    /// * `caption` - Caption of the block.
    ///
    /// A block with a map,
    /// corresponding to the custom HTML tag `<tg-map>`.
    /// The map's width and height must not exceed 10000 in total.
    /// The width and height ratio must be at most 20.
    pub fn map<T>(location: Location, zoom: Integer, width: Integer, height: Integer, caption: Option<T>) -> Self
    where
        T: Into<RichBlockCaption>,
    {
        Self::new(InputRichBlockData {
            data_type: InputRichBlockDataType::Map,
            parameters: InputRichBlockParameters {
                caption: caption.map(|x| x.into().into()),
                location: Some(location),
                height: Some(height),
                width: Some(width),
                zoom: Some(zoom),
                ..Default::default()
            },
        })
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `expression` - The mathematical expression in LaTeX format.
    ///
    /// A block with a mathematical expression in LaTeX format,
    /// corresponding to the custom HTML tag `<tg-math-block>`.
    pub fn mathematical_expression<T>(expression: T) -> Self
    where
        T: Into<String>,
    {
        Self::new(InputRichBlockData {
            data_type: InputRichBlockDataType::MathematicalExpression,
            parameters: InputRichBlockParameters {
                expression: Some(expression.into()),
                ..Default::default()
            },
        })
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `text` - Text of the block.
    ///
    /// A text paragraph, corresponding to the HTML tag `<p>`.
    pub fn paragraph<T>(text: T) -> Self
    where
        T: Into<RichText>,
    {
        Self::new(InputRichBlockData {
            data_type: InputRichBlockDataType::Paragraph,
            parameters: InputRichBlockParameters {
                text: Some(text.into()),
                ..Default::default()
            },
        })
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The file.
    ///
    /// A block with a photo, corresponding to the HTML tag `<img>`.
    pub fn photo<T>(value: T) -> Self
    where
        T: Into<InputMediaPhoto>,
    {
        Self::new(InputRichBlockData {
            data_type: InputRichBlockDataType::Photo,
            parameters: InputRichBlockParameters { ..Default::default() },
        })
        .with_input_media(value.into())
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `text` - Text of the block.
    /// * `language` - The programming language of the text.
    ///
    /// A preformatted text block,
    /// corresponding to the nested HTML tags `<pre>` and `<code>`.
    pub fn pre<A, B>(text: A, language: Option<B>) -> Self
    where
        A: Into<RichText>,
        B: Into<String>,
    {
        Self::new(InputRichBlockData {
            data_type: InputRichBlockDataType::Pre,
            parameters: InputRichBlockParameters {
                text: Some(text.into()),
                language: language.map(Into::into),
                ..Default::default()
            },
        })
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `text` - Text of the block.
    /// * `credit` - Credit of the block.
    ///
    /// A quotation with centered text, loosely corresponding to the HTML tag `<aside>`.
    pub fn pullquote<A, B>(text: A, credit: Option<B>) -> Self
    where
        A: Into<RichText>,
        B: Into<RichText>,
    {
        Self::new(InputRichBlockData {
            data_type: InputRichBlockDataType::Pullquote,
            parameters: InputRichBlockParameters {
                text: Some(text.into()),
                credit: credit.map(Into::into),
                ..Default::default()
            },
        })
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `blocks` - Elements of the slideshow.
    /// * `caption` - Caption of the block.
    ///
    /// A slideshow, corresponding to the custom HTML tag `<tg-slideshow>`.
    pub fn slideshow<A, B>(blocks: A, caption: Option<B>) -> Self
    where
        A: IntoIterator<Item = InputRichBlock>,
        B: Into<RichBlockCaption>,
    {
        let mut result = Self::from_blocks(InputRichBlockDataType::Slideshow, blocks);
        result.data.parameters.caption = caption.map(|x| x.into().into());
        result
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - Content of the table.
    ///
    /// A table, corresponding to the HTML tag `<table>`.
    pub fn table<T>(table: T) -> Self
    where
        T: Into<InputRichBlockTable>,
    {
        Self::new(InputRichBlockData {
            data_type: InputRichBlockDataType::Table,
            parameters: table.into().parameters,
        })
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `text` - Text of the block.
    ///
    /// A block with a "Thinking..." placeholder,
    /// corresponding to the custom HTML tag `<tg-thinking>`.
    ///
    /// The block may be used only in [`crate::types::SendRichMessageDraft`],
    /// therefore it can't be received in messages.
    ///
    /// See <https://t.me/addemoji/AIActions> for examples
    /// of custom emoji that are recommended for usage in the block.
    pub fn thinking<T>(text: T) -> Self
    where
        T: Into<RichText>,
    {
        Self::new(InputRichBlockData {
            data_type: InputRichBlockDataType::Thinking,
            parameters: InputRichBlockParameters {
                text: Some(text.into()),
                ..Default::default()
            },
        })
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The file.
    pub fn video<T>(value: T) -> Self
    where
        T: Into<InputMediaVideo>,
    {
        Self::new(InputRichBlockData {
            data_type: InputRichBlockDataType::Video,
            parameters: InputRichBlockParameters { ..Default::default() },
        })
        .with_input_media(value.into())
    }

    /// Creates a new `InputRichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The file.
    pub fn voice_note<T>(value: T) -> Self
    where
        T: Into<InputMediaVoiceNote>,
    {
        Self::new(InputRichBlockData {
            data_type: InputRichBlockDataType::VoiceNote,
            parameters: InputRichBlockParameters { ..Default::default() },
        })
        .with_input_media(value.into())
    }
}

impl WriteForm for InputRichBlock {
    type Output = InputRichBlockData;

    fn write(self, form: &mut Form) -> Self::Output {
        let Self {
            blocks,
            block_list_items,
            mut data,
            input_media,
        } = self;
        if let Some(input_media) = input_media {
            let media_data = input_media.write(form);
            match data.data_type {
                InputRichBlockDataType::Animation => {
                    data.parameters.animation = Some(media_data);
                }
                InputRichBlockDataType::Audio => {
                    data.parameters.audio = Some(media_data);
                }
                InputRichBlockDataType::Photo => {
                    data.parameters.photo = Some(media_data);
                }
                InputRichBlockDataType::Video => {
                    data.parameters.video = Some(media_data);
                }
                InputRichBlockDataType::VoiceNote => {
                    data.parameters.voice_note = Some(media_data);
                }
                _ => { /* noop */ }
            };
        }
        data.parameters.blocks = blocks.map(|items| items.into_iter().map(|x| x.write(form)).collect());
        data.parameters.items = block_list_items.map(|items| items.into_iter().map(|x| x.write(form)).collect());
        data
    }
}

/// An item of a list to be sent.
#[derive(Debug)]
pub struct InputRichBlockListItem {
    blocks: Vec<InputRichBlock>,
    data: InputRichBlockListItemData,
}

impl FromIterator<InputRichBlock> for InputRichBlockListItem {
    fn from_iter<T: IntoIterator<Item = InputRichBlock>>(value: T) -> Self {
        Self {
            blocks: value.into_iter().collect(),
            data: InputRichBlockListItemData { ..Default::default() },
        }
    }
}

impl InputRichBlockListItem {
    /// Sets a new value for the `has_checkbox` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the item has a checkbox.
    pub fn with_has_checkbox(mut self, value: bool) -> Self {
        self.data.has_checkbox = Some(value);
        self
    }

    /// Sets a new value for the `is_checked` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the item has a checked checkbox.
    pub fn with_is_checked(mut self, value: bool) -> Self {
        self.data.is_checked = Some(value);
        self
    }

    /// Sets a new type.
    ///
    /// # Arguments
    ///
    /// * `value` - For ordered lists, the type of the item label.
    pub fn with_type(mut self, value: RichBlockListItemType) -> Self {
        self.data.item_type = Some(value);
        self
    }

    /// Sets a new value
    ///
    /// # Arguments
    ///
    /// * `value` - For ordered lists, the numeric value of the item label.
    pub fn with_value(mut self, value: Integer) -> Self {
        self.data.value = Some(value);
        self
    }
}

impl WriteForm for InputRichBlockListItem {
    type Output = InputRichBlockListItemData;

    fn write(self, form: &mut Form) -> Self::Output {
        let Self { blocks, mut data } = self;
        data.blocks = blocks.into_iter().map(|x| x.write(form)).collect();
        data
    }
}

/// A table, corresponding to the HTML tag `<table>`.
#[derive(Debug)]
pub struct InputRichBlockTable {
    parameters: InputRichBlockParameters,
}

impl<A, B> FromIterator<A> for InputRichBlockTable
where
    A: IntoIterator<Item = B>,
    B: Into<RichBlockTableCell>,
{
    fn from_iter<T: IntoIterator<Item = A>>(value: T) -> Self {
        let cells = value
            .into_iter()
            .map(|x| x.into_iter().map(Into::into).collect())
            .collect();
        Self {
            parameters: InputRichBlockParameters {
                cells: Some(cells),
                ..Default::default()
            },
        }
    }
}

impl<A, B, C> From<A> for InputRichBlockTable
where
    A: IntoIterator<Item = B>,
    B: IntoIterator<Item = C>,
    C: Into<RichBlockTableCell>,
{
    fn from(value: A) -> Self {
        Self::from_iter(value)
    }
}

impl InputRichBlockTable {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption of the table.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<RichText>,
    {
        self.parameters.caption = Some(value.into().into());
        self
    }

    /// Sets a new value for the `is_bordered` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the table has borders.
    pub fn with_is_bordered(mut self, value: bool) -> Self {
        self.parameters.is_bordered = Some(value);
        self
    }

    /// Sets a new value for the `is_striped` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the table has stripes.
    pub fn with_is_striped(mut self, value: bool) -> Self {
        self.parameters.is_striped = Some(value);
        self
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
pub(crate) struct InputRichBlockListItemData {
    blocks: Vec<InputRichBlockData>,
    has_checkbox: Option<bool>,
    is_checked: Option<bool>,
    #[serde(rename = "type")]
    item_type: Option<RichBlockListItemType>,
    value: Option<Integer>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct InputRichBlockData {
    #[serde(rename = "type")]
    data_type: InputRichBlockDataType,
    #[serde(flatten)]
    parameters: InputRichBlockParameters,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Deserialize, Serialize)]
struct InputRichBlockParameters {
    animation: Option<InputMediaData>,
    audio: Option<InputMediaData>,
    blocks: Option<Vec<InputRichBlockData>>,
    caption: Option<InputRichBlockCaption>,
    cells: Option<Vec<Vec<RichBlockTableCell>>>,
    credit: Option<RichText>,
    expression: Option<String>,
    height: Option<Integer>,
    is_bordered: Option<bool>,
    is_open: Option<bool>,
    is_striped: Option<bool>,
    items: Option<Vec<InputRichBlockListItemData>>,
    language: Option<String>,
    location: Option<Location>,
    name: Option<String>,
    photo: Option<InputMediaData>,
    size: Option<Integer>,
    summary: Option<RichText>,
    text: Option<RichText>,
    video: Option<InputMediaData>,
    voice_note: Option<InputMediaData>,
    width: Option<Integer>,
    zoom: Option<Integer>,
}

#[derive(Debug, derive_more::From, Deserialize, Serialize)]
#[serde(untagged)]
enum InputRichBlockCaption {
    Caption(RichBlockCaption),
    Text(RichText),
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum InputRichBlockDataType {
    Anchor,
    Animation,
    Audio,
    Blockquote,
    Collage,
    Details,
    Divider,
    Footer,
    Heading,
    List,
    Map,
    MathematicalExpression,
    Paragraph,
    Photo,
    Pre,
    Pullquote,
    Slideshow,
    Table,
    Thinking,
    Video,
    VoiceNote,
}
