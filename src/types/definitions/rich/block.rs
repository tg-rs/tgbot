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

impl RichBlock {
    /// Creates an anchor `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The name of the anchor.
    pub fn anchor<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self::Anchor(value.into())
    }

    /// Creates an animation `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The animation.
    pub fn animation<T>(value: T) -> Self
    where
        T: Into<RichBlockAnimation>,
    {
        Self::Animation(value.into())
    }

    /// Creates an audio `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The audio.
    pub fn audio<T>(value: T) -> Self
    where
        T: Into<RichBlockAudio>,
    {
        Self::Audio(value.into())
    }

    /// Creates a block quotation `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The block quotation.
    pub fn block_quotation<T>(value: T) -> Self
    where
        T: Into<RichBlockBlockQuotation>,
    {
        Self::BlockQuotation(value.into())
    }

    /// Creates a collage `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The collage.
    pub fn collage<T>(value: T) -> Self
    where
        T: Into<RichBlockCollage>,
    {
        Self::Collage(value.into())
    }

    /// Creates a details `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The details.
    pub fn details<T>(value: T) -> Self
    where
        T: Into<RichBlockDetails>,
    {
        Self::Details(value.into())
    }

    /// Creates a footer `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The footer.
    pub fn footer<T>(value: T) -> Self
    where
        T: Into<RichText>,
    {
        Self::Footer(value.into())
    }

    /// Creates a list `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The list items.
    pub fn list<A, B>(value: A) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<RichBlockListItem>,
    {
        Self::List(value.into_iter().map(Into::into).collect())
    }

    /// Creates a map `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The map.
    pub fn map<T>(value: T) -> Self
    where
        T: Into<RichBlockMap>,
    {
        Self::Map(value.into())
    }

    /// Creates a mathematical expression `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The expression.
    pub fn mathematical_expression<T>(value: T) -> Self
    where
        T: Into<String>,
    {
        Self::MathematicalExpression(value.into())
    }

    /// Creates a paragraph `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The paragraph.
    pub fn paragraph<T>(value: T) -> Self
    where
        T: Into<RichText>,
    {
        Self::Paragraph(value.into())
    }

    /// Creates a photo `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The photo.
    pub fn photo<T>(value: T) -> Self
    where
        T: Into<RichBlockPhoto>,
    {
        Self::Photo(value.into())
    }

    /// Creates a preformatted `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The text.
    pub fn preformatted<T>(value: T) -> Self
    where
        T: Into<RichBlockPreformatted>,
    {
        Self::Preformatted(value.into())
    }

    /// Creates a pull quotation `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The quotation.
    pub fn pull_quotation<T>(value: T) -> Self
    where
        T: Into<RichBlockPullQuotation>,
    {
        Self::PullQuotation(value.into())
    }

    /// Creates a section heading `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `text` - The heading.
    /// * `size` - Size of the heading.
    pub fn section_heading<T>(text: T, size: Integer) -> Self
    where
        T: Into<RichText>,
    {
        Self::SectionHeading(text.into(), size)
    }

    /// Creates a slideshow `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - Thge slideshow.
    pub fn slideshow<T>(value: T) -> Self
    where
        T: Into<RichBlockSlideshow>,
    {
        Self::Slideshow(value.into())
    }

    /// Creates a table `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The table.
    pub fn table<T>(value: T) -> Self
    where
        T: Into<RichBlockTable>,
    {
        Self::Table(value.into())
    }

    /// Creates a thinking `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The content.
    pub fn thinking<T>(value: T) -> Self
    where
        T: Into<RichText>,
    {
        Self::Thinking(value.into())
    }

    /// Creates a video `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The video.
    pub fn video<T>(value: T) -> Self
    where
        T: Into<RichBlockVideo>,
    {
        Self::Video(value.into())
    }

    /// Creates a voice note `RichBlock`.
    ///
    /// # Arguments
    ///
    /// * `value` - The voice note.
    pub fn voice_note<T>(value: T) -> Self
    where
        T: Into<RichBlockVoiceNote>,
    {
        Self::VoiceNote(value.into())
    }
}

impl From<&str> for RichBlock {
    fn from(value: &str) -> Self {
        Self::paragraph(value)
    }
}

impl From<String> for RichBlock {
    fn from(value: String) -> Self {
        Self::paragraph(value)
    }
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

impl From<Animation> for RichBlockAnimation {
    fn from(value: Animation) -> Self {
        Self {
            animation: value,
            has_spoiler: None,
            caption: None,
        }
    }
}

impl<T> From<(Animation, T)> for RichBlockAnimation
where
    T: Into<RichBlockCaption>,
{
    fn from((animation, caption): (Animation, T)) -> Self {
        Self::from(animation).with_caption(caption)
    }
}

impl<T> From<(Animation, T, bool)> for RichBlockAnimation
where
    T: Into<RichBlockCaption>,
{
    fn from((animation, caption, has_spoiler): (Animation, T, bool)) -> Self {
        Self::from(animation)
            .with_caption(caption)
            .with_has_spoiler(has_spoiler)
    }
}

impl RichBlockAnimation {
    /// Sets a new caption
    ///
    /// # Arguments
    ///
    /// * `value` - Caption of the block.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<RichBlockCaption>,
    {
        self.caption = Some(value.into());
        self
    }

    /// Sets a new value for the `has_spoiler` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the media preview is covered by a spoiler animation.
    pub fn with_has_spoiler(mut self, value: bool) -> Self {
        self.has_spoiler = Some(value);
        self
    }
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

impl From<Audio> for RichBlockAudio {
    fn from(value: Audio) -> Self {
        Self {
            audio: value,
            caption: None,
        }
    }
}

impl<T> From<(Audio, T)> for RichBlockAudio
where
    T: Into<RichBlockCaption>,
{
    fn from((audio, caption): (Audio, T)) -> Self {
        Self::from(audio).with_caption(caption)
    }
}

impl RichBlockAudio {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption of the block.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<RichBlockCaption>,
    {
        self.caption = Some(value.into());
        self
    }
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

impl From<RichBlock> for RichBlockBlockQuotation {
    fn from(value: RichBlock) -> Self {
        Self {
            blocks: vec![value],
            credit: None,
        }
    }
}

impl<T> From<(RichBlock, T)> for RichBlockBlockQuotation
where
    T: Into<RichText>,
{
    fn from((block, credit): (RichBlock, T)) -> Self {
        Self {
            blocks: vec![block],
            credit: Some(credit.into()),
        }
    }
}

impl<I> FromIterator<I> for RichBlockBlockQuotation
where
    I: Into<RichBlock>,
{
    fn from_iter<T>(value: T) -> Self
    where
        T: IntoIterator<Item = I>,
    {
        Self {
            blocks: value.into_iter().map(Into::into).collect(),
            credit: None,
        }
    }
}

impl RichBlockBlockQuotation {
    /// Sets a new credit.
    ///
    /// # Arguments
    ///
    /// * `value` - Credit of the block.
    pub fn with_credit<T>(mut self, value: T) -> Self
    where
        T: Into<RichText>,
    {
        self.credit = Some(value.into());
        self
    }
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

impl<A, B> From<A> for RichBlockCollage
where
    A: IntoIterator<Item = B>,
    B: Into<RichBlock>,
{
    fn from(value: A) -> Self {
        value.into_iter().collect()
    }
}

impl<I> FromIterator<I> for RichBlockCollage
where
    I: Into<RichBlock>,
{
    fn from_iter<T>(value: T) -> Self
    where
        T: IntoIterator<Item = I>,
    {
        Self {
            blocks: value.into_iter().map(Into::into).collect(),
            caption: None,
        }
    }
}

impl RichBlockCollage {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption of the block.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<RichBlockCaption>,
    {
        self.caption = Some(value.into());
        self
    }
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

impl<A, B, C> From<(A, C)> for RichBlockDetails
where
    A: IntoIterator<Item = B>,
    B: Into<RichBlock>,
    C: Into<RichText>,
{
    fn from((blocks, summary): (A, C)) -> Self {
        Self::new(blocks, summary)
    }
}

impl<A, B, C> From<(A, C, bool)> for RichBlockDetails
where
    A: IntoIterator<Item = B>,
    B: Into<RichBlock>,
    C: Into<RichText>,
{
    fn from((blocks, summary, is_open): (A, C, bool)) -> Self {
        Self::new(blocks, summary).with_is_open(is_open)
    }
}

impl RichBlockDetails {
    /// Creates a new `RichBlockDetails`.
    ///
    /// # Arguments
    ///
    /// * `blocks` - Content of the block.
    /// * `summary` - Always shown summary of the block.
    pub fn new<A, B, C>(blocks: A, summary: C) -> Self
    where
        A: IntoIterator<Item = B>,
        B: Into<RichBlock>,
        C: Into<RichText>,
    {
        Self {
            blocks: blocks.into_iter().map(Into::into).collect(),
            summary: summary.into(),
            is_open: None,
        }
    }

    /// Sets a new value for the `is_open` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the content of the block is visible by default.
    pub fn with_is_open(mut self, value: bool) -> Self {
        self.is_open = Some(value);
        self
    }
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

impl<A, B, C> From<(A, B)> for RichBlockListItem
where
    A: Into<String>,
    B: IntoIterator<Item = C>,
    C: Into<RichBlock>,
{
    fn from((label, blocks): (A, B)) -> Self {
        Self::new(label, blocks)
    }
}

impl RichBlockListItem {
    /// Creates a new `RichBlockListItem`.
    ///
    /// # Arguments
    ///
    /// * `label` - Label of the item.
    /// * `blocks` - The content of the item.
    pub fn new<A, B, C>(label: A, blocks: B) -> Self
    where
        A: Into<String>,
        B: IntoIterator<Item = C>,
        C: Into<RichBlock>,
    {
        Self {
            blocks: blocks.into_iter().map(Into::into).collect(),
            label: label.into(),
            has_checkbox: None,
            is_checked: None,
            item_type: None,
            value: None,
        }
    }

    /// Sets a new value for the `has_checkbox` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the item has a checkbox.
    pub fn with_has_checkbox(mut self, value: bool) -> Self {
        self.has_checkbox = Some(value);
        self
    }

    /// Sets a new value for the `is_checked` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the item has a checked checkbox.
    pub fn with_is_checked(mut self, value: bool) -> Self {
        self.is_checked = Some(value);
        self
    }

    /// Sets a new item type.
    ///
    /// # Arguments
    ///
    /// * `value` - The type of the item label.
    pub fn with_item_type(mut self, value: RichBlockListItemType) -> Self {
        self.item_type = Some(value);
        self
    }

    /// Sets a new value.
    ///
    /// # Arguments
    ///
    /// * `value` - The numeric value of the item label.
    pub fn with_value(mut self, value: Integer) -> Self {
        self.value = Some(value);
        self
    }
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

impl From<Location> for RichBlockMap {
    fn from(value: Location) -> Self {
        Self {
            location: value,
            zoom: 13,
            width: 200,
            height: 200,
            caption: None,
        }
    }
}

impl RichBlockMap {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption of the block.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<RichBlockCaption>,
    {
        self.caption = Some(value.into());
        self
    }

    /// Sets a new expected height of the map.
    ///
    /// # Arguments
    ///
    /// * `value` - The expected height.
    pub fn with_height(mut self, value: Integer) -> Self {
        self.height = value;
        self
    }

    /// Sets a new expected width of the map.
    ///
    /// # Arguments
    ///
    /// * `value` - The expected width.
    pub fn with_width(mut self, value: Integer) -> Self {
        self.width = value;
        self
    }

    /// Sets a new zoom level.
    ///
    /// # Arguments
    ///
    /// * `value` - Map zoom level; 13-20.
    pub fn with_zoom(mut self, value: Integer) -> Self {
        self.zoom = value;
        self
    }
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

impl<T> From<T> for RichBlockPhoto
where
    T: IntoIterator<Item = PhotoSize>,
{
    fn from(value: T) -> Self {
        Self::from_iter(value)
    }
}

impl FromIterator<PhotoSize> for RichBlockPhoto {
    fn from_iter<T>(value: T) -> Self
    where
        T: IntoIterator<Item = PhotoSize>,
    {
        Self {
            photo: value.into_iter().collect(),
            caption: None,
            has_spoiler: None,
        }
    }
}

impl RichBlockPhoto {
    /// Sets a new caption.
    ///
    ///  # Arguments
    ///
    /// * `value` - Caption of the block.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<RichBlockCaption>,
    {
        self.caption = Some(value.into());
        self
    }

    /// Sets a new value for the `has_spoiler` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the media preview is covered by a spoiler animation.
    pub fn with_has_spoiler(mut self, value: bool) -> Self {
        self.has_spoiler = Some(value);
        self
    }
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

impl<T> From<T> for RichBlockPreformatted
where
    T: Into<RichText>,
{
    fn from(value: T) -> Self {
        Self {
            text: value.into(),
            language: None,
        }
    }
}

impl<A, B> From<(A, B)> for RichBlockPreformatted
where
    A: Into<String>,
    B: Into<RichText>,
{
    fn from((language, text): (A, B)) -> Self {
        Self {
            text: text.into(),
            language: Some(language.into()),
        }
    }
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

impl<T> From<T> for RichBlockPullQuotation
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

impl<A, B> From<(A, B)> for RichBlockPullQuotation
where
    A: Into<RichText>,
    B: Into<RichText>,
{
    fn from((text, credit): (A, B)) -> Self {
        Self {
            text: text.into(),
            credit: Some(credit.into()),
        }
    }
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

impl<A, B> From<A> for RichBlockSlideshow
where
    A: IntoIterator<Item = B>,
    B: Into<RichBlock>,
{
    fn from(value: A) -> Self {
        value.into_iter().collect()
    }
}

impl<I> FromIterator<I> for RichBlockSlideshow
where
    I: Into<RichBlock>,
{
    fn from_iter<T>(value: T) -> Self
    where
        T: IntoIterator<Item = I>,
    {
        Self {
            blocks: value.into_iter().map(Into::into).collect(),
            caption: None,
        }
    }
}

impl RichBlockSlideshow {
    /// Sets a new caption
    ///
    /// # Arguments
    ///
    /// * `value` - Caption of the block.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<RichBlockCaption>,
    {
        self.caption = Some(value.into());
        self
    }
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

impl<A, B, C> From<A> for RichBlockTable
where
    A: IntoIterator<Item = B>,
    B: IntoIterator<Item = C>,
    C: Into<RichBlockTableCell>,
{
    fn from(value: A) -> Self {
        value.into_iter().collect()
    }
}

impl<A, B> FromIterator<A> for RichBlockTable
where
    A: IntoIterator<Item = B>,
    B: Into<RichBlockTableCell>,
{
    fn from_iter<T>(value: T) -> Self
    where
        T: IntoIterator<Item = A>,
    {
        Self {
            cells: value
                .into_iter()
                .map(|x| x.into_iter().map(Into::into).collect())
                .collect(),
            caption: None,
            is_bordered: None,
            is_striped: None,
        }
    }
}

impl RichBlockTable {
    /// Sets a new caption.
    ///
    /// # Arguments
    ///
    /// * `value` - Caption of the table.
    pub fn with_caption<T>(mut self, value: T) -> Self
    where
        T: Into<RichText>,
    {
        self.caption = Some(value.into());
        self
    }

    /// Sets a new value for the `is_bordered` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the table has borders.
    pub fn with_is_bordered(mut self, value: bool) -> Self {
        self.is_bordered = Some(value);
        self
    }

    /// Sets a new value for the `is_striped` flag.
    ///
    /// # Arguments
    ///
    /// * `value` - Whether the table is striped.
    pub fn with_is_striped(mut self, value: bool) -> Self {
        self.is_striped = Some(value);
        self
    }
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

impl From<Video> for RichBlockVideo {
    fn from(value: Video) -> Self {
        Self {
            video: value,
            caption: None,
            has_spoiler: None,
        }
    }
}

impl<T> From<(Video, T)> for RichBlockVideo
where
    T: Into<RichBlockCaption>,
{
    fn from((video, caption): (Video, T)) -> Self {
        Self {
            video,
            caption: Some(caption.into()),
            has_spoiler: None,
        }
    }
}

impl<T> From<(Video, T, bool)> for RichBlockVideo
where
    T: Into<RichBlockCaption>,
{
    fn from((video, caption, has_spoiler): (Video, T, bool)) -> Self {
        Self {
            video,
            caption: Some(caption.into()),
            has_spoiler: Some(has_spoiler),
        }
    }
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

impl From<Voice> for RichBlockVoiceNote {
    fn from(value: Voice) -> Self {
        Self {
            voice_note: value,
            caption: None,
        }
    }
}

impl<T> From<(Voice, T)> for RichBlockVoiceNote
where
    T: Into<RichBlockCaption>,
{
    fn from((voice_note, caption): (Voice, T)) -> Self {
        Self {
            voice_note,
            caption: Some(caption.into()),
        }
    }
}

#[serde_with::skip_serializing_none]
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum RawRichBlock {
    Anchor { name: String },
    Animation(RichBlockAnimation),
    Audio(RichBlockAudio),
    BlockQuotation(RichBlockBlockQuotation),
    Collage(RichBlockCollage),
    Details(RichBlockDetails),
    Divider,
    Footer { text: RichText },
    List { items: Vec<RichBlockListItem> },
    Map(RichBlockMap),
    MathematicalExpression { expression: String },
    Paragraph { text: RichText },
    Photo(RichBlockPhoto),
    Preformatted(RichBlockPreformatted),
    PullQuotation(RichBlockPullQuotation),
    SectionHeading { text: RichText, size: Integer },
    Slideshow(RichBlockSlideshow),
    Table(RichBlockTable),
    Thinking { text: RichText },
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
