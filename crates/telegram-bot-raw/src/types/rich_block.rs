use serde::{Deserialize, Serialize};

use super::files::animation::Animation;
use super::files::audio::Audio;
use super::files::document::Document;
use super::files::location::Location;
use super::files::photo_size::PhotoSize;
use super::files::video::Video;
use super::files::voice::Voice;
use super::rich_text::RichText;

// ---------------------------------------------------------------------------
// Helper types
// ---------------------------------------------------------------------------

/// Caption of a rich formatted block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockCaption {
    /// Block caption.
    pub text: RichText,
    /// Block credit, corresponding to the HTML tag `<cite>`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<RichText>,
}

impl RichBlockCaption {
    /// Creates a block caption.
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: text.into(),
            credit: None,
        }
    }
}

/// Cell in a rich message table.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockTableCell {
    /// Text in the cell. If omitted, the cell is invisible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<RichText>,
    /// `true` if the cell is a header cell.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_header: Option<bool>,
    /// The number of columns the cell spans, if bigger than 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colspan: Option<i64>,
    /// The number of rows the cell spans, if bigger than 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rowspan: Option<i64>,
    /// Horizontal content alignment: one of `"left"`, `"center"`, `"right"`.
    pub align: String,
    /// Vertical content alignment: one of `"top"`, `"middle"`, `"bottom"`.
    pub valign: String,
}

impl RichBlockTableCell {
    /// Creates a table cell with the given alignment.
    pub fn new(align: impl Into<String>, valign: impl Into<String>) -> Self {
        Self {
            text: None,
            is_header: None,
            colspan: None,
            rowspan: None,
            align: align.into(),
            valign: valign.into(),
        }
    }
}

/// An item of a rich message list.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockListItem {
    /// Label of the item.
    pub label: String,
    /// Content of the item.
    pub blocks: Vec<RichBlock>,
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

impl RichBlockListItem {
    /// Creates a list item with the given label and content.
    /// JSON field name is `"type"` (reserved word in Rust).
    pub fn new(label: impl Into<String>, blocks: Vec<RichBlock>) -> Self {
        Self {
            label: label.into(),
            blocks,
            has_checkbox: None,
            is_checked: None,
            value: None,
            item_type: None,
        }
    }
}

// ---------------------------------------------------------------------------
// RichBlock
// ---------------------------------------------------------------------------

/// A block in a rich formatted message, discriminated by the `"type"` field.
///
/// Corresponds to the Bot API [`RichBlock`](https://core.telegram.org/bots/api#richblock) type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum RichBlock {
    /// Text paragraph (`<p>`).
    Paragraph(RichBlockParagraph),
    /// Section heading (`<h1>`–`<h6>`).
    #[serde(rename = "heading")]
    SectionHeading(RichBlockSectionHeading),
    /// Preformatted block (`<pre><code>`).
    #[serde(rename = "pre")]
    Preformatted(RichBlockPreformatted),
    /// Footer (`<footer>`).
    Footer(RichBlockFooter),
    /// Divider (`<hr/>`).
    Divider(RichBlockDivider),
    /// Mathematical expression (`<tg-math-block>`).
    MathematicalExpression(RichBlockMathematicalExpression),
    /// Anchor (`<a name>`).
    Anchor(RichBlockAnchor),
    /// List (`<ul>` / `<ol>`).
    List(RichBlockList),
    /// Block quotation (`<blockquote>`).
    #[serde(rename = "blockquote")]
    BlockQuotation(RichBlockBlockQuotation),
    /// Expandable block quotation (`<blockquote expandable>`).
    #[serde(rename = "expandable_blockquote")]
    ExpandableBlockQuotation(RichBlockExpandableBlockQuotation),
    /// Centered pull quotation (`<aside>`).
    #[serde(rename = "pullquote")]
    PullQuotation(RichBlockPullQuotation),
    /// Collage (`<tg-collage>`).
    Collage(RichBlockCollage),
    /// Slideshow (`<tg-slideshow>`).
    Slideshow(RichBlockSlideshow),
    /// Table (`<table>`).
    Table(RichBlockTable),
    /// Expandable details (`<details>`).
    Details(RichBlockDetails),
    /// Map (`<tg-map>`).
    Map(RichBlockMap),
    /// Button row (`<tg-button-row>`).
    Buttons(RichBlockButtons),
    /// Animation (`<video>`).
    Animation(RichBlockAnimation),
    /// Music file (`<audio>`).
    Audio(RichBlockAudio),
    /// General file (`<tg-document>`).
    Document(RichBlockDocument),
    /// Photo (`<img>`).
    Photo(RichBlockPhoto),
    /// Video (`<video>`).
    Video(RichBlockVideo),
    /// Voice note (`<audio>`).
    VoiceNote(RichBlockVoiceNote),
    /// Thinking placeholder (`<tg-thinking>`, drafts only).
    Thinking(RichBlockThinking),
}

// ---------------------------------------------------------------------------
// Block types
// ---------------------------------------------------------------------------

/// Text paragraph block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockParagraph {
    /// Text of the block.
    pub text: RichText,
}

impl RichBlockParagraph {
    /// Creates a paragraph block.
    pub fn new(text: impl Into<RichText>) -> Self {
        Self { text: text.into() }
    }
}

/// Section heading block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockSectionHeading {
    /// Text of the block.
    pub text: RichText,
    /// Font size, 1–6 (1 is the largest).
    pub size: i64,
}

impl RichBlockSectionHeading {
    /// Creates a heading block with the given size (1–6).
    pub fn new(text: impl Into<RichText>, size: i64) -> Self {
        Self {
            text: text.into(),
            size,
        }
    }
}

/// Preformatted text block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockPreformatted {
    /// Text of the block.
    pub text: RichText,
    /// Programming language of the text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl RichBlockPreformatted {
    /// Creates a preformatted block.
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: text.into(),
            language: None,
        }
    }
}

/// Footer block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockFooter {
    /// Text of the block.
    pub text: RichText,
}

impl RichBlockFooter {
    /// Creates a footer block.
    pub fn new(text: impl Into<RichText>) -> Self {
        Self { text: text.into() }
    }
}

/// Divider block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockDivider {}

impl RichBlockDivider {
    /// Creates a divider block.
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for RichBlockDivider {
    fn default() -> Self {
        Self::new()
    }
}

/// Mathematical expression block in LaTeX format.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockMathematicalExpression {
    /// The mathematical expression in LaTeX format.
    pub expression: String,
}

impl RichBlockMathematicalExpression {
    /// Creates a mathematical expression block.
    pub fn new(expression: impl Into<String>) -> Self {
        Self {
            expression: expression.into(),
        }
    }
}

/// Anchor block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockAnchor {
    /// The name of the anchor.
    pub name: String,
}

impl RichBlockAnchor {
    /// Creates an anchor block.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// List block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockList {
    /// Items of the list.
    pub items: Vec<RichBlockListItem>,
}

impl RichBlockList {
    /// Creates a list block.
    pub fn new(items: Vec<RichBlockListItem>) -> Self {
        Self { items }
    }
}

/// Block quotation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockBlockQuotation {
    /// Content of the block.
    pub blocks: Vec<RichBlock>,
    /// Credit of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<RichText>,
}

impl RichBlockBlockQuotation {
    /// Creates a block quotation.
    pub fn new(blocks: Vec<RichBlock>) -> Self {
        Self {
            blocks,
            credit: None,
        }
    }
}

/// Expandable block quotation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockExpandableBlockQuotation {
    /// Content of the block.
    pub text: RichText,
    /// Credit of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<RichText>,
}

impl RichBlockExpandableBlockQuotation {
    /// Creates an expandable block quotation.
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: text.into(),
            credit: None,
        }
    }
}

/// Centered pull quotation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockPullQuotation {
    /// Text of the block.
    pub text: RichText,
    /// Credit of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<RichText>,
}

impl RichBlockPullQuotation {
    /// Creates a pull quotation.
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: text.into(),
            credit: None,
        }
    }
}

/// Collage block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockCollage {
    /// Elements of the collage.
    pub blocks: Vec<RichBlock>,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

impl RichBlockCollage {
    /// Creates a collage block.
    pub fn new(blocks: Vec<RichBlock>) -> Self {
        Self {
            blocks,
            caption: None,
        }
    }
}

/// Slideshow block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockSlideshow {
    /// Elements of the slideshow.
    pub blocks: Vec<RichBlock>,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

impl RichBlockSlideshow {
    /// Creates a slideshow block.
    pub fn new(blocks: Vec<RichBlock>) -> Self {
        Self {
            blocks,
            caption: None,
        }
    }
}

/// Table block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockTable {
    /// Cells of the table.
    pub cells: Vec<Vec<RichBlockTableCell>>,
    /// `true` if the table has borders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_bordered: Option<bool>,
    /// `true` if the table is striped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_striped: Option<bool>,
    /// `true` if table cells have smaller indents.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_compact: Option<bool>,
    /// Caption of the table.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichText>,
}

impl RichBlockTable {
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

/// Expandable details block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockDetails {
    /// Always shown summary of the block.
    pub summary: RichText,
    /// Content of the block.
    pub blocks: Vec<RichBlock>,
    /// `true` if the content is visible by default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_open: Option<bool>,
}

impl RichBlockDetails {
    /// Creates a details block.
    pub fn new(summary: impl Into<RichText>, blocks: Vec<RichBlock>) -> Self {
        Self {
            summary: summary.into(),
            blocks,
            is_open: None,
        }
    }
}

/// Map block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockMap {
    /// Location of the center of the map.
    pub location: Location,
    /// Map zoom level.
    pub zoom: i64,
    /// Expected width of the map.
    pub width: i64,
    /// Expected height of the map.
    pub height: i64,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

impl RichBlockMap {
    /// Creates a map block.
    pub fn new(location: Location, zoom: i64, width: i64, height: i64) -> Self {
        Self {
            location,
            zoom,
            width,
            height,
            caption: None,
        }
    }
}

/// Button row block.
///
/// Button payloads are kept as raw JSON until the button type is modeled.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockButtons {
    /// The buttons.
    pub buttons: Vec<serde_json::Value>,
    /// Horizontal alignment: `"left"`, `"center"` or `"right"`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub align: Option<String>,
}

impl RichBlockButtons {
    /// Creates a button row block.
    pub fn new(buttons: Vec<serde_json::Value>) -> Self {
        Self {
            buttons,
            align: None,
        }
    }
}

/// Animation block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockAnimation {
    /// The animation.
    pub animation: Animation,
    /// `true` if the media preview is covered by a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

impl RichBlockAnimation {
    /// Creates an animation block.
    pub fn new(animation: Animation) -> Self {
        Self {
            animation,
            has_spoiler: None,
            caption: None,
        }
    }
}

/// Music file block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockAudio {
    /// The audio.
    pub audio: Audio,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

impl RichBlockAudio {
    /// Creates an audio block.
    pub fn new(audio: Audio) -> Self {
        Self {
            audio,
            caption: None,
        }
    }
}

/// General file block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockDocument {
    /// The document.
    pub document: Document,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

impl RichBlockDocument {
    /// Creates a document block.
    pub fn new(document: Document) -> Self {
        Self {
            document,
            caption: None,
        }
    }
}

/// Photo block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockPhoto {
    /// Available sizes of the photo.
    pub photo: Vec<PhotoSize>,
    /// `true` if the media preview is covered by a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

impl RichBlockPhoto {
    /// Creates a photo block.
    pub fn new(photo: Vec<PhotoSize>) -> Self {
        Self {
            photo,
            has_spoiler: None,
            caption: None,
        }
    }
}

/// Video block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockVideo {
    /// The video.
    pub video: Video,
    /// `true` if the media preview is covered by a spoiler animation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_spoiler: Option<bool>,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

impl RichBlockVideo {
    /// Creates a video block.
    pub fn new(video: Video) -> Self {
        Self {
            video,
            has_spoiler: None,
            caption: None,
        }
    }
}

/// Voice note block.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockVoiceNote {
    /// The voice note.
    pub voice_note: Voice,
    /// Caption of the block.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caption: Option<RichBlockCaption>,
}

impl RichBlockVoiceNote {
    /// Creates a voice note block.
    pub fn new(voice_note: Voice) -> Self {
        Self {
            voice_note,
            caption: None,
        }
    }
}

/// Thinking placeholder block (message drafts only).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichBlockThinking {
    /// Text of the block.
    pub text: RichText,
}

impl RichBlockThinking {
    /// Creates a thinking placeholder block.
    pub fn new(text: impl Into<RichText>) -> Self {
        Self { text: text.into() }
    }
}
