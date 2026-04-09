// ── Version ──────────────────────────────────────────────────────────────────

/// Telegram Bot API version supported by this crate.
pub const BOT_API_VERSION: &str = "9.6";
/// Major component of `BOT_API_VERSION`.
pub const BOT_API_VERSION_MAJOR: u32 = 9;
/// Minor component of `BOT_API_VERSION`.
pub const BOT_API_VERSION_MINOR: u32 = 6;

/// Ports accepted by `set_webhook`.
pub const SUPPORTED_WEBHOOK_PORTS: &[u16] = &[443, 80, 88, 8443];

/// The value of one nanostar as used in `StarTransaction.nanostar_amount`.
pub const NANOSTAR_VALUE: f64 = 1.0 / 1_000_000_000.0;

// ── AccentColor ───────────────────────────────────────────────────────────────

/// A Telegram accent color entry with optional hex color lists.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AccentColorEntry {
    /// The numeric identifier.
    pub identifier: u32,
    /// Optional human-readable name.
    pub name: Option<&'static str>,
    /// Light-theme hex colors.
    pub light_colors: &'static [u32],
    /// Dark-theme hex colors.
    pub dark_colors: &'static [u32],
}

/// Accent color palette for `ChatFullInfo.accent_color_id`.
pub mod accent_color {
    use super::AccentColorEntry;
    /// Red accent color (identifier 0).
    pub const COLOR_000: AccentColorEntry = AccentColorEntry {
        identifier: 0,
        name: Some("red"),
        light_colors: &[],
        dark_colors: &[],
    };
    /// Orange accent color (identifier 1).
    pub const COLOR_001: AccentColorEntry = AccentColorEntry {
        identifier: 1,
        name: Some("orange"),
        light_colors: &[],
        dark_colors: &[],
    };
    /// Purple/violet accent color (identifier 2).
    pub const COLOR_002: AccentColorEntry = AccentColorEntry {
        identifier: 2,
        name: Some("purple/violet"),
        light_colors: &[],
        dark_colors: &[],
    };
    /// Green accent color (identifier 3).
    pub const COLOR_003: AccentColorEntry = AccentColorEntry {
        identifier: 3,
        name: Some("green"),
        light_colors: &[],
        dark_colors: &[],
    };
    /// Cyan accent color (identifier 4).
    pub const COLOR_004: AccentColorEntry = AccentColorEntry {
        identifier: 4,
        name: Some("cyan"),
        light_colors: &[],
        dark_colors: &[],
    };
    /// Blue accent color (identifier 5).
    pub const COLOR_005: AccentColorEntry = AccentColorEntry {
        identifier: 5,
        name: Some("blue"),
        light_colors: &[],
        dark_colors: &[],
    };
    /// Pink accent color (identifier 6).
    pub const COLOR_006: AccentColorEntry = AccentColorEntry {
        identifier: 6,
        name: Some("pink"),
        light_colors: &[],
        dark_colors: &[],
    };
    /// Custom two-color accent (identifier 7).
    pub const COLOR_007: AccentColorEntry = AccentColorEntry {
        identifier: 7,
        name: None,
        light_colors: &[0xE15052, 0xF9AE63],
        dark_colors: &[0xFF9380, 0x992F37],
    };
    /// Custom two-color accent (identifier 8).
    pub const COLOR_008: AccentColorEntry = AccentColorEntry {
        identifier: 8,
        name: None,
        light_colors: &[0xE0802B, 0xFAC534],
        dark_colors: &[0xECB04E, 0xC35714],
    };
    /// Custom two-color accent (identifier 9).
    pub const COLOR_009: AccentColorEntry = AccentColorEntry {
        identifier: 9,
        name: None,
        light_colors: &[0xA05FF3, 0xF48FFF],
        dark_colors: &[0xC697FF, 0x5E31C8],
    };
    /// Custom two-color accent (identifier 10).
    pub const COLOR_010: AccentColorEntry = AccentColorEntry {
        identifier: 10,
        name: None,
        light_colors: &[0x27A910, 0xA7DC57],
        dark_colors: &[0xA7EB6E, 0x167E2D],
    };
    /// Custom two-color accent (identifier 11).
    pub const COLOR_011: AccentColorEntry = AccentColorEntry {
        identifier: 11,
        name: None,
        light_colors: &[0x27ACCE, 0x82E8D6],
        dark_colors: &[0x40D8D0, 0x045C7F],
    };
    /// Custom two-color accent (identifier 12).
    pub const COLOR_012: AccentColorEntry = AccentColorEntry {
        identifier: 12,
        name: None,
        light_colors: &[0x3391D4, 0x7DD3F0],
        dark_colors: &[0x52BFFF, 0x0B5494],
    };
    /// Custom two-color accent (identifier 13).
    pub const COLOR_013: AccentColorEntry = AccentColorEntry {
        identifier: 13,
        name: None,
        light_colors: &[0xDD4371, 0xFFBE9F],
        dark_colors: &[0xFF86A6, 0x8E366E],
    };
    /// Custom three-color accent (identifier 14).
    pub const COLOR_014: AccentColorEntry = AccentColorEntry {
        identifier: 14,
        name: None,
        light_colors: &[0x247BED, 0xF04856, 0xFFFFFF],
        dark_colors: &[0x3FA2FE, 0xE5424F, 0xFFFFFF],
    };
    /// Custom three-color accent (identifier 15).
    pub const COLOR_015: AccentColorEntry = AccentColorEntry {
        identifier: 15,
        name: None,
        light_colors: &[0xD67722, 0x1EA011, 0xFFFFFF],
        dark_colors: &[0xFF905E, 0x32A527, 0xFFFFFF],
    };
    /// Custom three-color accent (identifier 16).
    pub const COLOR_016: AccentColorEntry = AccentColorEntry {
        identifier: 16,
        name: None,
        light_colors: &[0x179E42, 0xE84A3F, 0xFFFFFF],
        dark_colors: &[0x66D364, 0xD5444F, 0xFFFFFF],
    };
    /// Custom three-color accent (identifier 17).
    pub const COLOR_017: AccentColorEntry = AccentColorEntry {
        identifier: 17,
        name: None,
        light_colors: &[0x2894AF, 0x6FC456, 0xFFFFFF],
        dark_colors: &[0x22BCE2, 0x3DA240, 0xFFFFFF],
    };
    /// Custom three-color accent (identifier 18).
    pub const COLOR_018: AccentColorEntry = AccentColorEntry {
        identifier: 18,
        name: None,
        light_colors: &[0x0C9AB3, 0xFFAD95, 0xFFE6B5],
        dark_colors: &[0x22BCE2, 0xFF9778, 0xFFDA6B],
    };
    /// Custom three-color accent (identifier 19).
    pub const COLOR_019: AccentColorEntry = AccentColorEntry {
        identifier: 19,
        name: None,
        light_colors: &[0x7757D6, 0xF79610, 0xFFDE8E],
        dark_colors: &[0x9791FF, 0xF2731D, 0xFFDB59],
    };
    /// Custom three-color accent (identifier 20).
    pub const COLOR_020: AccentColorEntry = AccentColorEntry {
        identifier: 20,
        name: None,
        light_colors: &[0x1585CF, 0xF2AB1D, 0xFFFFFF],
        dark_colors: &[0x3DA6EB, 0xEEA51D, 0xFFFFFF],
    };
}

/// Profile accent color palette for `ChatFullInfo.profile_accent_color_id`.
pub mod profile_accent_color {
    use super::AccentColorEntry;
    /// Single-color profile accent (identifier 0).
    pub const COLOR_000: AccentColorEntry = AccentColorEntry {
        identifier: 0,
        name: None,
        light_colors: &[0xBA5650],
        dark_colors: &[0x9C4540],
    };
    /// Single-color profile accent (identifier 1).
    pub const COLOR_001: AccentColorEntry = AccentColorEntry {
        identifier: 1,
        name: None,
        light_colors: &[0xC27C3E],
        dark_colors: &[0x945E2C],
    };
    /// Single-color profile accent (identifier 2).
    pub const COLOR_002: AccentColorEntry = AccentColorEntry {
        identifier: 2,
        name: None,
        light_colors: &[0x956AC8],
        dark_colors: &[0x715099],
    };
    /// Single-color profile accent (identifier 3).
    pub const COLOR_003: AccentColorEntry = AccentColorEntry {
        identifier: 3,
        name: None,
        light_colors: &[0x49A355],
        dark_colors: &[0x33713B],
    };
    /// Single-color profile accent (identifier 4).
    pub const COLOR_004: AccentColorEntry = AccentColorEntry {
        identifier: 4,
        name: None,
        light_colors: &[0x3E97AD],
        dark_colors: &[0x387E87],
    };
    /// Single-color profile accent (identifier 5).
    pub const COLOR_005: AccentColorEntry = AccentColorEntry {
        identifier: 5,
        name: None,
        light_colors: &[0x5A8FBB],
        dark_colors: &[0x477194],
    };
    /// Single-color profile accent (identifier 6).
    pub const COLOR_006: AccentColorEntry = AccentColorEntry {
        identifier: 6,
        name: None,
        light_colors: &[0xB85378],
        dark_colors: &[0x944763],
    };
    /// Single-color profile accent (identifier 7).
    pub const COLOR_007: AccentColorEntry = AccentColorEntry {
        identifier: 7,
        name: None,
        light_colors: &[0x7F8B95],
        dark_colors: &[0x435261],
    };
    /// Two-color profile accent (identifier 8).
    pub const COLOR_008: AccentColorEntry = AccentColorEntry {
        identifier: 8,
        name: None,
        light_colors: &[0xC9565D, 0xD97C57],
        dark_colors: &[0x994343, 0xAC583E],
    };
    /// Two-color profile accent (identifier 9).
    pub const COLOR_009: AccentColorEntry = AccentColorEntry {
        identifier: 9,
        name: None,
        light_colors: &[0xCF7244, 0xCC9433],
        dark_colors: &[0x8F552F, 0xA17232],
    };
    /// Two-color profile accent (identifier 10).
    pub const COLOR_010: AccentColorEntry = AccentColorEntry {
        identifier: 10,
        name: None,
        light_colors: &[0x9662D4, 0xB966B6],
        dark_colors: &[0x634691, 0x9250A2],
    };
    /// Two-color profile accent (identifier 11).
    pub const COLOR_011: AccentColorEntry = AccentColorEntry {
        identifier: 11,
        name: None,
        light_colors: &[0x3D9755, 0x89A650],
        dark_colors: &[0x296A43, 0x5F8F44],
    };
    /// Two-color profile accent (identifier 12).
    pub const COLOR_012: AccentColorEntry = AccentColorEntry {
        identifier: 12,
        name: None,
        light_colors: &[0x3D95BA, 0x50AD98],
        dark_colors: &[0x306C7C, 0x3E987E],
    };
    /// Two-color profile accent (identifier 13).
    pub const COLOR_013: AccentColorEntry = AccentColorEntry {
        identifier: 13,
        name: None,
        light_colors: &[0x538BC2, 0x4DA8BD],
        dark_colors: &[0x38618C, 0x458BA1],
    };
    /// Two-color profile accent (identifier 14).
    pub const COLOR_014: AccentColorEntry = AccentColorEntry {
        identifier: 14,
        name: None,
        light_colors: &[0xB04F74, 0xD1666D],
        dark_colors: &[0x884160, 0xA65259],
    };
    /// Two-color profile accent (identifier 15).
    pub const COLOR_015: AccentColorEntry = AccentColorEntry {
        identifier: 15,
        name: None,
        light_colors: &[0x637482, 0x7B8A97],
        dark_colors: &[0x53606E, 0x384654],
    };
}

// ── String enums ─────────────────────────────────────────────────────────────

use serde::{Deserialize, Serialize};
use std::fmt;

/// Available types of `BackgroundType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum BackgroundTypeType {
    /// A background filled with a single color, gradient, or freeform gradient.
    Fill,
    /// A wallpaper in JPEG format.
    Wallpaper,
    /// A PNG or TGV pattern to be combined with a background fill.
    Pattern,
    /// A background taken from a chat theme.
    ChatTheme,
}

/// Available types of `BackgroundFill`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum BackgroundFillType {
    /// A solid single-color fill.
    Solid,
    /// A linear gradient fill.
    Gradient,
    /// A freeform gradient with multiple colors.
    FreeformGradient,
}

/// Available types of `BotCommandScope`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum BotCommandScopeType {
    /// Default scope covering all chats.
    Default,
    /// Scope covering all private chats.
    AllPrivateChats,
    /// Scope covering all group chats.
    AllGroupChats,
    /// Scope covering all chat administrators.
    AllChatAdministrators,
    /// Scope covering a specific chat.
    Chat,
    /// Scope covering administrators of a specific chat.
    ChatAdministrators,
    /// Scope covering a specific member in a specific chat.
    ChatMember,
}

/// Available chat actions for `Bot::send_chat_action`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ChatAction {
    /// Bot is choosing a sticker.
    ChooseSticker,
    /// Bot is finding a location.
    FindLocation,
    /// Bot is recording a voice message.
    RecordVoice,
    /// Bot is recording a video.
    RecordVideo,
    /// Bot is recording a video note.
    RecordVideoNote,
    /// Bot is typing a text message.
    Typing,
    /// Bot is uploading a voice message.
    UploadVoice,
    /// Bot is uploading a document.
    UploadDocument,
    /// Bot is uploading a photo.
    UploadPhoto,
    /// Bot is uploading a video.
    UploadVideo,
    /// Bot is uploading a video note.
    UploadVideoNote,
}

/// Available sources for a `ChatBoostSource`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ChatBoostSources {
    /// Boost obtained via a gift code.
    GiftCode,
    /// Boost obtained via a giveaway.
    Giveaway,
    /// Boost obtained via a Telegram Premium subscription.
    Premium,
}

/// Available states for `ChatMember`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ChatMemberStatus {
    /// Chat member is an administrator.
    #[serde(rename = "administrator")]
    Administrator,
    /// Chat member is the owner (creator) of the chat.
    #[serde(rename = "creator")]
    Owner,
    /// Chat member has been banned (kicked) from the chat.
    #[serde(rename = "kicked")]
    Banned,
    /// Chat member has left the chat.
    #[serde(rename = "left")]
    Left,
    /// Chat member is a regular member.
    #[serde(rename = "member")]
    Member,
    /// Chat member is restricted (limited permissions).
    #[serde(rename = "restricted")]
    Restricted,
}

/// Available types of `Chat`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ChatType {
    /// Chat type used for inline query results sent on behalf of the user.
    Sender,
    /// A private one-on-one chat.
    Private,
    /// A basic group chat.
    Group,
    /// A supergroup chat.
    Supergroup,
    /// A channel.
    Channel,
}

/// Available emoji for `Dice` / `Bot::send_dice`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DiceEmoji {
    /// Game die emoji; values 1--6.
    #[serde(rename = "\u{1F3B2}")]
    Dice,
    /// Darts emoji; values 1--6.
    #[serde(rename = "\u{1F3AF}")]
    Darts,
    /// Basketball emoji; values 1--5.
    #[serde(rename = "\u{1F3C0}")]
    Basketball,
    /// Football (soccer) emoji; values 1--5.
    #[serde(rename = "\u{26BD}")]
    Football,
    /// Slot machine emoji; values 1--64.
    #[serde(rename = "\u{1F3B0}")]
    SlotMachine,
    /// Bowling emoji; values 1--6.
    #[serde(rename = "\u{1F3B3}")]
    Bowling,
}

/// Available types of `InlineQueryResult`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InlineQueryResultType {
    /// An audio file result.
    #[serde(rename = "audio")]
    Audio,
    /// A general document (file) result.
    #[serde(rename = "document")]
    Document,
    /// A GIF animation result.
    #[serde(rename = "gif")]
    Gif,
    /// An MPEG-4 GIF animation result.
    #[serde(rename = "mpeg4_gif")]
    Mpeg4Gif,
    /// A photo result.
    #[serde(rename = "photo")]
    Photo,
    /// A sticker result.
    #[serde(rename = "sticker")]
    Sticker,
    /// A video result.
    #[serde(rename = "video")]
    Video,
    /// A voice message result.
    #[serde(rename = "voice")]
    Voice,
    /// An article result (text content with optional URL).
    #[serde(rename = "article")]
    Article,
    /// A contact result.
    #[serde(rename = "contact")]
    Contact,
    /// A game result.
    #[serde(rename = "game")]
    Game,
    /// A location result.
    #[serde(rename = "location")]
    Location,
    /// A venue result.
    #[serde(rename = "venue")]
    Venue,
}

/// Available types of `InputMedia`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum InputMediaType {
    /// An animation (GIF or H.264/MPEG-4 AVC video without sound).
    Animation,
    /// A general file to be sent as a document.
    Document,
    /// An audio file to be treated as music.
    Audio,
    /// A photo.
    Photo,
    /// A video.
    Video,
}

/// Available types of `InputPaidMedia`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum InputPaidMediaType {
    /// A paid photo.
    Photo,
    /// A paid video.
    Video,
}

/// Available types of `InputProfilePhoto`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum InputProfilePhotoType {
    /// A static profile photo (JPEG).
    Static,
    /// An animated profile photo (MPEG-4).
    Animated,
}

/// Available types of `InputStoryContent`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum InputStoryContentType {
    /// A photo story.
    Photo,
    /// A video story.
    Video,
}

/// Available button styles for `InlineKeyboardButton` and `KeyboardButton`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum KeyboardButtonStyle {
    /// Primary (default blue) button style.
    Primary,
    /// Success (green) button style.
    Success,
    /// Danger (red) button style.
    Danger,
}

/// Available positions for `MaskPosition`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MaskPositionPoint {
    /// The mask is placed on the forehead.
    Forehead,
    /// The mask is placed on the eyes.
    Eyes,
    /// The mask is placed on the mouth.
    Mouth,
    /// The mask is placed on the chin.
    Chin,
}

/// Available types of `MenuButton`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MenuButtonType {
    /// A button that opens the list of bot commands.
    #[serde(rename = "commands")]
    Commands,
    /// A button that launches a Web App.
    #[serde(rename = "web_app")]
    WebApp,
    /// No specific menu button is set; the default behavior applies.
    #[serde(rename = "default")]
    Default,
}

/// Available types of `Message` that can be seen as attachment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MessageAttachmentType {
    /// An animation (GIF or H.264/MPEG-4 AVC video without sound).
    Animation,
    /// An audio file.
    Audio,
    /// A shared contact.
    Contact,
    /// A dice with a random value.
    Dice,
    /// A general file (document).
    Document,
    /// A game.
    Game,
    /// An invoice for a payment.
    Invoice,
    /// A shared location.
    Location,
    /// Paid media content.
    PaidMedia,
    /// Telegram Passport data.
    PassportData,
    /// A photo.
    Photo,
    /// A native poll.
    Poll,
    /// A sticker.
    Sticker,
    /// A forwarded story.
    Story,
    /// A successful payment notification.
    SuccessfulPayment,
    /// A video file.
    Video,
    /// A video note (rounded video message).
    VideoNote,
    /// A voice message.
    Voice,
    /// A venue.
    Venue,
}

/// Available types of `MessageEntity`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MessageEntityType {
    /// A block quotation.
    Blockquote,
    /// Bold text.
    Bold,
    /// A bot command (e.g. `/start`).
    BotCommand,
    /// A cashtag (e.g. `$USD`).
    Cashtag,
    /// Monowidth inline code.
    Code,
    /// A custom emoji specified by its unique identifier.
    CustomEmoji,
    /// A date/time entity with optional formatting.
    DateTime,
    /// An email address.
    Email,
    /// An expandable block quotation.
    ExpandableBlockquote,
    /// A hashtag (e.g. `#hashtag`).
    Hashtag,
    /// Italic text.
    Italic,
    /// An `@username` mention.
    Mention,
    /// A phone number.
    PhoneNumber,
    /// Monowidth code block (optionally with a language).
    Pre,
    /// A spoiler (hidden until tapped).
    Spoiler,
    /// Strikethrough text.
    Strikethrough,
    /// A clickable text URL.
    TextLink,
    /// A mention of a user without a username.
    TextMention,
    /// Underlined text.
    Underline,
    /// A URL (e.g. `https://telegram.org`).
    Url,
}

/// All possible formats for `MessageEntity.date_time_format`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MessageEntityDateTimeFormats {
    /// Relative time (e.g. "in 5 minutes").
    #[serde(rename = "r")]
    Relative,
    /// Localized weekday name.
    #[serde(rename = "w")]
    LocalizedWeekday,
    /// Short date format.
    #[serde(rename = "d")]
    ShortDate,
    /// Long date format.
    #[serde(rename = "D")]
    LongDate,
    /// Short time format.
    #[serde(rename = "t")]
    ShortTime,
    /// Long time format.
    #[serde(rename = "T")]
    LongTime,
    /// Localized weekday with short date.
    #[serde(rename = "wd")]
    LocalizedWeekdayShortDate,
    /// Localized weekday with long date.
    #[serde(rename = "wD")]
    LocalizedWeekdayLongDate,
    /// Localized weekday with short time.
    #[serde(rename = "wt")]
    LocalizedWeekdayShortTime,
    /// Localized weekday with long time.
    #[serde(rename = "wT")]
    LocalizedWeekdayLongTime,
    /// Localized weekday with short date and short time.
    #[serde(rename = "wdt")]
    LocalizedWeekdayShortDateShortTime,
    /// Localized weekday with short date and long time.
    #[serde(rename = "wdT")]
    LocalizedWeekdayShortDateLongTime,
    /// Localized weekday with long date and short time.
    #[serde(rename = "wDt")]
    LocalizedWeekdayLongDateShortTime,
    /// Localized weekday with long date and long time.
    #[serde(rename = "wDT")]
    LocalizedWeekdayLongDateLongTime,
    /// Short date with short time.
    #[serde(rename = "dt")]
    ShortDateShortTime,
    /// Short date with long time.
    #[serde(rename = "dT")]
    ShortDateLongTime,
    /// Long date with short time.
    #[serde(rename = "Dt")]
    LongDateShortTime,
    /// Long date with long time.
    #[serde(rename = "DT")]
    LongDateLongTime,
}

/// Available types of `MessageOrigin`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MessageOriginType {
    /// The message was originally sent by a known user.
    User,
    /// The message was originally sent by an unknown user.
    HiddenUser,
    /// The message was originally sent on behalf of a chat.
    Chat,
    /// The message was originally sent to a channel.
    Channel,
}

/// Available types of `Message`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MessageType {
    /// An animation (GIF or H.264/MPEG-4 AVC video without sound).
    Animation,
    /// An audio file.
    Audio,
    /// Service message: a user boosted the chat.
    BoostAdded,
    /// The message was sent via a business connection.
    BusinessConnectionId,
    /// Service message: channel chat was created.
    ChannelChatCreated,
    /// Service message: chat background was set.
    ChatBackgroundSet,
    /// Service message: the chat owner has changed.
    ChatOwnerChanged,
    /// Service message: the chat owner has left.
    ChatOwnerLeft,
    /// Service message: a chat was shared with the bot.
    ChatShared,
    /// A checklist message.
    Checklist,
    /// Service message: tasks were added to a checklist.
    ChecklistTasksAdded,
    /// Service message: checklist tasks were marked done.
    ChecklistTasksDone,
    /// Service message: a website was connected via the bot.
    ConnectedWebsite,
    /// A shared contact.
    Contact,
    /// Service message: the chat photo was deleted.
    DeleteChatPhoto,
    /// A dice with a random value.
    Dice,
    /// Service message: the direct message price was changed.
    DirectMessagePriceChanged,
    /// A general file (document).
    Document,
    /// The message contains a visual effect.
    EffectId,
    /// Service message: a forum topic was created.
    ForumTopicCreated,
    /// Service message: a forum topic was closed.
    ForumTopicClosed,
    /// Service message: a forum topic was edited.
    ForumTopicEdited,
    /// Service message: a forum topic was reopened.
    ForumTopicReopened,
    /// A game.
    Game,
    /// Service message: the general forum topic was hidden.
    GeneralForumTopicHidden,
    /// Service message: the general forum topic was unhidden.
    GeneralForumTopicUnhidden,
    /// A gift sent in the message.
    Gift,
    /// Service message: a gift upgrade was sent.
    GiftUpgradeSent,
    /// A scheduled giveaway message.
    Giveaway,
    /// Service message: a giveaway was created.
    GiveawayCreated,
    /// Service message: giveaway winners were selected.
    GiveawayWinners,
    /// Service message: a giveaway was completed.
    GiveawayCompleted,
    /// Service message: a basic group was created.
    GroupChatCreated,
    /// An invoice for a payment.
    Invoice,
    /// Service message: a member left the chat.
    LeftChatMember,
    /// A shared location.
    Location,
    /// Service message: the auto-delete timer was changed.
    MessageAutoDeleteTimerChanged,
    /// Service message: the group was migrated to a supergroup.
    MigrateToChatId,
    /// Service message: new members joined the chat.
    NewChatMembers,
    /// Service message: the chat title was changed.
    NewChatTitle,
    /// Service message: the chat photo was changed.
    NewChatPhoto,
    /// Paid media content.
    PaidMedia,
    /// Service message: the paid message price was changed.
    PaidMessagePriceChanged,
    /// Service message: a suggested post approval failed.
    SuggestedPostApprovalFailed,
    /// Service message: a suggested post was approved.
    SuggestedPostApproved,
    /// Service message: a suggested post was declined.
    SuggestedPostDeclined,
    /// Service message: information about a suggested post.
    SuggestedPostInfo,
    /// Service message: a suggested post was paid for.
    SuggestedPostPaid,
    /// Service message: a suggested post was refunded.
    SuggestedPostRefunded,
    /// Telegram Passport data.
    PassportData,
    /// A photo.
    Photo,
    /// Service message: a message was pinned.
    PinnedMessage,
    /// A native poll.
    Poll,
    /// Service message: a proximity alert was triggered.
    ProximityAlertTriggered,
    /// Service message: a refunded payment.
    RefundedPayment,
    /// The message is a reply to a story.
    ReplyToStory,
    /// The sender's boost count in the chat.
    SenderBoostCount,
    /// The business bot that sent this message.
    SenderBusinessBot,
    /// A sticker.
    Sticker,
    /// A forwarded story.
    Story,
    /// Service message: a supergroup was created.
    SupergroupChatCreated,
    /// A successful payment notification.
    SuccessfulPayment,
    /// A text message.
    Text,
    /// A unique gift sent in the message.
    UniqueGift,
    /// Service message: users were shared with the bot.
    UsersShared,
    /// A venue.
    Venue,
    /// A video file.
    Video,
    /// Service message: a video chat was scheduled.
    VideoChatScheduled,
    /// Service message: a video chat was started.
    VideoChatStarted,
    /// Service message: a video chat has ended.
    VideoChatEnded,
    /// Service message: new participants were invited to a video chat.
    VideoChatParticipantsInvited,
    /// A video note (rounded video message).
    VideoNote,
    /// A voice message.
    Voice,
    /// Data sent from a Web App.
    WebAppData,
    /// Service message: a user was granted write access.
    WriteAccessAllowed,
}

/// Available types of `OwnedGift`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum OwnedGiftType {
    /// A regular gift.
    Regular,
    /// A unique (upgraded) gift.
    Unique,
}

/// Available types of `PaidMedia`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PaidMediaType {
    /// A preview of paid media (before purchase).
    Preview,
    /// A paid video.
    Video,
    /// A paid photo.
    Photo,
}

/// Available parse modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ParseMode {
    /// HTML-style formatting.
    #[serde(rename = "HTML")]
    Html,
    /// MarkdownV2-style formatting.
    #[serde(rename = "MarkdownV2")]
    MarkdownV2,
    /// Legacy Markdown formatting (use MarkdownV2 for new code).
    #[serde(rename = "Markdown")]
    Markdown,
}

/// Available types for `Poll`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PollType {
    /// A regular poll allowing one or multiple answers.
    Regular,
    /// A quiz poll with exactly one correct answer.
    Quiz,
}

/// Available types of `ReactionType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ReactionType {
    /// A reaction based on a standard Unicode emoji.
    Emoji,
    /// A reaction based on a custom emoji.
    CustomEmoji,
    /// A paid reaction (star reaction).
    Paid,
}

/// Available types of `RevenueWithdrawalState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RevenueWithdrawalStateType {
    /// The withdrawal is pending.
    Pending,
    /// The withdrawal has succeeded.
    Succeeded,
    /// The withdrawal has failed.
    Failed,
}

/// Available formats of `Sticker` in the set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum StickerFormat {
    /// A static `.WEBP` or `.PNG` sticker.
    Static,
    /// An animated `.TGS` sticker.
    Animated,
    /// A video `.WEBM` sticker.
    Video,
}

/// Available types of `Sticker`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum StickerType {
    /// A regular sticker.
    Regular,
    /// A mask sticker positioned on faces in photos.
    Mask,
    /// A custom emoji sticker usable as a custom emoji.
    CustomEmoji,
}

/// Available types of `StoryAreaType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum StoryAreaTypeType {
    /// A location area in the story.
    Location,
    /// A suggested reaction area in the story.
    SuggestedReaction,
    /// A clickable link area in the story.
    Link,
    /// A weather widget area in the story.
    Weather,
    /// A unique gift area in the story.
    UniqueGift,
}

/// Available states of `SuggestedPostInfo.state`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SuggestedPostInfoState {
    /// The suggested post is pending review.
    Pending,
    /// The suggested post has been approved.
    Approved,
    /// The suggested post has been declined.
    Declined,
}

/// Available refund reasons for `SuggestedPostRefunded`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SuggestedPostRefunded {
    /// The post was deleted, triggering a refund.
    PostDeleted,
    /// The payment was refunded.
    PaymentRefunded,
}

/// Available types of `TransactionPartner`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TransactionPartnerType {
    /// Transaction with an affiliate program.
    AffiliateProgram,
    /// Transaction with a chat (channel).
    Chat,
    /// Withdrawal to the Fragment platform.
    Fragment,
    /// Transaction with an unknown partner.
    Other,
    /// Transaction with Telegram Ads.
    TelegramAds,
    /// Transaction with the Telegram Bot API (e.g. paid broadcasts).
    TelegramApi,
    /// Transaction with a user.
    User,
}

/// Constants for `TransactionPartnerUser.transaction_type`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TransactionPartnerUser {
    /// Payment for a bot invoice.
    InvoicePayment,
    /// Payment for paid media.
    PaidMediaPayment,
    /// A gift purchase.
    GiftPurchase,
    /// A Telegram Premium subscription purchase.
    PremiumPurchase,
    /// A business account transfer.
    BusinessAccountTransfer,
}

/// Available origins for `UniqueGiftInfo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum UniqueGiftInfoOrigin {
    /// Obtained via a gifted upgrade.
    #[serde(rename = "gifted_upgrade")]
    GiftedUpgrade,
    /// Obtained via an offer.
    #[serde(rename = "OFFER")]
    Offer,
    /// Obtained via resale.
    #[serde(rename = "resale")]
    Resale,
    /// Obtained via a transfer.
    #[serde(rename = "transfer")]
    Transfer,
    /// Obtained via an upgrade.
    #[serde(rename = "upgrade")]
    Upgrade,
}

/// Available rarities for `UniqueGiftModel`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum UniqueGiftModelRarity {
    /// Uncommon rarity.
    Uncommon,
    /// Rare rarity.
    Rare,
    /// Epic rarity.
    Epic,
    /// Legendary rarity.
    Legendary,
}

/// Available types of `Update`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum UpdateType {
    /// A new incoming message of any kind.
    Message,
    /// An edited message.
    EditedMessage,
    /// A new incoming channel post of any kind.
    ChannelPost,
    /// An edited channel post.
    EditedChannelPost,
    /// A new incoming inline query.
    InlineQuery,
    /// A result of an inline query that was chosen by the user.
    ChosenInlineResult,
    /// A new incoming callback query.
    CallbackQuery,
    /// A new incoming shipping query (payments only).
    ShippingQuery,
    /// A new incoming pre-checkout query (payments only).
    PreCheckoutQuery,
    /// A new poll state (stopped or answer option counts changed).
    Poll,
    /// A user changed their answer in a non-anonymous poll.
    PollAnswer,
    /// The bot's chat member status was updated.
    MyChatMember,
    /// A chat member's status was updated.
    ChatMember,
    /// A request to join the chat has been sent.
    ChatJoinRequest,
    /// A chat boost was added or changed.
    ChatBoost,
    /// A chat boost was removed.
    RemovedChatBoost,
    /// A reaction to a message was changed by a user.
    MessageReaction,
    /// Anonymous reaction counts on a message were updated.
    MessageReactionCount,
    /// A business connection update.
    BusinessConnection,
    /// A new message from a connected business account.
    BusinessMessage,
    /// An edited business message.
    EditedBusinessMessage,
    /// Messages were deleted from a connected business account.
    DeletedBusinessMessages,
    /// A user purchased paid media from the bot.
    PurchasedPaidMedia,
}

/// Available emojis of `ReactionTypeEmoji`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ReactionEmoji {
    /// Thumbs up reaction.
    #[serde(rename = "\u{1F44D}")]
    ThumbsUp,
    /// Thumbs down reaction.
    #[serde(rename = "\u{1F44E}")]
    ThumbsDown,
    /// Red heart reaction.
    #[serde(rename = "\u{2764}")]
    RedHeart,
    /// Fire reaction.
    #[serde(rename = "\u{1F525}")]
    Fire,
    /// Smiling face with hearts reaction.
    #[serde(rename = "\u{1F970}")]
    SmilingFaceWithHearts,
    /// Clapping hands reaction.
    #[serde(rename = "\u{1F44F}")]
    ClappingHands,
    /// Grinning face with smiling eyes reaction.
    #[serde(rename = "\u{1F601}")]
    GrinningFaceWithSmilingEyes,
    /// Thinking face reaction.
    #[serde(rename = "\u{1F914}")]
    ThinkingFace,
    /// Shocked face with exploding head reaction.
    #[serde(rename = "\u{1F92F}")]
    ShockedFaceWithExplodingHead,
    /// Face screaming in fear reaction.
    #[serde(rename = "\u{1F631}")]
    FaceScreamingInFear,
    /// Serious face with symbols covering mouth reaction.
    #[serde(rename = "\u{1F92C}")]
    SeriousFaceWithSymbolsCoveringMouth,
    /// Crying face reaction.
    #[serde(rename = "\u{1F622}")]
    CryingFace,
    /// Party popper reaction.
    #[serde(rename = "\u{1F389}")]
    PartyPopper,
    /// Grinning face with star eyes reaction.
    #[serde(rename = "\u{1F929}")]
    GrinningFaceWithStarEyes,
    /// Face with open mouth vomiting reaction.
    #[serde(rename = "\u{1F92E}")]
    FaceWithOpenMouthVomiting,
    /// Pile of poo reaction.
    #[serde(rename = "\u{1F4A9}")]
    PileOfPoo,
    /// Person with folded hands (prayer) reaction.
    #[serde(rename = "\u{1F64F}")]
    PersonWithFoldedHands,
    /// OK hand sign reaction.
    #[serde(rename = "\u{1F44C}")]
    OkHandSign,
    /// Dove of peace reaction.
    #[serde(rename = "\u{1F54A}")]
    DoveOfPeace,
    /// Clown face reaction.
    #[serde(rename = "\u{1F921}")]
    ClownFace,
    /// Yawning face reaction.
    #[serde(rename = "\u{1F971}")]
    YawningFace,
    /// Face with uneven eyes and wavy mouth reaction.
    #[serde(rename = "\u{1F974}")]
    FaceWithUnevenEyesAndWavyMouth,
    /// Smiling face with heart-shaped eyes reaction.
    #[serde(rename = "\u{1F60D}")]
    SmilingFaceWithHeartShapedEyes,
    /// Spouting whale reaction.
    #[serde(rename = "\u{1F433}")]
    SpoutingWhale,
    /// Heart on fire reaction.
    #[serde(rename = "\u{2764}\u{FE0F}\u{200D}\u{1F525}")]
    HeartOnFire,
    /// New moon with face reaction.
    #[serde(rename = "\u{1F31A}")]
    NewMoonWithFace,
    /// Hot dog reaction.
    #[serde(rename = "\u{1F32D}")]
    HotDog,
    /// Hundred points symbol reaction.
    #[serde(rename = "\u{1F4AF}")]
    HundredPointsSymbol,
    /// Rolling on the floor laughing reaction.
    #[serde(rename = "\u{1F923}")]
    RollingOnTheFloorLaughing,
    /// High voltage sign reaction.
    #[serde(rename = "\u{26A1}")]
    HighVoltageSign,
    /// Banana reaction.
    #[serde(rename = "\u{1F34C}")]
    Banana,
    /// Trophy reaction.
    #[serde(rename = "\u{1F3C6}")]
    Trophy,
    /// Broken heart reaction.
    #[serde(rename = "\u{1F494}")]
    BrokenHeart,
    /// Face with one eyebrow raised reaction.
    #[serde(rename = "\u{1F928}")]
    FaceWithOneEyebrowRaised,
    /// Neutral face reaction.
    #[serde(rename = "\u{1F610}")]
    NeutralFace,
    /// Strawberry reaction.
    #[serde(rename = "\u{1F353}")]
    Strawberry,
    /// Bottle with popping cork reaction.
    #[serde(rename = "\u{1F37E}")]
    BottleWithPoppingCork,
    /// Kiss mark reaction.
    #[serde(rename = "\u{1F48B}")]
    KissMark,
    /// Middle finger reaction.
    #[serde(rename = "\u{1F595}")]
    ReversedHandWithMiddleFingerExtended,
    /// Smiling face with horns reaction.
    #[serde(rename = "\u{1F608}")]
    SmilingFaceWithHorns,
    /// Sleeping face reaction.
    #[serde(rename = "\u{1F634}")]
    SleepingFace,
    /// Loudly crying face reaction.
    #[serde(rename = "\u{1F62D}")]
    LoudlyCryingFace,
    /// Nerd face reaction.
    #[serde(rename = "\u{1F913}")]
    NerdFace,
    /// Ghost reaction.
    #[serde(rename = "\u{1F47B}")]
    Ghost,
    /// Man technologist reaction.
    #[serde(rename = "\u{1F468}\u{200D}\u{1F4BB}")]
    ManTechnologist,
    /// Eyes reaction.
    #[serde(rename = "\u{1F440}")]
    Eyes,
    /// Jack-o-lantern reaction.
    #[serde(rename = "\u{1F383}")]
    JackOLantern,
    /// See-no-evil monkey reaction.
    #[serde(rename = "\u{1F648}")]
    SeeNoEvilMonkey,
    /// Smiling face with halo reaction.
    #[serde(rename = "\u{1F607}")]
    SmilingFaceWithHalo,
    /// Fearful face reaction.
    #[serde(rename = "\u{1F628}")]
    FearfulFace,
    /// Handshake reaction.
    #[serde(rename = "\u{1F91D}")]
    Handshake,
    /// Writing hand reaction.
    #[serde(rename = "\u{270D}")]
    WritingHand,
    /// Hugging face reaction.
    #[serde(rename = "\u{1F917}")]
    HuggingFace,
    /// Saluting face reaction.
    #[serde(rename = "\u{1FAE1}")]
    SalutingFace,
    /// Father Christmas (Santa Claus) reaction.
    #[serde(rename = "\u{1F385}")]
    FatherChristmas,
    /// Christmas tree reaction.
    #[serde(rename = "\u{1F384}")]
    ChristmasTree,
    /// Snowman reaction.
    #[serde(rename = "\u{2603}")]
    Snowman,
    /// Nail polish reaction.
    #[serde(rename = "\u{1F485}")]
    NailPolish,
    /// Zany face (grinning with one large and one small eye) reaction.
    #[serde(rename = "\u{1F92A}")]
    GrinningFaceWithOneLargeAndOneSmallEye,
    /// Moyai (Easter Island statue) reaction.
    #[serde(rename = "\u{1F5FF}")]
    Moyai,
    /// Squared COOL reaction.
    #[serde(rename = "\u{1F192}")]
    SquaredCool,
    /// Heart with arrow reaction.
    #[serde(rename = "\u{1F498}")]
    HeartWithArrow,
    /// Hear-no-evil monkey reaction.
    #[serde(rename = "\u{1F649}")]
    HearNoEvilMonkey,
    /// Unicorn face reaction.
    #[serde(rename = "\u{1F984}")]
    UnicornFace,
    /// Face throwing a kiss reaction.
    #[serde(rename = "\u{1F618}")]
    FaceThrowingAKiss,
    /// Pill reaction.
    #[serde(rename = "\u{1F48A}")]
    Pill,
    /// Speak-no-evil monkey reaction.
    #[serde(rename = "\u{1F64A}")]
    SpeakNoEvilMonkey,
    /// Smiling face with sunglasses reaction.
    #[serde(rename = "\u{1F60E}")]
    SmilingFaceWithSunglasses,
    /// Alien monster reaction.
    #[serde(rename = "\u{1F47E}")]
    AlienMonster,
    /// Man shrugging reaction.
    #[serde(rename = "\u{1F937}\u{200D}\u{2642}\u{FE0F}")]
    ManShrugging,
    /// Person shrugging reaction.
    #[serde(rename = "\u{1F937}")]
    Shrug,
    /// Woman shrugging reaction.
    #[serde(rename = "\u{1F937}\u{200D}\u{2640}\u{FE0F}")]
    WomanShrugging,
    /// Pouting face reaction.
    #[serde(rename = "\u{1F621}")]
    PoutingFace,
}

// ── Display / From / PartialEq implementations ─────────────────────────────
//
// These trait implementations let the typed constant enums interoperate
// seamlessly with the `String` / `&str` fields used throughout the API types
// and builder methods.  In particular:
//
// - `Display` / `From<Enum> for String` lets you pass enum values directly
//   into builder methods that accept `impl Into<String>`.
// - `PartialEq<Enum> for String` (and `&str`) lets you compare deserialized
//   JSON string fields against the typed constants without `.as_str()`.

// ── helpers ──────────────────────────────────────────────────────────────────

/// Generate `as_str()`, `Display`, `From<$Enum> for String`,
/// `PartialEq<$Enum> for String`, `PartialEq<$Enum> for str`,
/// `PartialEq<String> for $Enum`, and `PartialEq<&str> for $Enum`
/// for a `snake_case` serde enum.
macro_rules! impl_str_traits_snake {
    ($Enum:ident { $( $Variant:ident => $wire:expr ),+ $(,)? }) => {
        impl $Enum {
            /// Returns the wire-format string representation of this variant.
            pub fn as_str(&self) -> &'static str {
                match self {
                    $( Self::$Variant => $wire, )+
                }
            }
        }

        impl fmt::Display for $Enum {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl From<$Enum> for String {
            fn from(val: $Enum) -> Self { val.as_str().to_owned() }
        }

        impl PartialEq<$Enum> for String {
            fn eq(&self, other: &$Enum) -> bool { self.as_str() == other.as_str() }
        }

        impl PartialEq<$Enum> for str {
            fn eq(&self, other: &$Enum) -> bool { self == other.as_str() }
        }

        impl PartialEq<String> for $Enum {
            fn eq(&self, other: &String) -> bool { self.as_str() == other.as_str() }
        }

        impl PartialEq<&str> for $Enum {
            fn eq(&self, other: &&str) -> bool { self.as_str() == *other }
        }
    };
}

// ── ParseMode ────────────────────────────────────────────────────────────────

impl_str_traits_snake!(ParseMode {
    Html       => "HTML",
    MarkdownV2 => "MarkdownV2",
    Markdown   => "Markdown",
});

// ── ChatAction ───────────────────────────────────────────────────────────────

impl_str_traits_snake!(ChatAction {
    ChooseSticker   => "choose_sticker",
    FindLocation    => "find_location",
    RecordVoice     => "record_voice",
    RecordVideo     => "record_video",
    RecordVideoNote => "record_video_note",
    Typing          => "typing",
    UploadVoice     => "upload_voice",
    UploadDocument  => "upload_document",
    UploadPhoto     => "upload_photo",
    UploadVideo     => "upload_video",
    UploadVideoNote => "upload_video_note",
});

// ── ChatMemberStatus ─────────────────────────────────────────────────────────

impl_str_traits_snake!(ChatMemberStatus {
    Administrator => "administrator",
    Owner         => "creator",
    Banned        => "kicked",
    Left          => "left",
    Member        => "member",
    Restricted    => "restricted",
});

// ── ChatType ─────────────────────────────────────────────────────────────────

impl_str_traits_snake!(ChatType {
    Sender     => "sender",
    Private    => "private",
    Group      => "group",
    Supergroup => "supergroup",
    Channel    => "channel",
});

// ── MessageEntityType ────────────────────────────────────────────────────────

impl_str_traits_snake!(MessageEntityType {
    Blockquote           => "blockquote",
    Bold                 => "bold",
    BotCommand           => "bot_command",
    Cashtag              => "cashtag",
    Code                 => "code",
    CustomEmoji          => "custom_emoji",
    DateTime             => "date_time",
    Email                => "email",
    ExpandableBlockquote => "expandable_blockquote",
    Hashtag              => "hashtag",
    Italic               => "italic",
    Mention              => "mention",
    PhoneNumber          => "phone_number",
    Pre                  => "pre",
    Spoiler              => "spoiler",
    Strikethrough        => "strikethrough",
    TextLink             => "text_link",
    TextMention          => "text_mention",
    Underline            => "underline",
    Url                  => "url",
});

// ── Additional enums with custom serde renames ───────────────────────────────

impl_str_traits_snake!(BackgroundTypeType {
    Fill      => "fill",
    Wallpaper => "wallpaper",
    Pattern   => "pattern",
    ChatTheme => "chat_theme",
});

impl_str_traits_snake!(BackgroundFillType {
    Solid            => "solid",
    Gradient         => "gradient",
    FreeformGradient => "freeform_gradient",
});

impl_str_traits_snake!(BotCommandScopeType {
    Default              => "default",
    AllPrivateChats      => "all_private_chats",
    AllGroupChats        => "all_group_chats",
    AllChatAdministrators => "all_chat_administrators",
    Chat                 => "chat",
    ChatAdministrators   => "chat_administrators",
    ChatMember           => "chat_member",
});

impl_str_traits_snake!(ChatBoostSources {
    GiftCode => "gift_code",
    Giveaway => "giveaway",
    Premium  => "premium",
});

impl_str_traits_snake!(InputMediaType {
    Animation => "animation",
    Document  => "document",
    Audio     => "audio",
    Photo     => "photo",
    Video     => "video",
});

impl_str_traits_snake!(InputPaidMediaType {
    Photo => "photo",
    Video => "video",
});

impl_str_traits_snake!(InputProfilePhotoType {
    Static   => "static",
    Animated => "animated",
});

impl_str_traits_snake!(InputStoryContentType {
    Photo => "photo",
    Video => "video",
});

impl_str_traits_snake!(KeyboardButtonStyle {
    Primary => "primary",
    Success => "success",
    Danger  => "danger",
});

impl_str_traits_snake!(MaskPositionPoint {
    Forehead => "forehead",
    Eyes     => "eyes",
    Mouth    => "mouth",
    Chin     => "chin",
});

impl_str_traits_snake!(MenuButtonType {
    Commands => "commands",
    WebApp   => "web_app",
    Default  => "default",
});

impl_str_traits_snake!(MessageAttachmentType {
    Animation         => "animation",
    Audio             => "audio",
    Contact           => "contact",
    Dice              => "dice",
    Document          => "document",
    Game              => "game",
    Invoice           => "invoice",
    Location          => "location",
    PaidMedia         => "paid_media",
    PassportData      => "passport_data",
    Photo             => "photo",
    Poll              => "poll",
    Sticker           => "sticker",
    Story             => "story",
    SuccessfulPayment => "successful_payment",
    Video             => "video",
    VideoNote         => "video_note",
    Voice             => "voice",
    Venue             => "venue",
});

impl_str_traits_snake!(MessageOriginType {
    User       => "user",
    HiddenUser => "hidden_user",
    Chat       => "chat",
    Channel    => "channel",
});

impl_str_traits_snake!(PaidMediaType {
    Preview => "preview",
    Video   => "video",
    Photo   => "photo",
});

impl_str_traits_snake!(PollType {
    Regular => "regular",
    Quiz    => "quiz",
});

impl_str_traits_snake!(ReactionType {
    Emoji       => "emoji",
    CustomEmoji => "custom_emoji",
    Paid        => "paid",
});

impl_str_traits_snake!(RevenueWithdrawalStateType {
    Pending   => "pending",
    Succeeded => "succeeded",
    Failed    => "failed",
});

impl_str_traits_snake!(StickerFormat {
    Static   => "static",
    Animated => "animated",
    Video    => "video",
});

impl_str_traits_snake!(StickerType {
    Regular     => "regular",
    Mask        => "mask",
    CustomEmoji => "custom_emoji",
});

impl_str_traits_snake!(StoryAreaTypeType {
    Location          => "location",
    SuggestedReaction => "suggested_reaction",
    Link              => "link",
    Weather           => "weather",
    UniqueGift        => "unique_gift",
});

impl_str_traits_snake!(SuggestedPostInfoState {
    Pending  => "pending",
    Approved => "approved",
    Declined => "declined",
});

impl_str_traits_snake!(SuggestedPostRefunded {
    PostDeleted     => "post_deleted",
    PaymentRefunded => "payment_refunded",
});

impl_str_traits_snake!(TransactionPartnerType {
    AffiliateProgram => "affiliate_program",
    Chat             => "chat",
    Fragment         => "fragment",
    Other            => "other",
    TelegramAds      => "telegram_ads",
    TelegramApi      => "telegram_api",
    User             => "user",
});

impl_str_traits_snake!(TransactionPartnerUser {
    InvoicePayment          => "invoice_payment",
    PaidMediaPayment        => "paid_media_payment",
    GiftPurchase            => "gift_purchase",
    PremiumPurchase         => "premium_purchase",
    BusinessAccountTransfer => "business_account_transfer",
});

impl_str_traits_snake!(UniqueGiftInfoOrigin {
    GiftedUpgrade => "gifted_upgrade",
    Offer         => "OFFER",
    Resale        => "resale",
    Transfer      => "transfer",
    Upgrade       => "upgrade",
});

impl_str_traits_snake!(UniqueGiftModelRarity {
    Uncommon  => "uncommon",
    Rare      => "rare",
    Epic      => "epic",
    Legendary => "legendary",
});

impl_str_traits_snake!(OwnedGiftType {
    Regular => "regular",
    Unique  => "unique",
});

// ── Integer / Limit constants ────────────────────────────────────────────────

/// Limitations for `BotCommand` and `Bot::set_my_commands`.
pub mod bot_command_limit {
    /// Minimum length of a bot command string.
    pub const MIN_COMMAND: u32 = 1;
    /// Maximum length of a bot command string.
    pub const MAX_COMMAND: u32 = 32;
    /// Minimum length of a bot command description.
    pub const MIN_DESCRIPTION: u32 = 1;
    /// Maximum length of a bot command description.
    pub const MAX_DESCRIPTION: u32 = 256;
    /// Maximum number of commands in a single `set_my_commands` call.
    pub const MAX_COMMAND_NUMBER: u32 = 100;
}

/// Limitations for `Bot::set_my_description` and `Bot::set_my_short_description`.
pub mod bot_description_limit {
    /// Maximum length of the bot description.
    pub const MAX_DESCRIPTION_LENGTH: u32 = 512;
    /// Maximum length of the bot short description.
    pub const MAX_SHORT_DESCRIPTION_LENGTH: u32 = 120;
}

/// Limitations for `Bot::set_my_name`.
pub mod bot_name_limit {
    /// Maximum length of the bot name.
    pub const MAX_NAME_LENGTH: u32 = 64;
}

/// Limitations for `Bot::delete_messages`, `Bot::forward_messages`, and `Bot::copy_messages`.
pub mod bulk_request_limit {
    /// Minimum number of messages per bulk request.
    pub const MIN_LIMIT: u32 = 1;
    /// Maximum number of messages per bulk request.
    pub const MAX_LIMIT: u32 = 100;
}

/// Limitations related to handling business accounts.
pub mod business_limit {
    /// 24 hours in seconds.
    pub const CHAT_ACTIVITY_TIMEOUT: u32 = 86400;
    /// Minimum length of a business account name.
    pub const MIN_NAME_LENGTH: u32 = 1;
    /// Maximum length of a business account name.
    pub const MAX_NAME_LENGTH: u32 = 64;
    /// Maximum length of a business account username.
    pub const MAX_USERNAME_LENGTH: u32 = 32;
    /// Maximum length of a business account bio.
    pub const MAX_BIO_LENGTH: u32 = 140;
    /// Minimum number of gift results to request.
    pub const MIN_GIFT_RESULTS: u32 = 1;
    /// Maximum number of gift results to request.
    pub const MAX_GIFT_RESULTS: u32 = 100;
    /// Minimum star count for business operations.
    pub const MIN_STAR_COUNT: u32 = 1;
    /// Maximum star count for business operations.
    pub const MAX_STAR_COUNT: u32 = 10000;
}

/// Limitations for `CallbackQuery` / `Bot::answer_callback_query`.
pub mod callback_query_limit {
    /// Maximum length of the `answer_callback_query` text parameter.
    pub const ANSWER_CALLBACK_QUERY_TEXT_LENGTH: u32 = 200;
}

/// Special chat IDs.
pub mod chat_id {
    /// Chat ID of the anonymous admin sender.
    pub const ANONYMOUS_ADMIN: i64 = 1_087_968_824;
    /// Chat ID of the Telegram service notifications chat.
    pub const SERVICE_CHAT: i64 = 777_000;
    /// Chat ID of the fake channel sender used by Telegram.
    pub const FAKE_CHANNEL: i64 = 136_817_688;
}

/// Limitations for `ChatInviteLink`.
pub mod chat_invite_link_limit {
    /// Minimum member limit for an invite link.
    pub const MIN_MEMBER_LIMIT: u32 = 1;
    /// Maximum member limit for an invite link.
    pub const MAX_MEMBER_LIMIT: u32 = 99999;
    /// Maximum length of an invite link name.
    pub const NAME_LENGTH: u32 = 32;
}

/// Limitations for chat title, description, and admin custom title.
pub mod chat_limit {
    /// Maximum length of a chat administrator custom title.
    pub const CHAT_ADMINISTRATOR_CUSTOM_TITLE_LENGTH: u32 = 16;
    /// Maximum length of a chat description.
    pub const CHAT_DESCRIPTION_LENGTH: u32 = 255;
    /// Minimum length of a chat title.
    pub const MIN_CHAT_TITLE_LENGTH: u32 = 1;
    /// Maximum length of a chat title.
    pub const MAX_CHAT_TITLE_LENGTH: u32 = 128;
}

/// Limitations for chat subscription invite links.
pub mod chat_subscription_limit {
    /// 30 days in seconds.
    pub const SUBSCRIPTION_PERIOD: u32 = 2_592_000;
    /// Minimum subscription price in Telegram Stars.
    pub const MIN_PRICE: u32 = 1;
    /// Maximum subscription price in Telegram Stars.
    pub const MAX_PRICE: u32 = 10000;
}

/// Limitations for `ChatPhoto` sizes.
pub mod chat_photo_size {
    /// Small chat photo size in pixels.
    pub const SMALL: u32 = 160;
    /// Big chat photo size in pixels.
    pub const BIG: u32 = 640;
}

/// Limitations for `BackgroundType` subclasses.
pub mod background_type_limit {
    /// Maximum dimming percentage for background types.
    pub const MAX_DIMMING: u32 = 100;
    /// Maximum pattern intensity percentage.
    pub const MAX_INTENSITY: u32 = 100;
}

/// Limitations for `BackgroundFillGradient`.
pub mod background_fill_limit {
    /// Maximum rotation angle in degrees for a gradient fill.
    pub const MAX_ROTATION_ANGLE: u32 = 359;
}

/// Limitations for `Contact` vcard.
pub mod contact_limit {
    /// Maximum length of a vCard string.
    pub const VCARD: u32 = 2048;
}

/// Limitations for `Bot::get_custom_emoji_stickers`.
pub mod custom_emoji_sticker_limit {
    /// Maximum number of custom emoji identifiers per request.
    pub const CUSTOM_EMOJI_IDENTIFIER_LIMIT: u32 = 200;
}

/// Limitations for `Dice` value ranges.
pub mod dice_limit {
    /// Minimum dice value.
    pub const MIN_VALUE: u32 = 1;
    /// Maximum value for basketball dice.
    pub const MAX_VALUE_BASKETBALL: u32 = 5;
    /// Maximum value for bowling dice.
    pub const MAX_VALUE_BOWLING: u32 = 6;
    /// Maximum value for darts dice.
    pub const MAX_VALUE_DARTS: u32 = 6;
    /// Maximum value for standard dice.
    pub const MAX_VALUE_DICE: u32 = 6;
    /// Maximum value for football dice.
    pub const MAX_VALUE_FOOTBALL: u32 = 5;
    /// Maximum value for slot machine dice.
    pub const MAX_VALUE_SLOT_MACHINE: u32 = 64;
}

/// Limitations regarding file upload and download sizes.
pub mod file_size_limit {
    /// 20 MB.
    pub const FILESIZE_DOWNLOAD: u64 = 20_000_000;
    /// 50 MB.
    pub const FILESIZE_UPLOAD: u64 = 50_000_000;
    /// 2000 MB.
    pub const FILESIZE_UPLOAD_LOCAL_MODE: u64 = 2_000_000_000;
    /// Unlimited when using a local bot API server.
    pub const FILESIZE_DOWNLOAD_LOCAL_MODE: u64 = u64::MAX;
    /// 10 MB.
    pub const PHOTOSIZE_UPLOAD: u64 = 10_000_000;
    /// 1 MB.
    pub const VOICE_NOTE_FILE_SIZE: u64 = 1_000_000;
}

/// Limitations regarding flood limits.
pub mod flood_limit {
    /// Maximum messages per second to the same chat.
    pub const MESSAGES_PER_SECOND_PER_CHAT: u32 = 1;
    /// Maximum messages per second overall.
    pub const MESSAGES_PER_SECOND: u32 = 30;
    /// Maximum messages per minute per group chat.
    pub const MESSAGES_PER_MINUTE_PER_GROUP: u32 = 20;
    /// Maximum paid messages per second.
    pub const PAID_MESSAGES_PER_SECOND: u32 = 1000;
}

/// Available colors for `Bot::create_forum_topic.icon_color`.
pub mod forum_icon_color {
    /// Blue forum topic icon color.
    pub const BLUE: u32 = 0x6FB9F0;
    /// Yellow forum topic icon color.
    pub const YELLOW: u32 = 0xFFD67E;
    /// Purple forum topic icon color.
    pub const PURPLE: u32 = 0xCB86DB;
    /// Green forum topic icon color.
    pub const GREEN: u32 = 0x8EEE98;
    /// Pink forum topic icon color.
    pub const PINK: u32 = 0xFF93B2;
    /// Red forum topic icon color.
    pub const RED: u32 = 0xFB6F5F;
}

/// Limitations for `Bot::create_forum_topic` and `Bot::edit_forum_topic`.
pub mod forum_topic_limit {
    /// Minimum length of a forum topic name.
    pub const MIN_NAME_LENGTH: u32 = 1;
    /// Maximum length of a forum topic name.
    pub const MAX_NAME_LENGTH: u32 = 128;
}

/// Limitations for `Bot::send_gift`.
pub mod gift_limit {
    /// Maximum length of the gift text.
    pub const MAX_TEXT_LENGTH: u32 = 128;
}

/// Limitations for `Giveaway` and related classes.
pub mod giveaway_limit {
    /// Maximum number of giveaway winners.
    pub const MAX_WINNERS: u32 = 100;
}

/// Limitations for `InlineKeyboardButton`.
pub mod inline_keyboard_button_limit {
    /// Minimum length of callback data.
    pub const MIN_CALLBACK_DATA: u32 = 1;
    /// Maximum length of callback data.
    pub const MAX_CALLBACK_DATA: u32 = 64;
    /// Minimum length of copy text.
    pub const MIN_COPY_TEXT: u32 = 1;
    /// Maximum length of copy text.
    pub const MAX_COPY_TEXT: u32 = 256;
}

/// Limitations for `InlineKeyboardMarkup`.
pub mod inline_keyboard_markup_limit {
    /// Maximum total number of buttons in an inline keyboard.
    pub const TOTAL_BUTTON_NUMBER: u32 = 100;
    /// Maximum number of buttons per row.
    pub const BUTTONS_PER_ROW: u32 = 8;
}

/// Limitations for `InputChecklist` / `InputChecklistTask`.
pub mod input_checklist_limit {
    /// Minimum length of a checklist title.
    pub const MIN_TITLE_LENGTH: u32 = 1;
    /// Maximum length of a checklist title.
    pub const MAX_TITLE_LENGTH: u32 = 255;
    /// Minimum length of a checklist task text.
    pub const MIN_TEXT_LENGTH: u32 = 1;
    /// Maximum length of a checklist task text.
    pub const MAX_TEXT_LENGTH: u32 = 100;
    /// Minimum number of tasks in a checklist.
    pub const MIN_TASK_NUMBER: u32 = 1;
    /// Maximum number of tasks in a checklist.
    pub const MAX_TASK_NUMBER: u32 = 30;
}

/// Limitations for `InputStoryContentPhoto` / `InputStoryContentVideo`.
pub mod input_story_content_limit {
    /// 10 MB (same as `file_size_limit::PHOTOSIZE_UPLOAD`).
    pub const PHOTOSIZE_UPLOAD: u64 = 10_000_000;
    /// Maximum photo width in pixels for stories.
    pub const PHOTO_WIDTH: u32 = 1080;
    /// Maximum photo height in pixels for stories.
    pub const PHOTO_HEIGHT: u32 = 1920;
    /// 30 MB.
    pub const VIDEOSIZE_UPLOAD: u64 = 30_000_000;
    /// Maximum video width in pixels for stories.
    pub const VIDEO_WIDTH: u32 = 720;
    /// Maximum video height in pixels for stories.
    pub const VIDEO_HEIGHT: u32 = 1080;
    /// 60 seconds.
    pub const MAX_VIDEO_DURATION: u32 = 60;
}

/// Limitations for `InlineQuery` / `Bot::answer_inline_query`.
pub mod inline_query_limit {
    /// Maximum number of inline query results per response.
    pub const RESULTS: u32 = 50;
    /// Maximum length of the offset string.
    pub const MAX_OFFSET_LENGTH: u32 = 64;
    /// Maximum length of the inline query text.
    pub const MAX_QUERY_LENGTH: u32 = 256;
}

/// Limitations for `InlineQueryResult` and its subclasses.
pub mod inline_query_result_limit {
    /// Minimum length of an inline query result ID.
    pub const MIN_ID_LENGTH: u32 = 1;
    /// Maximum length of an inline query result ID.
    pub const MAX_ID_LENGTH: u32 = 64;
}

/// Limitations for `InlineQueryResultsButton`.
pub mod inline_query_results_button_limit {
    /// Minimum length of the start parameter.
    pub const MIN_START_PARAMETER_LENGTH: u32 = 1;
    /// Maximum length of the start parameter.
    pub const MAX_START_PARAMETER_LENGTH: u32 = 64;
}

/// Limitations for `Invoice` / `Bot::send_invoice` / `Bot::create_invoice_link`.
pub mod invoice_limit {
    /// Minimum length of an invoice title.
    pub const MIN_TITLE_LENGTH: u32 = 1;
    /// Maximum length of an invoice title.
    pub const MAX_TITLE_LENGTH: u32 = 32;
    /// Minimum length of an invoice description.
    pub const MIN_DESCRIPTION_LENGTH: u32 = 1;
    /// Maximum length of an invoice description.
    pub const MAX_DESCRIPTION_LENGTH: u32 = 255;
    /// Minimum length of an invoice payload.
    pub const MIN_PAYLOAD_LENGTH: u32 = 1;
    /// Maximum length of an invoice payload.
    pub const MAX_PAYLOAD_LENGTH: u32 = 128;
    /// Maximum number of suggested tip amounts.
    pub const MAX_TIP_AMOUNTS: u32 = 4;
    /// Minimum star count for an invoice.
    pub const MIN_STAR_COUNT: u32 = 1;
    /// Maximum star count for an invoice.
    pub const MAX_STAR_COUNT: u32 = 25000;
    /// 30 days in seconds.
    pub const SUBSCRIPTION_PERIOD: f64 = 2_592_000.0;
    /// Maximum subscription price in Telegram Stars.
    pub const SUBSCRIPTION_MAX_PRICE: u32 = 10000;
}

/// Limitations for `KeyboardButtonRequestUsers`.
pub mod keyboard_button_request_users_limit {
    /// Minimum number of users to request.
    pub const MIN_QUANTITY: u32 = 1;
    /// Maximum number of users to request.
    pub const MAX_QUANTITY: u32 = 10;
}

/// Limitations for `Location` / `ChatLocation` / `Bot::send_location`.
pub mod location_limit {
    /// Minimum length of a chat location address.
    pub const MIN_CHAT_LOCATION_ADDRESS: u32 = 1;
    /// Maximum length of a chat location address.
    pub const MAX_CHAT_LOCATION_ADDRESS: u32 = 64;
    /// Maximum horizontal accuracy in meters.
    pub const HORIZONTAL_ACCURACY: u32 = 1500;
    /// Minimum heading in degrees.
    pub const MIN_HEADING: u32 = 1;
    /// Maximum heading in degrees.
    pub const MAX_HEADING: u32 = 360;
    /// Minimum live period in seconds.
    pub const MIN_LIVE_PERIOD: u32 = 60;
    /// Maximum live period in seconds.
    pub const MAX_LIVE_PERIOD: u32 = 86400;
    /// `0x7FFFFFFF` -- edit indefinitely.
    pub const LIVE_PERIOD_FOREVER: u32 = 0x7FFF_FFFF;
    /// Minimum proximity alert radius in meters.
    pub const MIN_PROXIMITY_ALERT_RADIUS: u32 = 1;
    /// Maximum proximity alert radius in meters.
    pub const MAX_PROXIMITY_ALERT_RADIUS: u32 = 100_000;
}

/// Limitations for `Bot::send_media_group`.
pub mod media_group_limit {
    /// Minimum number of media items in a group.
    pub const MIN_MEDIA_LENGTH: u32 = 2;
    /// Maximum number of media items in a group.
    pub const MAX_MEDIA_LENGTH: u32 = 10;
}

/// Limitations for `Message` / `InputTextMessageContent` / `Bot::send_message`.
pub mod message_limit {
    /// Maximum length of a text message.
    pub const MAX_TEXT_LENGTH: u32 = 4096;
    /// Maximum length of a media caption.
    pub const CAPTION_LENGTH: u32 = 1024;
    /// Minimum length of a text message.
    pub const MIN_TEXT_LENGTH: u32 = 1;
    /// Maximum length of a deep link parameter.
    pub const DEEP_LINK_LENGTH: u32 = 64;
    /// Maximum number of entities in a single message.
    pub const MESSAGE_ENTITIES: u32 = 100;
}

/// Nanostar value constant.
pub mod nanostar {
    /// One nanostar expressed as a fraction of a star.
    pub const VALUE: f64 = 1.0 / 1_000_000_000.0;
}

/// Limitations for nanostar amounts.
pub mod nanostar_limit {
    /// Minimum nanostar amount.
    pub const MIN_AMOUNT: i64 = -999_999_999;
    /// Maximum nanostar amount.
    pub const MAX_AMOUNT: i64 = 999_999_999;
}

/// Limitations for `Bot::get_updates.limit`.
pub mod polling_limit {
    /// Minimum number of updates to retrieve per request.
    pub const MIN_LIMIT: u32 = 1;
    /// Maximum number of updates to retrieve per request.
    pub const MAX_LIMIT: u32 = 100;
}

/// Limitations for `Poll` / `PollOption` / `Bot::send_poll`.
pub mod poll_limit {
    /// Minimum length of a poll question.
    pub const MIN_QUESTION_LENGTH: u32 = 1;
    /// Maximum length of a poll question.
    pub const MAX_QUESTION_LENGTH: u32 = 300;
    /// Minimum length of a poll option text.
    pub const MIN_OPTION_LENGTH: u32 = 1;
    /// Maximum length of a poll option text.
    pub const MAX_OPTION_LENGTH: u32 = 100;
    /// Minimum number of poll options.
    pub const MIN_OPTION_NUMBER: u32 = 2;
    /// Maximum number of poll options.
    pub const MAX_OPTION_NUMBER: u32 = 12;
    /// Maximum length of a quiz explanation text.
    pub const MAX_EXPLANATION_LENGTH: u32 = 200;
    /// Maximum number of line feeds in a quiz explanation.
    pub const MAX_EXPLANATION_LINE_FEEDS: u32 = 2;
    /// Minimum open period for a poll in seconds.
    pub const MIN_OPEN_PERIOD: u32 = 5;
    /// Maximum open period for a poll in seconds.
    pub const MAX_OPEN_PERIOD: u32 = 600;
}

/// Limitations for `Bot::gift_premium_subscription`.
pub mod premium_subscription {
    /// Maximum length of the premium subscription gift text.
    pub const MAX_TEXT_LENGTH: u32 = 128;
    /// Three-month subscription duration.
    pub const MONTH_COUNT_THREE: u32 = 3;
    /// Six-month subscription duration.
    pub const MONTH_COUNT_SIX: u32 = 6;
    /// Twelve-month subscription duration.
    pub const MONTH_COUNT_TWELVE: u32 = 12;
    /// Star cost for a three-month premium subscription.
    pub const STARS_THREE_MONTHS: u32 = 1000;
    /// Star cost for a six-month premium subscription.
    pub const STARS_SIX_MONTHS: u32 = 1500;
    /// Star cost for a twelve-month premium subscription.
    pub const STARS_TWELVE_MONTHS: u32 = 2500;
}

/// Limitations for `ForceReply` and `ReplyKeyboardMarkup`.
pub mod reply_limit {
    /// Minimum length of the input field placeholder.
    pub const MIN_INPUT_FIELD_PLACEHOLDER: u32 = 1;
    /// Maximum length of the input field placeholder.
    pub const MAX_INPUT_FIELD_PLACEHOLDER: u32 = 64;
}

/// Limitations for `Bot::get_star_transactions`.
pub mod star_transactions_limit {
    /// Minimum number of transactions to retrieve.
    pub const MIN_LIMIT: u32 = 1;
    /// Maximum number of transactions to retrieve.
    pub const MAX_LIMIT: u32 = 100;
}

/// Limitations for various sticker methods.
pub mod sticker_limit {
    /// Minimum length of a sticker set name or title.
    pub const MIN_NAME_AND_TITLE: u32 = 1;
    /// Maximum length of a sticker set name or title.
    pub const MAX_NAME_AND_TITLE: u32 = 64;
    /// Minimum number of emojis associated with a sticker.
    pub const MIN_STICKER_EMOJI: u32 = 1;
    /// Maximum number of emojis associated with a sticker.
    pub const MAX_STICKER_EMOJI: u32 = 20;
    /// Maximum number of search keywords for a sticker.
    pub const MAX_SEARCH_KEYWORDS: u32 = 20;
    /// Maximum length of a single search keyword.
    pub const MAX_KEYWORD_LENGTH: u32 = 64;
}

/// Limitations for sticker set methods.
pub mod sticker_set_limit {
    /// Minimum number of initial stickers when creating a set.
    pub const MIN_INITIAL_STICKERS: u32 = 1;
    /// Maximum number of initial stickers when creating a set.
    pub const MAX_INITIAL_STICKERS: u32 = 50;
    /// Maximum number of emoji stickers in a set.
    pub const MAX_EMOJI_STICKERS: u32 = 200;
    /// Maximum number of animated stickers in a set.
    pub const MAX_ANIMATED_STICKERS: u32 = 50;
    /// Maximum number of static stickers in a set.
    pub const MAX_STATIC_STICKERS: u32 = 120;
    /// Maximum static thumbnail size in kilobytes.
    pub const MAX_STATIC_THUMBNAIL_SIZE: u32 = 128;
    /// Maximum animated thumbnail size in kilobytes.
    pub const MAX_ANIMATED_THUMBNAIL_SIZE: u32 = 32;
    /// Required dimensions (width/height) for a static thumbnail in pixels.
    pub const STATIC_THUMB_DIMENSIONS: u32 = 100;
}

/// Limitations for `StoryAreaPosition`.
pub mod story_area_position_limit {
    /// Maximum rotation angle in degrees for a story area.
    pub const MAX_ROTATION_ANGLE: u32 = 360;
}

/// Limitations for subclasses of `StoryAreaType`.
pub mod story_area_type_limit {
    /// Maximum number of location areas per story.
    pub const MAX_LOCATION_AREAS: u32 = 10;
    /// Maximum number of suggested reaction areas per story.
    pub const MAX_SUGGESTED_REACTION_AREAS: u32 = 5;
    /// Maximum number of link areas per story.
    pub const MAX_LINK_AREAS: u32 = 3;
    /// Maximum number of weather areas per story.
    pub const MAX_WEATHER_AREAS: u32 = 3;
    /// Maximum number of unique gift areas per story.
    pub const MAX_UNIQUE_GIFT_AREAS: u32 = 1;
}

/// Limitations for `Bot::post_story` and `Bot::edit_story`.
pub mod story_limit {
    /// Maximum length of a story caption.
    pub const CAPTION_LENGTH: u32 = 2048;
    /// 6 hours in seconds.
    pub const ACTIVITY_SIX_HOURS: u32 = 6 * 3600;
    /// 12 hours in seconds.
    pub const ACTIVITY_TWELVE_HOURS: u32 = 12 * 3600;
    /// 1 day in seconds.
    pub const ACTIVITY_ONE_DAY: u32 = 86400;
    /// 2 days in seconds.
    pub const ACTIVITY_TWO_DAYS: u32 = 2 * 86400;
}

/// Limitations for `SuggestedPostPrice` / `SuggestedPostParameters` / `Bot::decline_suggested_post`.
pub mod suggested_post {
    /// Minimum suggested post price in Telegram Stars.
    pub const MIN_PRICE_STARS: u32 = 5;
    /// Maximum suggested post price in Telegram Stars.
    pub const MAX_PRICE_STARS: u32 = 100_000;
    /// Minimum suggested post price in nanotoncoins.
    pub const MIN_PRICE_NANOTONCOINS: u64 = 10_000_000;
    /// Maximum suggested post price in nanotoncoins.
    pub const MAX_PRICE_NANOTONCOINS: u64 = 10_000_000_000_000;
    /// Minimum send date offset in seconds from now.
    pub const MIN_SEND_DATE: u32 = 300;
    /// Maximum send date offset in seconds from now.
    pub const MAX_SEND_DATE: u32 = 2_678_400;
    /// Maximum length of a decline comment.
    pub const MAX_COMMENT_LENGTH: u32 = 128;
}

/// Limitations for `Bot::get_user_profile_photos.limit`.
pub mod user_profile_photos_limit {
    /// Minimum number of profile photos to retrieve.
    pub const MIN_LIMIT: u32 = 1;
    /// Maximum number of profile photos to retrieve.
    pub const MAX_LIMIT: u32 = 100;
}

/// Limitations for `Bot::get_user_profile_audios.limit`.
pub mod user_profile_audios_limit {
    /// Minimum number of profile audios to retrieve.
    pub const MIN_LIMIT: u32 = 1;
    /// Maximum number of profile audios to retrieve.
    pub const MAX_LIMIT: u32 = 100;
}

/// Limitations for `Bot::set_webhook`.
pub mod webhook_limit {
    /// Minimum number of simultaneous webhook connections.
    pub const MIN_CONNECTIONS_LIMIT: u32 = 1;
    /// Maximum number of simultaneous webhook connections.
    pub const MAX_CONNECTIONS_LIMIT: u32 = 100;
    /// Minimum length of the webhook secret token.
    pub const MIN_SECRET_TOKEN_LENGTH: u32 = 1;
    /// Maximum length of the webhook secret token.
    pub const MAX_SECRET_TOKEN_LENGTH: u32 = 256;
}

/// Limitations for `Bot::verify_chat` and `Bot::verify_user`.
pub mod verify_limit {
    /// Maximum length of the verification text.
    pub const MAX_TEXT_LENGTH: u32 = 70;
}

/// Limitations for `Bot::set_chat_member_tag`.
pub mod tag_limit {
    /// Maximum length of a chat member tag.
    pub const MAX_TAG_LENGTH: u32 = 16;
}
