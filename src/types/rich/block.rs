use serde::{Deserialize, Serialize};

use super::text::RichText;
use crate::types::{Animation, Audio, Integer, Location, PhotoSize, Video, Voice};

/// Represents a block in a rich formatted message.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(from = "RawRichBlock", into = "RawRichBlock")]
pub enum RichBlock {
    /// An anchor (`<a>` with the `name` attribute).
    Anchor(String),
    /// An animation (`<video>`).
    Animation(RichBlockAnimation),
    /// An audio (`<audio>`).
    Audio(RichBlockAudio),
    /// A block quotation (`<blockquote>`).
    BlockQuotation(RichBlockBlockQuotation),
    /// A collage (`<tg-collage>`).
    Collage(RichBlockCollage),
    /// An expandable block for details disclosure (`<details>`).
    Details(RichBlockDetails),
    /// A divider (`<hr />`).
    Divider,
    /// A footer (`<footer>`).
    Footer(RichText),
    /// A list (`<ul>` or `<ol>`).
    List(Vec<RichBlockListItem>),
    /// A map (`<tg-map>`).
    Map(RichBlockMap),
    /// A mathematical expression (`<tg-math-block>`).
    MathematicalExpression(String),
    /// A paragraph (`<p>`).
    Paragraph(RichText),
    /// A Photo (`<photo>`).
    Photo(RichBlockPhoto),
    /// A preformatted text (`<pre>` or `<code>`).
    Preformatted(RichBlockPreformatted),
    /// A quotation with centered text (`<aside>`).
    PullQuotation(RichBlockPullQuotation),
    /// A section heading (`<h[1-6]>` text, size).
    SectionHeading(RichText, Integer),
    /// A slideshow (`<tg-slideshow>`).
    Slideshow(RichBlockSlideshow),
    /// A table (`<table>`).
    Table(RichBlockTable),
    /// A "Thinking ..." placeholder (`<tg-thinking>`).
    ///
    /// The block may be used only in `[crate::types::SendRichMessageDraft]`.
    Thinking(RichText),
    /// A video (`<video>`).
    Video(RichBlockVideo),
    /// A voice note (`<audio>`).
    VoiceNote(RichBlockVoiceNote),
}

/// A block with an animation (`<video>`).
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RichBlockAnimation {
    /// The animation.
    pub animation: Animation,
    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
    /// Whether the media preview is covered by a spoiler animation.
    pub has_spoiler: Option<bool>,
}

/// A block with a music file (`<audio>`)
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RichBlockAudio {
    /// The audio.
    pub audio: Audio,
    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// A block quotation (`<blockquote>`).
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RichBlockBlockQuotation {
    /// Content of the block.
    pub blocks: Vec<RichBlock>,
    /// Credit of the block.
    pub credit: Option<RichText>,
}

/// Caption of a rich formatted block.
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RichBlockCaption {
    /// Block caption.
    pub text: RichText,
    /// Block credit which corresponds to the HTML tag `<cite>`.
    pub credit: Option<RichText>,
}

impl<T> From<T> for RichBlockCaption
where
    T: Into<RichText>,
{
    fn from(value: T) -> Self {
        Self {
            text: value.into(),
            credit: None,
        }
    }
}

impl<A, B> From<(A, B)> for RichBlockCaption
where
    A: Into<RichText>,
    B: Into<RichText>,
{
    fn from((text, credit): (A, B)) -> Self {
        Self::from(text).with_credit(credit)
    }
}

impl RichBlockCaption {
    /// Sets a new credit.
    ///
    /// # Arguments
    ///
    /// * `value` - Block credit.
    pub fn with_credit<T>(mut self, value: T) -> Self
    where
        T: Into<RichText>,
    {
        self.credit = Some(value.into());
        self
    }
}

/// A collage (`<tg-collage>`).
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RichBlockCollage {
    /// Elements of the collage.
    pub blocks: Vec<RichBlock>,
    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// An expandable block for details disclosure (`<details>`).
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RichBlockDetails {
    /// Content of the block.
    pub blocks: Vec<RichBlock>,
    /// Always shown summary of the block.
    pub summary: RichText,
    /// Whether the content of the block is visible by default.
    pub is_open: Option<bool>,
}

/// An item of a list.
#[serde_with::skip_serializing_none]
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RichBlockListItem {
    /// The content of the item.
    pub blocks: Vec<RichBlock>,
    /// Label of the item.
    pub label: String,
    /// Whether the item has a checkbox.
    pub has_checkbox: Option<bool>,
    /// Whether the item has a checked checkbox.
    pub is_checked: Option<bool>,
    /// For ordered lists, the type of the item label.
    #[serde(rename = "type")]
    pub item_type: Option<RichBlockListItemType>,
    /// For ordered lists, the numberic value of the item label.
    pub value: Option<Integer>,
}

/// Represents the type of the item label.
#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum RichBlockListItemType {
    /// Decimal numbers.
    #[serde(rename = "1")]
    Decimal,
    /// Lowercase letters.
    #[serde(rename = "a")]
    LowercaseLetters,
    /// Lowercase roman numerals.
    #[serde(rename = "i")]
    LowercaseRoman,
    /// Uppercase letters.
    #[serde(rename = "A")]
    UppercaseLetters,
    /// Uppercase roman numerals.
    #[serde(rename = "I")]
    UppercaseRoman,
}

/// Ablock with a map (`<tg-map>`).
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RichBlockMap {
    /// Location of the center of the map.
    pub location: Location,
    /// Expected height of the map.
    pub height: Integer,
    /// Expected width of the map.
    pub width: Integer,
    /// Map zoom level; 13-20.
    pub zoom: Integer,
    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// A block with a photo (`<photo>`).
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RichBlockPhoto {
    /// Available sizes of the photo.
    pub photo: Vec<PhotoSize>,
    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
    /// Whether the media preview is covered by a spoiler animation.
    pub has_spoiler: Option<bool>,
}

/// A preformatted text block (`<pre>` or `<code>`).
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RichBlockPreformatted {
    /// Text of the block.
    pub text: RichText,
    /// The programming language of the text.
    pub language: Option<String>,
}

/// A quotation with centered text (`<aside>`).
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RichBlockPullQuotation {
    /// Text of the block.
    pub text: RichText,
    /// Credit of the block.
    pub credit: Option<RichText>,
}

/// A slideshow (`<tg-slideshow>`).
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RichBlockSlideshow {
    /// Elements of the slideshow.
    pub blocks: Vec<RichBlock>,
    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

/// A table (`<table>`).
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RichBlockTable {
    /// Cells of the table.
    pub cells: Vec<Vec<RichBlockTableCell>>,
    /// Caption of the table.
    pub caption: Option<RichText>,
    /// Whether the table has borders.
    pub is_bordered: Option<bool>,
    /// Whether the table is striped.
    pub is_striped: Option<bool>,
}

/// Cell in a table.
#[serde_with::skip_serializing_none]
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct RichBlockTableCell {
    /// Horizontal cell content alignment.
    pub align: RichBlockTableCellAlign,
    /// Vertical cell content alignment.
    pub valign: RichBlockTableCellValign,
    /// The number of columns the cell spans if it is bigger than 1.
    pub colspan: Option<Integer>,
    /// Whether the cell is a header cell.
    pub is_header: Option<bool>,
    /// The number of rows the cell spans if it is bigger than 1.
    pub rowspan: Option<Integer>,
    /// Text in the cell.
    ///
    /// If omitted, the the cell is invisible.
    pub text: Option<RichText>,
}

impl<T> From<T> for RichBlockTableCell
where
    T: Into<RichText>,
{
    fn from(value: T) -> Self {
        Self::default().with_text(value)
    }
}

impl RichBlockTableCell {
    /// Sets a new horizontal alignment.
    ///
    /// # Arguments
    ///
    /// * `value` - The horizontal alignment.
    pub fn with_align(mut self, value: RichBlockTableCellAlign) -> Self {
        self.align = value;
        self
    }

    /// Sets a new colspan.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of columns the cell span.
    pub fn with_colspan(mut self, value: Integer) -> Self {
        self.colspan = Some(value);
        self
    }

    /// Sets a new value for the `is_header` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the cell is a header cell.
    pub fn with_is_header(mut self, value: bool) -> Self {
        self.is_header = Some(value);
        self
    }

    /// Sets a new rowspan.
    ///
    /// # Arguments
    ///
    /// * `value` - The number of rows the cell spans.
    pub fn with_rowspan(mut self, value: Integer) -> Self {
        self.rowspan = Some(value);
        self
    }

    /// Sets a new text.
    ///
    /// # Arguments
    ///
    /// * `value` - The text.
    pub fn with_text<T>(mut self, value: T) -> Self
    where
        T: Into<RichText>,
    {
        self.text = Some(value.into());
        self
    }

    /// Sets a new vertical alignment.
    ///
    /// # Arguments
    ///
    /// * `value` - The vertical alignment.
    pub fn with_valign(mut self, value: RichBlockTableCellValign) -> Self {
        self.valign = value;
        self
    }
}

/// Horizontall cell content alignment.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RichBlockTableCellAlign {
    /// Left.
    #[default]
    Left,
    /// Center.
    Center,
    /// Right.
    Right,
}

/// Vertical cell content alignment.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RichBlockTableCellValign {
    /// Top.
    Top,
    /// Middle.
    #[default]
    Middle,
    /// Bottom.
    Bottom,
}

/// A block with a video (`<video`).
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RichBlockVideo {
    /// The video.
    pub video: Video,
    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
    /// Whether the media preview is covered by a spoiler animation.
    pub has_spoiler: Option<bool>,
}

/// A block with a voice note (`<audio>`).
#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RichBlockVoiceNote {
    /// The voice note.
    pub voice_note: Voice,
    /// Caption of the block.
    pub caption: Option<RichBlockCaption>,
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum RawRichBlock {
    Anchor {
        name: String,
    },
    Animation(RichBlockAnimation),
    Audio(RichBlockAudio),
    #[serde(rename = "blockquote")]
    BlockQuotation(RichBlockBlockQuotation),
    Collage(RichBlockCollage),
    Details(RichBlockDetails),
    Divider,
    Footer {
        text: RichText,
    },
    List {
        items: Vec<RichBlockListItem>,
    },
    Map(RichBlockMap),
    MathematicalExpression {
        expression: String,
    },
    Paragraph {
        text: RichText,
    },
    Photo(RichBlockPhoto),
    #[serde(rename = "pre")]
    Preformatted(RichBlockPreformatted),
    #[serde(rename = "pullquote")]
    PullQuotation(RichBlockPullQuotation),
    #[serde(rename = "heading")]
    SectionHeading {
        text: RichText,
        size: Integer,
    },
    Slideshow(RichBlockSlideshow),
    Table(RichBlockTable),
    Thinking {
        text: RichText,
    },
    Video(RichBlockVideo),
    VoiceNote(RichBlockVoiceNote),
}

impl From<RichBlock> for RawRichBlock {
    fn from(value: RichBlock) -> Self {
        match value {
            RichBlock::Anchor(name) => Self::Anchor { name },
            RichBlock::Animation(value) => Self::Animation(value),
            RichBlock::Audio(value) => Self::Audio(value),
            RichBlock::BlockQuotation(value) => Self::BlockQuotation(value),
            RichBlock::Collage(value) => Self::Collage(value),
            RichBlock::Details(value) => Self::Details(value),
            RichBlock::Divider => Self::Divider,
            RichBlock::Footer(text) => Self::Footer { text },
            RichBlock::List(items) => Self::List { items },
            RichBlock::Map(value) => Self::Map(value),
            RichBlock::MathematicalExpression(expression) => Self::MathematicalExpression { expression },
            RichBlock::Paragraph(text) => Self::Paragraph { text },
            RichBlock::Photo(value) => Self::Photo(value),
            RichBlock::Preformatted(value) => Self::Preformatted(value),
            RichBlock::PullQuotation(value) => Self::PullQuotation(value),
            RichBlock::SectionHeading(text, size) => Self::SectionHeading { text, size },
            RichBlock::Slideshow(value) => Self::Slideshow(value),
            RichBlock::Table(value) => Self::Table(value),
            RichBlock::Thinking(text) => Self::Thinking { text },
            RichBlock::Video(value) => Self::Video(value),
            RichBlock::VoiceNote(value) => Self::VoiceNote(value),
        }
    }
}

impl From<RawRichBlock> for RichBlock {
    fn from(value: RawRichBlock) -> Self {
        match value {
            RawRichBlock::Anchor { name } => Self::Anchor(name),
            RawRichBlock::Animation(value) => Self::Animation(value),
            RawRichBlock::Audio(value) => Self::Audio(value),
            RawRichBlock::BlockQuotation(value) => Self::BlockQuotation(value),
            RawRichBlock::Collage(value) => Self::Collage(value),
            RawRichBlock::Details(value) => Self::Details(value),
            RawRichBlock::Divider => Self::Divider,
            RawRichBlock::Footer { text } => Self::Footer(text),
            RawRichBlock::List { items } => Self::List(items),
            RawRichBlock::Map(value) => Self::Map(value),
            RawRichBlock::MathematicalExpression { expression } => Self::MathematicalExpression(expression),
            RawRichBlock::Paragraph { text } => Self::Paragraph(text),
            RawRichBlock::Photo(value) => Self::Photo(value),
            RawRichBlock::Preformatted(value) => Self::Preformatted(value),
            RawRichBlock::PullQuotation(value) => Self::PullQuotation(value),
            RawRichBlock::SectionHeading { text, size } => Self::SectionHeading(text, size),
            RawRichBlock::Slideshow(value) => Self::Slideshow(value),
            RawRichBlock::Table(value) => Self::Table(value),
            RawRichBlock::Thinking { text } => Self::Thinking(text),
            RawRichBlock::Video(value) => Self::Video(value),
            RawRichBlock::VoiceNote(value) => Self::VoiceNote(value),
        }
    }
}
