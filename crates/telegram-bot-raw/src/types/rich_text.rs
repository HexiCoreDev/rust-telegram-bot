use serde::{Deserialize, Serialize};

use super::rich_message::RichMessageButton;
use super::user::User;

// ---------------------------------------------------------------------------
// RichText
// ---------------------------------------------------------------------------

/// Rich formatted text: plain text, an array of parts, or one formatted element.
///
/// Corresponds to the Bot API [`RichText`](https://core.telegram.org/bots/api#richtext) type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
#[serde(untagged)]
pub enum RichText {
    /// Plain text.
    Text(String),
    /// A sequence of rich text parts.
    Array(Vec<RichText>),
    /// One formatted rich text element.
    Formatted(Box<RichTextFormatted>),
}

impl Default for RichText {
    fn default() -> Self {
        Self::Text(String::new())
    }
}

impl From<String> for RichText {
    fn from(value: String) -> Self {
        Self::Text(value)
    }
}

impl From<&str> for RichText {
    fn from(value: &str) -> Self {
        Self::Text(value.to_owned())
    }
}

/// One formatted rich text element, discriminated by the `"type"` field.
///
/// Corresponds to the members of the Bot API [`RichText`](https://core.telegram.org/bots/api#richtext) type.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum RichTextFormatted {
    /// Bold text.
    Bold(RichTextBold),
    /// Italicized text.
    Italic(RichTextItalic),
    /// Underlined text.
    Underline(RichTextUnderline),
    /// Strikethrough text.
    Strikethrough(RichTextStrikethrough),
    /// Spoiler-covered text.
    Spoiler(RichTextSpoiler),
    /// Formatted date and time.
    DateTime(RichTextDateTime),
    /// Mention of a user by identifier.
    TextMention(RichTextTextMention),
    /// Subscript text.
    Subscript(RichTextSubscript),
    /// Superscript text.
    Superscript(RichTextSuperscript),
    /// Marked text.
    Marked(RichTextMarked),
    /// Monowidth text.
    Code(RichTextCode),
    /// Custom emoji.
    CustomEmoji(RichTextCustomEmoji),
    /// Mathematical expression in LaTeX format.
    MathematicalExpression(RichTextMathematicalExpression),
    /// Text with a link.
    Url(RichTextUrl),
    /// Text with an email address.
    EmailAddress(RichTextEmailAddress),
    /// Text with a phone number.
    PhoneNumber(RichTextPhoneNumber),
    /// Text with a bank card number.
    BankCardNumber(RichTextBankCardNumber),
    /// Mention by username.
    Mention(RichTextMention),
    /// Hashtag.
    Hashtag(RichTextHashtag),
    /// Cashtag.
    Cashtag(RichTextCashtag),
    /// Bot command.
    BotCommand(RichTextBotCommand),
    /// Button.
    Button(RichTextButton),
    /// Anchor.
    Anchor(RichTextAnchor),
    /// Link to an anchor.
    AnchorLink(RichTextAnchorLink),
    /// Reference.
    Reference(RichTextReference),
    /// Link to a reference.
    ReferenceLink(RichTextReferenceLink),
}

// ---------------------------------------------------------------------------
// Member types
// ---------------------------------------------------------------------------

/// Bold rich text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextBold {
    /// The text.
    pub text: Box<RichText>,
}

impl RichTextBold {
    /// Creates bold rich text from any text convertible into [`RichText`].
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: Box::new(text.into()),
        }
    }
}

/// Italicized rich text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextItalic {
    /// The text.
    pub text: Box<RichText>,
}

impl RichTextItalic {
    /// Creates italic rich text from any text convertible into [`RichText`].
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: Box::new(text.into()),
        }
    }
}

/// Underlined rich text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextUnderline {
    /// The text.
    pub text: Box<RichText>,
}

impl RichTextUnderline {
    /// Creates underlined rich text from any text convertible into [`RichText`].
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: Box::new(text.into()),
        }
    }
}

/// Strikethrough rich text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextStrikethrough {
    /// The text.
    pub text: Box<RichText>,
}

impl RichTextStrikethrough {
    /// Creates strikethrough rich text from any text convertible into [`RichText`].
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: Box::new(text.into()),
        }
    }
}

/// Spoiler-covered rich text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextSpoiler {
    /// The text.
    pub text: Box<RichText>,
}

impl RichTextSpoiler {
    /// Creates spoiler rich text from any text convertible into [`RichText`].
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: Box::new(text.into()),
        }
    }
}

/// Formatted date and time.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextDateTime {
    /// The text.
    pub text: Box<RichText>,
    /// The Unix time associated with the entity.
    pub unix_time: i64,
    /// The format string defining how the date and time are displayed.
    pub date_time_format: String,
}

impl RichTextDateTime {
    /// Creates date-time rich text.
    pub fn new(
        text: impl Into<RichText>,
        unix_time: i64,
        date_time_format: impl Into<String>,
    ) -> Self {
        Self {
            text: Box::new(text.into()),
            unix_time,
            date_time_format: date_time_format.into(),
        }
    }
}

/// Mention of a Telegram user by identifier.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextTextMention {
    /// The text.
    pub text: Box<RichText>,
    /// The mentioned user.
    pub user: User,
}

impl RichTextTextMention {
    /// Creates a user-mention rich text.
    pub fn new(text: impl Into<RichText>, user: User) -> Self {
        Self {
            text: Box::new(text.into()),
            user,
        }
    }
}

/// Subscript rich text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextSubscript {
    /// The text.
    pub text: Box<RichText>,
}

impl RichTextSubscript {
    /// Creates subscript rich text from any text convertible into [`RichText`].
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: Box::new(text.into()),
        }
    }
}

/// Superscript rich text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextSuperscript {
    /// The text.
    pub text: Box<RichText>,
}

impl RichTextSuperscript {
    /// Creates superscript rich text from any text convertible into [`RichText`].
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: Box::new(text.into()),
        }
    }
}

/// Marked rich text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextMarked {
    /// The text.
    pub text: Box<RichText>,
}

impl RichTextMarked {
    /// Creates marked rich text from any text convertible into [`RichText`].
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: Box::new(text.into()),
        }
    }
}

/// Monowidth rich text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextCode {
    /// The text.
    pub text: Box<RichText>,
}

impl RichTextCode {
    /// Creates monowidth rich text from any text convertible into [`RichText`].
    pub fn new(text: impl Into<RichText>) -> Self {
        Self {
            text: Box::new(text.into()),
        }
    }
}

/// Custom emoji rich text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextCustomEmoji {
    /// Unique identifier of the custom emoji.
    pub custom_emoji_id: String,
    /// Alternative emoji shown where the custom emoji cannot be displayed.
    pub alternative_text: String,
}

impl RichTextCustomEmoji {
    /// Creates custom emoji rich text.
    pub fn new(custom_emoji_id: impl Into<String>, alternative_text: impl Into<String>) -> Self {
        Self {
            custom_emoji_id: custom_emoji_id.into(),
            alternative_text: alternative_text.into(),
        }
    }
}

/// Mathematical expression in LaTeX format.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextMathematicalExpression {
    /// The expression in LaTeX format.
    pub expression: String,
}

impl RichTextMathematicalExpression {
    /// Creates a mathematical expression rich text.
    pub fn new(expression: impl Into<String>) -> Self {
        Self {
            expression: expression.into(),
        }
    }
}

/// Rich text with a link.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextUrl {
    /// The text.
    pub text: Box<RichText>,
    /// URL of the link.
    pub url: String,
}

impl RichTextUrl {
    /// Creates linked rich text.
    pub fn new(text: impl Into<RichText>, url: impl Into<String>) -> Self {
        Self {
            text: Box::new(text.into()),
            url: url.into(),
        }
    }
}

/// Rich text with an email address.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextEmailAddress {
    /// The text.
    pub text: Box<RichText>,
    /// The email address.
    pub email_address: String,
}

impl RichTextEmailAddress {
    /// Creates email-address rich text.
    pub fn new(text: impl Into<RichText>, email_address: impl Into<String>) -> Self {
        Self {
            text: Box::new(text.into()),
            email_address: email_address.into(),
        }
    }
}

/// Rich text with a phone number.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextPhoneNumber {
    /// The text.
    pub text: Box<RichText>,
    /// The phone number.
    pub phone_number: String,
}

impl RichTextPhoneNumber {
    /// Creates phone-number rich text.
    pub fn new(text: impl Into<RichText>, phone_number: impl Into<String>) -> Self {
        Self {
            text: Box::new(text.into()),
            phone_number: phone_number.into(),
        }
    }
}

/// Rich text with a bank card number.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextBankCardNumber {
    /// The text.
    pub text: Box<RichText>,
    /// The bank card number.
    pub bank_card_number: String,
}

impl RichTextBankCardNumber {
    /// Creates bank-card-number rich text.
    pub fn new(text: impl Into<RichText>, bank_card_number: impl Into<String>) -> Self {
        Self {
            text: Box::new(text.into()),
            bank_card_number: bank_card_number.into(),
        }
    }
}

/// Mention by username.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextMention {
    /// The text.
    pub text: Box<RichText>,
    /// The username.
    pub username: String,
}

impl RichTextMention {
    /// Creates username-mention rich text.
    pub fn new(text: impl Into<RichText>, username: impl Into<String>) -> Self {
        Self {
            text: Box::new(text.into()),
            username: username.into(),
        }
    }
}

/// Hashtag rich text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextHashtag {
    /// The text.
    pub text: Box<RichText>,
    /// The hashtag.
    pub hashtag: String,
}

impl RichTextHashtag {
    /// Creates hashtag rich text.
    pub fn new(text: impl Into<RichText>, hashtag: impl Into<String>) -> Self {
        Self {
            text: Box::new(text.into()),
            hashtag: hashtag.into(),
        }
    }
}

/// Cashtag rich text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextCashtag {
    /// The text.
    pub text: Box<RichText>,
    /// The cashtag.
    pub cashtag: String,
}

impl RichTextCashtag {
    /// Creates cashtag rich text.
    pub fn new(text: impl Into<RichText>, cashtag: impl Into<String>) -> Self {
        Self {
            text: Box::new(text.into()),
            cashtag: cashtag.into(),
        }
    }
}

/// Bot command rich text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextBotCommand {
    /// The text.
    pub text: Box<RichText>,
    /// The bot command.
    pub bot_command: String,
}

impl RichTextBotCommand {
    /// Creates bot-command rich text.
    pub fn new(text: impl Into<RichText>, bot_command: impl Into<String>) -> Self {
        Self {
            text: Box::new(text.into()),
            bot_command: bot_command.into(),
        }
    }
}

/// Button rich text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextButton {
    /// The button.
    pub button: Box<RichMessageButton>,
}

impl RichTextButton {
    /// Creates button rich text.
    pub fn new(button: RichMessageButton) -> Self {
        Self {
            button: Box::new(button),
        }
    }
}

/// Anchor rich text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextAnchor {
    /// The name of the anchor.
    pub name: String,
}

impl RichTextAnchor {
    /// Creates anchor rich text.
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

/// Link to an anchor.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextAnchorLink {
    /// The link text.
    pub text: Box<RichText>,
    /// The name of the anchor. An empty name links back to the top of the message.
    pub anchor_name: String,
}

impl RichTextAnchorLink {
    /// Creates anchor-link rich text.
    pub fn new(text: impl Into<RichText>, anchor_name: impl Into<String>) -> Self {
        Self {
            text: Box::new(text.into()),
            anchor_name: anchor_name.into(),
        }
    }
}

/// Reference rich text.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextReference {
    /// Text of the reference.
    pub text: Box<RichText>,
    /// The name of the reference.
    pub name: String,
}

impl RichTextReference {
    /// Creates reference rich text.
    pub fn new(text: impl Into<RichText>, name: impl Into<String>) -> Self {
        Self {
            text: Box::new(text.into()),
            name: name.into(),
        }
    }
}

/// Link to a reference.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct RichTextReferenceLink {
    /// The link text.
    pub text: Box<RichText>,
    /// The name of the reference.
    pub reference_name: String,
}

impl RichTextReferenceLink {
    /// Creates reference-link rich text.
    pub fn new(text: impl Into<RichText>, reference_name: impl Into<String>) -> Self {
        Self {
            text: Box::new(text.into()),
            reference_name: reference_name.into(),
        }
    }
}
