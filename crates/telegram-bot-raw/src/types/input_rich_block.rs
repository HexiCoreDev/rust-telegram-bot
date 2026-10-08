use serde::{Deserialize, Serialize};

use super::files::input_media::{
    InputMediaAnimation, InputMediaAudio, InputMediaDocument, InputMediaPhoto, InputMediaVideo,
    InputMediaVoiceNote,
};
use super::files::location::Location;
use super::rich_block::{RichBlockCaption, RichBlockTableCell};
use super::rich_message::RichMessageButton;
use super::rich_text::RichText;

// ---------------------------------------------------------------------------
// InputRichBlockListItem
// ---------------------------------------------------------------------------

/// An item of a list to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockListItem {
    /// Content of the item.
    pub blocks: Vec<InputRichBlock>,
    /// `true` if the item has a checkbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_checkbox: Option<bool>,
    /// `true` if the item has a checked checkbox.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_checked: Option<bool>,
    /// For ordered lists, the numeric value of the item label.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    /// For ordered lists, the label type: `"a"`, `"A"`, `"i"`, `"I"` or `"1"`.
    ///
    /// JSON field name is `"type"` (reserved word in Rust).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub item_type: Option<String>,
}

impl InputRichBlockListItem {
    /// Creates a list item with the given content.
    pub fn new(blocks: Vec<InputRichBlock>) -> Self {
        Self {
            blocks,
            has_checkbox: None,
            is_checked: None,
            value: None,
            item_type: None,
        }
    }
}

// ---------------------------------------------------------------------------
// InputRichBlock
// ---------------------------------------------------------------------------

/// A block of a rich formatted message to be sent, discriminated by the
/// `"type"` field.
///
/// Corresponds to the Bot API [`InputRichBlock`](https://core.telegram.org/bots/api#inputrichblock) type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum InputRichBlock {
    /// Text paragraph (`<p>`).
    Paragraph(InputRichBlockParagraph),
    /// Section heading (`<h1>`–`<h6>`).
    #[serde(rename = "heading")]
    SectionHeading(InputRichBlockSectionHeading),
    /// Preformatted block (`<pre><code>`).
    #[serde(rename = "pre")]
    Preformatted(InputRichBlockPreformatted),
    /// Footer (`<footer>`).
    Footer(InputRichBlockFooter),
    /// Divider (`<hr/>`).
    Divider(InputRichBlockDivider),
    /// Mathematical expression (`<tg-math-block>`).
    MathematicalExpression(InputRichBlockMathematicalExpression),
    /// Anchor (`<a name>`).
    Anchor(InputRichBlockAnchor),
    /// List (`<ul>` / `<ol>`).
    List(InputRichBlockList),
    /// Block quotation (`<blockquote>`).
    #[serde(rename = "blockquote")]
    BlockQuotation(InputRichBlockBlockQuotation),
    /// Expandable block quotation (`<blockquote expandable>`).
    #[serde(rename = "expandable_blockquote")]
    ExpandableBlockQuotation(InputRichBlockExpandableBlockQuotation),
    /// Centered pull quotation (`<aside>`).
    #[serde(rename = "pullquote")]
    PullQuotation(InputRichBlockPullQuotation),
    /// Collage (`<tg-collage>`).
    Collage(InputRichBlockCollage),
    /// Slideshow (`<tg-slideshow>`).
    Slideshow(InputRichBlockSlideshow),
    /// Table (`<table>`).
    Table(InputRichBlockTable),
    /// Expandable details (`<details>`).
    Details(InputRichBlockDetails),
    /// Map (`<tg-map>`).
    Map(InputRichBlockMap),
    /// Button row (`<tg-button-row>`).
    Buttons(InputRichBlockButtons),
    /// General file (`<tg-document>`).
    Document(InputRichBlockDocument),
    /// Animation (`<video>`).
    Animation(InputRichBlockAnimation),
    /// Music file (`<audio>`).
    Audio(InputRichBlockAudio),
    /// Photo (`<img>`).
    Photo(InputRichBlockPhoto),
    /// Video (`<video>`).
    Video(InputRichBlockVideo),
    /// Voice note (`<audio>`).
    VoiceNote(InputRichBlockVoiceNote),
    /// Thinking placeholder (`<tg-thinking>`, drafts only).
    Thinking(InputRichBlockThinking),
}

// ---------------------------------------------------------------------------
// Block types
// ---------------------------------------------------------------------------

/// Text paragraph block to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockParagraph {
    /// Text of the block.
    pub text: RichText,
}

impl InputRichBlockParagraph {
    /// Creates a paragraph block.
    pub fn new(text: impl Into<RichText>) -> Self {
        Self { text: text.into() }
    }
}

/// Section heading block to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockSectionHeading {
    /// Text of the block.
    pub text: RichText,
    /// Font size, 1–6 (1 is the largest).
    pub size: i64,
}

impl InputRichBlockSectionHeading {
    /// Creates a heading block with the given size (1–6).
    pub fn new(text: impl Into<RichText>, size: i64) -> Self {
        Self {
            text: text.into(),
            size,
        }
    }
}

/// Preformatted text block to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockPreformatted {
    /// Text of the block.
    pub text: RichText,
    /// Programming language of the text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl InputRichBlockPreformatted {
    /// Creates a preformatted block.
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: text.into(),
            language: None,
        }
    }
}

/// Footer block to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockFooter {
    /// Text of the block.
    pub text: RichText,
}

impl InputRichBlockFooter {
    /// Creates a footer block.
    pub fn new(text: impl Into<RichText>) -> Self {
        Self { text: text.into() }
    }
}

/// Divider block to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockDivider {}

impl InputRichBlockDivider {
    /// Creates a divider block.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for InputRichBlockDivider {
    fn default() -> Self {
        Self::new()
    }
}

/// Mathematical expression block in LaTeX format to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockMathematicalExpression {
    /// The mathematical expression in LaTeX format.
    pub expression: String,
}

impl InputRichBlockMathematicalExpression {
    /// Creates a mathematical expression block.
    pub fn new(expression: impl Into<String>) -> Self {
        Self {
            expression: expression.into(),
        }
    }
}

/// Anchor block to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockAnchor {
    /// The name of the anchor.
    pub name: String,
}

impl InputRichBlockAnchor {
    /// Creates an anchor block.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// List block to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockList {
    /// Items of the list.
    pub items: Vec<InputRichBlockListItem>,
}

impl InputRichBlockList {
    /// Creates a list block.
    pub fn new(items: Vec<InputRichBlockListItem>) -> Self {
        Self { items }
    }
}

/// Block quotation to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockBlockQuotation {
    /// Content of the block.
    pub blocks: Vec<InputRichBlock>,
    /// Credit of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<RichText>,
}

impl InputRichBlockBlockQuotation {
    /// Creates a block quotation.
    pub fn new(blocks: Vec<InputRichBlock>) -> Self {
        Self {
            blocks,
            credit: None,
        }
    }
}

/// Expandable block quotation to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockExpandableBlockQuotation {
    /// Content of the block.
    pub text: RichText,
    /// Credit of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<RichText>,
}

impl InputRichBlockExpandableBlockQuotation {
    /// Creates an expandable block quotation.
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: text.into(),
            credit: None,
        }
    }
}

/// Centered pull quotation to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockPullQuotation {
    /// Text of the block.
    pub text: RichText,
    /// Credit of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<RichText>,
}

impl InputRichBlockPullQuotation {
    /// Creates a pull quotation.
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: text.into(),
            credit: None,
        }
    }
}

/// Collage block to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockCollage {
    /// Elements of the collage.
    pub blocks: Vec<InputRichBlock>,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

impl InputRichBlockCollage {
    /// Creates a collage block.
    pub fn new(blocks: Vec<InputRichBlock>) -> Self {
        Self {
            blocks,
            caption: None,
        }
    }
}

/// Slideshow block to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockSlideshow {
    /// Elements of the slideshow.
    pub blocks: Vec<InputRichBlock>,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

impl InputRichBlockSlideshow {
    /// Creates a slideshow block.
    pub fn new(blocks: Vec<InputRichBlock>) -> Self {
        Self {
            blocks,
            caption: None,
        }
    }
}

/// Table block to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockTable {
    /// Cells of the table.
    pub cells: Vec<Vec<RichBlockTableCell>>,
    /// `true` if the table has borders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bordered: Option<bool>,
    /// `true` if the table is striped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_striped: Option<bool>,
    /// `true` if table cells must have smaller indents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_compact: Option<bool>,
    /// Caption of the table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichText>,
}

impl InputRichBlockTable {
    /// Creates a table block.
    pub fn new(cells: Vec<Vec<RichBlockTableCell>>) -> Self {
        Self {
            cells,
            is_bordered: None,
            is_striped: None,
            is_compact: None,
            caption: None,
        }
    }
}

/// Expandable details block to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockDetails {
    /// Always shown summary of the block.
    pub summary: RichText,
    /// Content of the block.
    pub blocks: Vec<InputRichBlock>,
    /// `true` if the content is visible by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_open: Option<bool>,
}

impl InputRichBlockDetails {
    /// Creates a details block.
    pub fn new(summary: impl Into<RichText>, blocks: Vec<InputRichBlock>) -> Self {
        Self {
            summary: summary.into(),
            blocks,
            is_open: None,
        }
    }
}

/// Map block to be sent.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockMap {
    /// Location of the center of the map.
    pub location: Location,
    /// Map zoom level, 0–24.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoom: Option<i64>,
    /// Map width, 0–10000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
    /// Map height, 0–10000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<i64>,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

impl InputRichBlockMap {
    /// Creates a map block centered on the given location.
    pub fn new(location: Location) -> Self {
        Self {
            location,
            zoom: None,
            width: None,
            height: None,
            caption: None,
        }
    }
}

/// Button row block to be sent, with 1–8 buttons.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockButtons {
    /// List of buttons to send.
    pub buttons: Vec<RichMessageButton>,
    /// Horizontal alignment: `"left"`, `"center"` or `"right"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<String>,
}

impl InputRichBlockButtons {
    /// Creates a button row block.
    pub fn new(buttons: Vec<RichMessageButton>) -> Self {
        Self {
            buttons,
            align: None,
        }
    }
}

/// General file block to be sent. The caption is ignored.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockDocument {
    /// The document.
    pub document: InputMediaDocument,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

impl InputRichBlockDocument {
    /// Creates a document block.
    pub fn new(document: InputMediaDocument) -> Self {
        Self {
            document,
            caption: None,
        }
    }
}

/// Animation block to be sent. The caption is ignored.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockAnimation {
    /// The animation.
    pub animation: InputMediaAnimation,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

impl InputRichBlockAnimation {
    /// Creates an animation block.
    pub fn new(animation: InputMediaAnimation) -> Self {
        Self {
            animation,
            caption: None,
        }
    }
}

/// Music file block to be sent. The caption is ignored.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockAudio {
    /// The audio.
    pub audio: InputMediaAudio,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

impl InputRichBlockAudio {
    /// Creates an audio block.
    pub fn new(audio: InputMediaAudio) -> Self {
        Self {
            audio,
            caption: None,
        }
    }
}

/// Photo block to be sent. The caption is ignored.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockPhoto {
    /// The photo.
    pub photo: InputMediaPhoto,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

impl InputRichBlockPhoto {
    /// Creates a photo block.
    pub fn new(photo: InputMediaPhoto) -> Self {
        Self {
            photo,
            caption: None,
        }
    }
}

/// Video block to be sent. The caption is ignored.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockVideo {
    /// The video.
    pub video: InputMediaVideo,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

impl InputRichBlockVideo {
    /// Creates a video block.
    pub fn new(video: InputMediaVideo) -> Self {
        Self {
            video,
            caption: None,
        }
    }
}

/// Voice note block to be sent. The caption is ignored.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockVoiceNote {
    /// The voice note.
    pub voice_note: InputMediaVoiceNote,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

impl InputRichBlockVoiceNote {
    /// Creates a voice note block.
    pub fn new(voice_note: InputMediaVoiceNote) -> Self {
        Self {
            voice_note,
            caption: None,
        }
    }
}

/// Thinking placeholder block to be sent (message drafts only).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichBlockThinking {
    /// Text of the block.
    pub text: RichText,
}

impl InputRichBlockThinking {
    /// Creates a thinking placeholder block.
    pub fn new(text: impl Into<RichText>) -> Self {
        Self { text: text.into() }
    }
}

// ---------------------------------------------------------------------------
// InputRichMessageMedia
// ---------------------------------------------------------------------------

/// Media embedded in an outgoing rich message, referenced from `html` or
/// `markdown` through `tg://photo?id=` style links.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct InputRichMessageMedia {
    /// Unique identifier of the media used in the link. 1–64 characters;
    /// only `A-Z`, `a-z`, `0-9`, `_` and `-` are allowed.
    pub id: String,
    /// The media to be sent. Everything except the media itself and its
    /// properties is ignored.
    pub media: InputRichMessageMediaContent,
}

impl InputRichMessageMedia {
    /// Creates rich message media with the given id and content.
    pub fn new(id: impl Into<String>, media: InputRichMessageMediaContent) -> Self {
        Self {
            id: id.into(),
            media,
        }
    }
}

/// Media content embedded in an outgoing rich message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum InputRichMessageMediaContent {
    /// An animation.
    Animation(InputMediaAnimation),
    /// An audio file.
    Audio(InputMediaAudio),
    /// A general file.
    Document(InputMediaDocument),
    /// A photo.
    Photo(InputMediaPhoto),
    /// A video.
    Video(InputMediaVideo),
    /// A voice message.
    VoiceNote(InputMediaVoiceNote),
}
