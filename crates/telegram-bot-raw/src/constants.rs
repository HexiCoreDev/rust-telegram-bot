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
    pub const COLOR_000: AccentColorEntry = AccentColorEntry {
        identifier: 0,
        name: Some("red"),
        light_colors: &[],
        dark_colors: &[],
    };
    pub const COLOR_001: AccentColorEntry = AccentColorEntry {
        identifier: 1,
        name: Some("orange"),
        light_colors: &[],
        dark_colors: &[],
    };
    pub const COLOR_002: AccentColorEntry = AccentColorEntry {
        identifier: 2,
        name: Some("purple/violet"),
        light_colors: &[],
        dark_colors: &[],
    };
    pub const COLOR_003: AccentColorEntry = AccentColorEntry {
        identifier: 3,
        name: Some("green"),
        light_colors: &[],
        dark_colors: &[],
    };
    pub const COLOR_004: AccentColorEntry = AccentColorEntry {
        identifier: 4,
        name: Some("cyan"),
        light_colors: &[],
        dark_colors: &[],
    };
    pub const COLOR_005: AccentColorEntry = AccentColorEntry {
        identifier: 5,
        name: Some("blue"),
        light_colors: &[],
        dark_colors: &[],
    };
    pub const COLOR_006: AccentColorEntry = AccentColorEntry {
        identifier: 6,
        name: Some("pink"),
        light_colors: &[],
        dark_colors: &[],
    };
    pub const COLOR_007: AccentColorEntry = AccentColorEntry {
        identifier: 7,
        name: None,
        light_colors: &[0xE15052, 0xF9AE63],
        dark_colors: &[0xFF9380, 0x992F37],
    };
    pub const COLOR_008: AccentColorEntry = AccentColorEntry {
        identifier: 8,
        name: None,
        light_colors: &[0xE0802B, 0xFAC534],
        dark_colors: &[0xECB04E, 0xC35714],
    };
    pub const COLOR_009: AccentColorEntry = AccentColorEntry {
        identifier: 9,
        name: None,
        light_colors: &[0xA05FF3, 0xF48FFF],
        dark_colors: &[0xC697FF, 0x5E31C8],
    };
    pub const COLOR_010: AccentColorEntry = AccentColorEntry {
        identifier: 10,
        name: None,
        light_colors: &[0x27A910, 0xA7DC57],
        dark_colors: &[0xA7EB6E, 0x167E2D],
    };
    pub const COLOR_011: AccentColorEntry = AccentColorEntry {
        identifier: 11,
        name: None,
        light_colors: &[0x27ACCE, 0x82E8D6],
        dark_colors: &[0x40D8D0, 0x045C7F],
    };
    pub const COLOR_012: AccentColorEntry = AccentColorEntry {
        identifier: 12,
        name: None,
        light_colors: &[0x3391D4, 0x7DD3F0],
        dark_colors: &[0x52BFFF, 0x0B5494],
    };
    pub const COLOR_013: AccentColorEntry = AccentColorEntry {
        identifier: 13,
        name: None,
        light_colors: &[0xDD4371, 0xFFBE9F],
        dark_colors: &[0xFF86A6, 0x8E366E],
    };
    pub const COLOR_014: AccentColorEntry = AccentColorEntry {
        identifier: 14,
        name: None,
        light_colors: &[0x247BED, 0xF04856, 0xFFFFFF],
        dark_colors: &[0x3FA2FE, 0xE5424F, 0xFFFFFF],
    };
    pub const COLOR_015: AccentColorEntry = AccentColorEntry {
        identifier: 15,
        name: None,
        light_colors: &[0xD67722, 0x1EA011, 0xFFFFFF],
        dark_colors: &[0xFF905E, 0x32A527, 0xFFFFFF],
    };
    pub const COLOR_016: AccentColorEntry = AccentColorEntry {
        identifier: 16,
        name: None,
        light_colors: &[0x179E42, 0xE84A3F, 0xFFFFFF],
        dark_colors: &[0x66D364, 0xD5444F, 0xFFFFFF],
    };
    pub const COLOR_017: AccentColorEntry = AccentColorEntry {
        identifier: 17,
        name: None,
        light_colors: &[0x2894AF, 0x6FC456, 0xFFFFFF],
        dark_colors: &[0x22BCE2, 0x3DA240, 0xFFFFFF],
    };
    pub const COLOR_018: AccentColorEntry = AccentColorEntry {
        identifier: 18,
        name: None,
        light_colors: &[0x0C9AB3, 0xFFAD95, 0xFFE6B5],
        dark_colors: &[0x22BCE2, 0xFF9778, 0xFFDA6B],
    };
    pub const COLOR_019: AccentColorEntry = AccentColorEntry {
        identifier: 19,
        name: None,
        light_colors: &[0x7757D6, 0xF79610, 0xFFDE8E],
        dark_colors: &[0x9791FF, 0xF2731D, 0xFFDB59],
    };
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
    pub const COLOR_000: AccentColorEntry = AccentColorEntry {
        identifier: 0,
        name: None,
        light_colors: &[0xBA5650],
        dark_colors: &[0x9C4540],
    };
    pub const COLOR_001: AccentColorEntry = AccentColorEntry {
        identifier: 1,
        name: None,
        light_colors: &[0xC27C3E],
        dark_colors: &[0x945E2C],
    };
    pub const COLOR_002: AccentColorEntry = AccentColorEntry {
        identifier: 2,
        name: None,
        light_colors: &[0x956AC8],
        dark_colors: &[0x715099],
    };
    pub const COLOR_003: AccentColorEntry = AccentColorEntry {
        identifier: 3,
        name: None,
        light_colors: &[0x49A355],
        dark_colors: &[0x33713B],
    };
    pub const COLOR_004: AccentColorEntry = AccentColorEntry {
        identifier: 4,
        name: None,
        light_colors: &[0x3E97AD],
        dark_colors: &[0x387E87],
    };
    pub const COLOR_005: AccentColorEntry = AccentColorEntry {
        identifier: 5,
        name: None,
        light_colors: &[0x5A8FBB],
        dark_colors: &[0x477194],
    };
    pub const COLOR_006: AccentColorEntry = AccentColorEntry {
        identifier: 6,
        name: None,
        light_colors: &[0xB85378],
        dark_colors: &[0x944763],
    };
    pub const COLOR_007: AccentColorEntry = AccentColorEntry {
        identifier: 7,
        name: None,
        light_colors: &[0x7F8B95],
        dark_colors: &[0x435261],
    };
    pub const COLOR_008: AccentColorEntry = AccentColorEntry {
        identifier: 8,
        name: None,
        light_colors: &[0xC9565D, 0xD97C57],
        dark_colors: &[0x994343, 0xAC583E],
    };
    pub const COLOR_009: AccentColorEntry = AccentColorEntry {
        identifier: 9,
        name: None,
        light_colors: &[0xCF7244, 0xCC9433],
        dark_colors: &[0x8F552F, 0xA17232],
    };
    pub const COLOR_010: AccentColorEntry = AccentColorEntry {
        identifier: 10,
        name: None,
        light_colors: &[0x9662D4, 0xB966B6],
        dark_colors: &[0x634691, 0x9250A2],
    };
    pub const COLOR_011: AccentColorEntry = AccentColorEntry {
        identifier: 11,
        name: None,
        light_colors: &[0x3D9755, 0x89A650],
        dark_colors: &[0x296A43, 0x5F8F44],
    };
    pub const COLOR_012: AccentColorEntry = AccentColorEntry {
        identifier: 12,
        name: None,
        light_colors: &[0x3D95BA, 0x50AD98],
        dark_colors: &[0x306C7C, 0x3E987E],
    };
    pub const COLOR_013: AccentColorEntry = AccentColorEntry {
        identifier: 13,
        name: None,
        light_colors: &[0x538BC2, 0x4DA8BD],
        dark_colors: &[0x38618C, 0x458BA1],
    };
    pub const COLOR_014: AccentColorEntry = AccentColorEntry {
        identifier: 14,
        name: None,
        light_colors: &[0xB04F74, 0xD1666D],
        dark_colors: &[0x884160, 0xA65259],
    };
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
    Fill,
    Wallpaper,
    Pattern,
    ChatTheme,
}

/// Available types of `BackgroundFill`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum BackgroundFillType {
    Solid,
    Gradient,
    FreeformGradient,
}

/// Available types of `BotCommandScope`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum BotCommandScopeType {
    Default,
    AllPrivateChats,
    AllGroupChats,
    AllChatAdministrators,
    Chat,
    ChatAdministrators,
    ChatMember,
}

/// Available chat actions for `Bot::send_chat_action`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ChatAction {
    ChooseSticker,
    FindLocation,
    RecordVoice,
    RecordVideo,
    RecordVideoNote,
    Typing,
    UploadVoice,
    UploadDocument,
    UploadPhoto,
    UploadVideo,
    UploadVideoNote,
}

/// Available sources for a `ChatBoostSource`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ChatBoostSources {
    GiftCode,
    Giveaway,
    Premium,
}

/// Available states for `ChatMember`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ChatMemberStatus {
    #[serde(rename = "administrator")]
    Administrator,
    #[serde(rename = "creator")]
    Owner,
    #[serde(rename = "kicked")]
    Banned,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "member")]
    Member,
    #[serde(rename = "restricted")]
    Restricted,
}

/// Available types of `Chat`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ChatType {
    Sender,
    Private,
    Group,
    Supergroup,
    Channel,
}

/// Available emoji for `Dice` / `Bot::send_dice`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DiceEmoji {
    #[serde(rename = "\u{1F3B2}")]
    Dice,
    #[serde(rename = "\u{1F3AF}")]
    Darts,
    #[serde(rename = "\u{1F3C0}")]
    Basketball,
    #[serde(rename = "\u{26BD}")]
    Football,
    #[serde(rename = "\u{1F3B0}")]
    SlotMachine,
    #[serde(rename = "\u{1F3B3}")]
    Bowling,
}

/// Available types of `InlineQueryResult`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum InlineQueryResultType {
    #[serde(rename = "audio")]
    Audio,
    #[serde(rename = "document")]
    Document,
    #[serde(rename = "gif")]
    Gif,
    #[serde(rename = "mpeg4_gif")]
    Mpeg4Gif,
    #[serde(rename = "photo")]
    Photo,
    #[serde(rename = "sticker")]
    Sticker,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "voice")]
    Voice,
    #[serde(rename = "article")]
    Article,
    #[serde(rename = "contact")]
    Contact,
    #[serde(rename = "game")]
    Game,
    #[serde(rename = "location")]
    Location,
    #[serde(rename = "venue")]
    Venue,
}

/// Available types of `InputMedia`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum InputMediaType {
    Animation,
    Document,
    Audio,
    Photo,
    Video,
}

/// Available types of `InputPaidMedia`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum InputPaidMediaType {
    Photo,
    Video,
}

/// Available types of `InputProfilePhoto`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum InputProfilePhotoType {
    Static,
    Animated,
}

/// Available types of `InputStoryContent`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum InputStoryContentType {
    Photo,
    Video,
}

/// Available button styles for `InlineKeyboardButton` and `KeyboardButton`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum KeyboardButtonStyle {
    Primary,
    Success,
    Danger,
}

/// Available positions for `MaskPosition`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MaskPositionPoint {
    Forehead,
    Eyes,
    Mouth,
    Chin,
}

/// Available types of `MenuButton`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MenuButtonType {
    #[serde(rename = "commands")]
    Commands,
    #[serde(rename = "web_app")]
    WebApp,
    #[serde(rename = "default")]
    Default,
}

/// Available types of `Message` that can be seen as attachment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MessageAttachmentType {
    Animation,
    Audio,
    Contact,
    Dice,
    Document,
    Game,
    Invoice,
    Location,
    PaidMedia,
    PassportData,
    Photo,
    Poll,
    Sticker,
    Story,
    SuccessfulPayment,
    Video,
    VideoNote,
    Voice,
    Venue,
}

/// Available types of `MessageEntity`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MessageEntityType {
    Blockquote,
    Bold,
    BotCommand,
    Cashtag,
    Code,
    CustomEmoji,
    DateTime,
    Email,
    ExpandableBlockquote,
    Hashtag,
    Italic,
    Mention,
    PhoneNumber,
    Pre,
    Spoiler,
    Strikethrough,
    TextLink,
    TextMention,
    Underline,
    Url,
}

/// All possible formats for `MessageEntity.date_time_format`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MessageEntityDateTimeFormats {
    #[serde(rename = "r")]
    Relative,
    #[serde(rename = "w")]
    LocalizedWeekday,
    #[serde(rename = "d")]
    ShortDate,
    #[serde(rename = "D")]
    LongDate,
    #[serde(rename = "t")]
    ShortTime,
    #[serde(rename = "T")]
    LongTime,
    #[serde(rename = "wd")]
    LocalizedWeekdayShortDate,
    #[serde(rename = "wD")]
    LocalizedWeekdayLongDate,
    #[serde(rename = "wt")]
    LocalizedWeekdayShortTime,
    #[serde(rename = "wT")]
    LocalizedWeekdayLongTime,
    #[serde(rename = "wdt")]
    LocalizedWeekdayShortDateShortTime,
    #[serde(rename = "wdT")]
    LocalizedWeekdayShortDateLongTime,
    #[serde(rename = "wDt")]
    LocalizedWeekdayLongDateShortTime,
    #[serde(rename = "wDT")]
    LocalizedWeekdayLongDateLongTime,
    #[serde(rename = "dt")]
    ShortDateShortTime,
    #[serde(rename = "dT")]
    ShortDateLongTime,
    #[serde(rename = "Dt")]
    LongDateShortTime,
    #[serde(rename = "DT")]
    LongDateLongTime,
}

/// Available types of `MessageOrigin`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MessageOriginType {
    User,
    HiddenUser,
    Chat,
    Channel,
}

/// Available types of `Message`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum MessageType {
    Animation,
    Audio,
    BoostAdded,
    BusinessConnectionId,
    ChannelChatCreated,
    ChatBackgroundSet,
    ChatOwnerChanged,
    ChatOwnerLeft,
    ChatShared,
    Checklist,
    ChecklistTasksAdded,
    ChecklistTasksDone,
    ConnectedWebsite,
    Contact,
    DeleteChatPhoto,
    Dice,
    DirectMessagePriceChanged,
    Document,
    EffectId,
    ForumTopicCreated,
    ForumTopicClosed,
    ForumTopicEdited,
    ForumTopicReopened,
    Game,
    GeneralForumTopicHidden,
    GeneralForumTopicUnhidden,
    Gift,
    GiftUpgradeSent,
    Giveaway,
    GiveawayCreated,
    GiveawayWinners,
    GiveawayCompleted,
    GroupChatCreated,
    Invoice,
    LeftChatMember,
    Location,
    MessageAutoDeleteTimerChanged,
    MigrateToChatId,
    NewChatMembers,
    NewChatTitle,
    NewChatPhoto,
    PaidMedia,
    PaidMessagePriceChanged,
    SuggestedPostApprovalFailed,
    SuggestedPostApproved,
    SuggestedPostDeclined,
    SuggestedPostInfo,
    SuggestedPostPaid,
    SuggestedPostRefunded,
    PassportData,
    Photo,
    PinnedMessage,
    Poll,
    ProximityAlertTriggered,
    RefundedPayment,
    ReplyToStory,
    SenderBoostCount,
    SenderBusinessBot,
    Sticker,
    Story,
    SupergroupChatCreated,
    SuccessfulPayment,
    Text,
    UniqueGift,
    UsersShared,
    Venue,
    Video,
    VideoChatScheduled,
    VideoChatStarted,
    VideoChatEnded,
    VideoChatParticipantsInvited,
    VideoNote,
    Voice,
    WebAppData,
    WriteAccessAllowed,
}

/// Available types of `OwnedGift`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum OwnedGiftType {
    Regular,
    Unique,
}

/// Available types of `PaidMedia`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PaidMediaType {
    Preview,
    Video,
    Photo,
}

/// Available parse modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ParseMode {
    #[serde(rename = "HTML")]
    Html,
    #[serde(rename = "MarkdownV2")]
    MarkdownV2,
    #[serde(rename = "Markdown")]
    Markdown,
}

/// Available types for `Poll`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum PollType {
    Regular,
    Quiz,
}

/// Available types of `ReactionType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum ReactionType {
    Emoji,
    CustomEmoji,
    Paid,
}

/// Available types of `RevenueWithdrawalState`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum RevenueWithdrawalStateType {
    Pending,
    Succeeded,
    Failed,
}

/// Available formats of `Sticker` in the set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum StickerFormat {
    Static,
    Animated,
    Video,
}

/// Available types of `Sticker`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum StickerType {
    Regular,
    Mask,
    CustomEmoji,
}

/// Available types of `StoryAreaType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum StoryAreaTypeType {
    Location,
    SuggestedReaction,
    Link,
    Weather,
    UniqueGift,
}

/// Available states of `SuggestedPostInfo.state`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SuggestedPostInfoState {
    Pending,
    Approved,
    Declined,
}

/// Available refund reasons for `SuggestedPostRefunded`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum SuggestedPostRefunded {
    PostDeleted,
    PaymentRefunded,
}

/// Available types of `TransactionPartner`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TransactionPartnerType {
    AffiliateProgram,
    Chat,
    Fragment,
    Other,
    TelegramAds,
    TelegramApi,
    User,
}

/// Constants for `TransactionPartnerUser.transaction_type`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum TransactionPartnerUser {
    InvoicePayment,
    PaidMediaPayment,
    GiftPurchase,
    PremiumPurchase,
    BusinessAccountTransfer,
}

/// Available origins for `UniqueGiftInfo`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum UniqueGiftInfoOrigin {
    #[serde(rename = "gifted_upgrade")]
    GiftedUpgrade,
    #[serde(rename = "OFFER")]
    Offer,
    #[serde(rename = "resale")]
    Resale,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "upgrade")]
    Upgrade,
}

/// Available rarities for `UniqueGiftModel`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum UniqueGiftModelRarity {
    Uncommon,
    Rare,
    Epic,
    Legendary,
}

/// Available types of `Update`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[non_exhaustive]
pub enum UpdateType {
    Message,
    EditedMessage,
    ChannelPost,
    EditedChannelPost,
    InlineQuery,
    ChosenInlineResult,
    CallbackQuery,
    ShippingQuery,
    PreCheckoutQuery,
    Poll,
    PollAnswer,
    MyChatMember,
    ChatMember,
    ChatJoinRequest,
    ChatBoost,
    RemovedChatBoost,
    MessageReaction,
    MessageReactionCount,
    BusinessConnection,
    BusinessMessage,
    EditedBusinessMessage,
    DeletedBusinessMessages,
    PurchasedPaidMedia,
}

/// Available emojis of `ReactionTypeEmoji`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum ReactionEmoji {
    #[serde(rename = "\u{1F44D}")]
    ThumbsUp,
    #[serde(rename = "\u{1F44E}")]
    ThumbsDown,
    #[serde(rename = "\u{2764}")]
    RedHeart,
    #[serde(rename = "\u{1F525}")]
    Fire,
    #[serde(rename = "\u{1F970}")]
    SmilingFaceWithHearts,
    #[serde(rename = "\u{1F44F}")]
    ClappingHands,
    #[serde(rename = "\u{1F601}")]
    GrinningFaceWithSmilingEyes,
    #[serde(rename = "\u{1F914}")]
    ThinkingFace,
    #[serde(rename = "\u{1F92F}")]
    ShockedFaceWithExplodingHead,
    #[serde(rename = "\u{1F631}")]
    FaceScreamingInFear,
    #[serde(rename = "\u{1F92C}")]
    SeriousFaceWithSymbolsCoveringMouth,
    #[serde(rename = "\u{1F622}")]
    CryingFace,
    #[serde(rename = "\u{1F389}")]
    PartyPopper,
    #[serde(rename = "\u{1F929}")]
    GrinningFaceWithStarEyes,
    #[serde(rename = "\u{1F92E}")]
    FaceWithOpenMouthVomiting,
    #[serde(rename = "\u{1F4A9}")]
    PileOfPoo,
    #[serde(rename = "\u{1F64F}")]
    PersonWithFoldedHands,
    #[serde(rename = "\u{1F44C}")]
    OkHandSign,
    #[serde(rename = "\u{1F54A}")]
    DoveOfPeace,
    #[serde(rename = "\u{1F921}")]
    ClownFace,
    #[serde(rename = "\u{1F971}")]
    YawningFace,
    #[serde(rename = "\u{1F974}")]
    FaceWithUnevenEyesAndWavyMouth,
    #[serde(rename = "\u{1F60D}")]
    SmilingFaceWithHeartShapedEyes,
    #[serde(rename = "\u{1F433}")]
    SpoutingWhale,
    #[serde(rename = "\u{2764}\u{FE0F}\u{200D}\u{1F525}")]
    HeartOnFire,
    #[serde(rename = "\u{1F31A}")]
    NewMoonWithFace,
    #[serde(rename = "\u{1F32D}")]
    HotDog,
    #[serde(rename = "\u{1F4AF}")]
    HundredPointsSymbol,
    #[serde(rename = "\u{1F923}")]
    RollingOnTheFloorLaughing,
    #[serde(rename = "\u{26A1}")]
    HighVoltageSign,
    #[serde(rename = "\u{1F34C}")]
    Banana,
    #[serde(rename = "\u{1F3C6}")]
    Trophy,
    #[serde(rename = "\u{1F494}")]
    BrokenHeart,
    #[serde(rename = "\u{1F928}")]
    FaceWithOneEyebrowRaised,
    #[serde(rename = "\u{1F610}")]
    NeutralFace,
    #[serde(rename = "\u{1F353}")]
    Strawberry,
    #[serde(rename = "\u{1F37E}")]
    BottleWithPoppingCork,
    #[serde(rename = "\u{1F48B}")]
    KissMark,
    #[serde(rename = "\u{1F595}")]
    ReversedHandWithMiddleFingerExtended,
    #[serde(rename = "\u{1F608}")]
    SmilingFaceWithHorns,
    #[serde(rename = "\u{1F634}")]
    SleepingFace,
    #[serde(rename = "\u{1F62D}")]
    LoudlyCryingFace,
    #[serde(rename = "\u{1F913}")]
    NerdFace,
    #[serde(rename = "\u{1F47B}")]
    Ghost,
    #[serde(rename = "\u{1F468}\u{200D}\u{1F4BB}")]
    ManTechnologist,
    #[serde(rename = "\u{1F440}")]
    Eyes,
    #[serde(rename = "\u{1F383}")]
    JackOLantern,
    #[serde(rename = "\u{1F648}")]
    SeeNoEvilMonkey,
    #[serde(rename = "\u{1F607}")]
    SmilingFaceWithHalo,
    #[serde(rename = "\u{1F628}")]
    FearfulFace,
    #[serde(rename = "\u{1F91D}")]
    Handshake,
    #[serde(rename = "\u{270D}")]
    WritingHand,
    #[serde(rename = "\u{1F917}")]
    HuggingFace,
    #[serde(rename = "\u{1FAE1}")]
    SalutingFace,
    #[serde(rename = "\u{1F385}")]
    FatherChristmas,
    #[serde(rename = "\u{1F384}")]
    ChristmasTree,
    #[serde(rename = "\u{2603}")]
    Snowman,
    #[serde(rename = "\u{1F485}")]
    NailPolish,
    #[serde(rename = "\u{1F92A}")]
    GrinningFaceWithOneLargeAndOneSmallEye,
    #[serde(rename = "\u{1F5FF}")]
    Moyai,
    #[serde(rename = "\u{1F192}")]
    SquaredCool,
    #[serde(rename = "\u{1F498}")]
    HeartWithArrow,
    #[serde(rename = "\u{1F649}")]
    HearNoEvilMonkey,
    #[serde(rename = "\u{1F984}")]
    UnicornFace,
    #[serde(rename = "\u{1F618}")]
    FaceThrowingAKiss,
    #[serde(rename = "\u{1F48A}")]
    Pill,
    #[serde(rename = "\u{1F64A}")]
    SpeakNoEvilMonkey,
    #[serde(rename = "\u{1F60E}")]
    SmilingFaceWithSunglasses,
    #[serde(rename = "\u{1F47E}")]
    AlienMonster,
    #[serde(rename = "\u{1F937}\u{200D}\u{2642}\u{FE0F}")]
    ManShrugging,
    #[serde(rename = "\u{1F937}")]
    Shrug,
    #[serde(rename = "\u{1F937}\u{200D}\u{2640}\u{FE0F}")]
    WomanShrugging,
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
    pub const MIN_COMMAND: u32 = 1;
    pub const MAX_COMMAND: u32 = 32;
    pub const MIN_DESCRIPTION: u32 = 1;
    pub const MAX_DESCRIPTION: u32 = 256;
    pub const MAX_COMMAND_NUMBER: u32 = 100;
}

/// Limitations for `Bot::set_my_description` and `Bot::set_my_short_description`.
pub mod bot_description_limit {
    pub const MAX_DESCRIPTION_LENGTH: u32 = 512;
    pub const MAX_SHORT_DESCRIPTION_LENGTH: u32 = 120;
}

/// Limitations for `Bot::set_my_name`.
pub mod bot_name_limit {
    pub const MAX_NAME_LENGTH: u32 = 64;
}

/// Limitations for `Bot::delete_messages`, `Bot::forward_messages`, and `Bot::copy_messages`.
pub mod bulk_request_limit {
    pub const MIN_LIMIT: u32 = 1;
    pub const MAX_LIMIT: u32 = 100;
}

/// Limitations related to handling business accounts.
pub mod business_limit {
    /// 24 hours in seconds.
    pub const CHAT_ACTIVITY_TIMEOUT: u32 = 86400;
    pub const MIN_NAME_LENGTH: u32 = 1;
    pub const MAX_NAME_LENGTH: u32 = 64;
    pub const MAX_USERNAME_LENGTH: u32 = 32;
    pub const MAX_BIO_LENGTH: u32 = 140;
    pub const MIN_GIFT_RESULTS: u32 = 1;
    pub const MAX_GIFT_RESULTS: u32 = 100;
    pub const MIN_STAR_COUNT: u32 = 1;
    pub const MAX_STAR_COUNT: u32 = 10000;
}

/// Limitations for `CallbackQuery` / `Bot::answer_callback_query`.
pub mod callback_query_limit {
    pub const ANSWER_CALLBACK_QUERY_TEXT_LENGTH: u32 = 200;
}

/// Special chat IDs.
pub mod chat_id {
    pub const ANONYMOUS_ADMIN: i64 = 1_087_968_824;
    pub const SERVICE_CHAT: i64 = 777_000;
    pub const FAKE_CHANNEL: i64 = 136_817_688;
}

/// Limitations for `ChatInviteLink`.
pub mod chat_invite_link_limit {
    pub const MIN_MEMBER_LIMIT: u32 = 1;
    pub const MAX_MEMBER_LIMIT: u32 = 99999;
    pub const NAME_LENGTH: u32 = 32;
}

/// Limitations for chat title, description, and admin custom title.
pub mod chat_limit {
    pub const CHAT_ADMINISTRATOR_CUSTOM_TITLE_LENGTH: u32 = 16;
    pub const CHAT_DESCRIPTION_LENGTH: u32 = 255;
    pub const MIN_CHAT_TITLE_LENGTH: u32 = 1;
    pub const MAX_CHAT_TITLE_LENGTH: u32 = 128;
}

/// Limitations for chat subscription invite links.
pub mod chat_subscription_limit {
    /// 30 days in seconds.
    pub const SUBSCRIPTION_PERIOD: u32 = 2_592_000;
    pub const MIN_PRICE: u32 = 1;
    pub const MAX_PRICE: u32 = 10000;
}

/// Limitations for `ChatPhoto` sizes.
pub mod chat_photo_size {
    pub const SMALL: u32 = 160;
    pub const BIG: u32 = 640;
}

/// Limitations for `BackgroundType` subclasses.
pub mod background_type_limit {
    pub const MAX_DIMMING: u32 = 100;
    pub const MAX_INTENSITY: u32 = 100;
}

/// Limitations for `BackgroundFillGradient`.
pub mod background_fill_limit {
    pub const MAX_ROTATION_ANGLE: u32 = 359;
}

/// Limitations for `Contact` vcard.
pub mod contact_limit {
    pub const VCARD: u32 = 2048;
}

/// Limitations for `Bot::get_custom_emoji_stickers`.
pub mod custom_emoji_sticker_limit {
    pub const CUSTOM_EMOJI_IDENTIFIER_LIMIT: u32 = 200;
}

/// Limitations for `Dice` value ranges.
pub mod dice_limit {
    pub const MIN_VALUE: u32 = 1;
    pub const MAX_VALUE_BASKETBALL: u32 = 5;
    pub const MAX_VALUE_BOWLING: u32 = 6;
    pub const MAX_VALUE_DARTS: u32 = 6;
    pub const MAX_VALUE_DICE: u32 = 6;
    pub const MAX_VALUE_FOOTBALL: u32 = 5;
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
    pub const MESSAGES_PER_SECOND_PER_CHAT: u32 = 1;
    pub const MESSAGES_PER_SECOND: u32 = 30;
    pub const MESSAGES_PER_MINUTE_PER_GROUP: u32 = 20;
    pub const PAID_MESSAGES_PER_SECOND: u32 = 1000;
}

/// Available colors for `Bot::create_forum_topic.icon_color`.
pub mod forum_icon_color {
    pub const BLUE: u32 = 0x6FB9F0;
    pub const YELLOW: u32 = 0xFFD67E;
    pub const PURPLE: u32 = 0xCB86DB;
    pub const GREEN: u32 = 0x8EEE98;
    pub const PINK: u32 = 0xFF93B2;
    pub const RED: u32 = 0xFB6F5F;
}

/// Limitations for `Bot::create_forum_topic` and `Bot::edit_forum_topic`.
pub mod forum_topic_limit {
    pub const MIN_NAME_LENGTH: u32 = 1;
    pub const MAX_NAME_LENGTH: u32 = 128;
}

/// Limitations for `Bot::send_gift`.
pub mod gift_limit {
    pub const MAX_TEXT_LENGTH: u32 = 128;
}

/// Limitations for `Giveaway` and related classes.
pub mod giveaway_limit {
    pub const MAX_WINNERS: u32 = 100;
}

/// Limitations for `InlineKeyboardButton`.
pub mod inline_keyboard_button_limit {
    pub const MIN_CALLBACK_DATA: u32 = 1;
    pub const MAX_CALLBACK_DATA: u32 = 64;
    pub const MIN_COPY_TEXT: u32 = 1;
    pub const MAX_COPY_TEXT: u32 = 256;
}

/// Limitations for `InlineKeyboardMarkup`.
pub mod inline_keyboard_markup_limit {
    pub const TOTAL_BUTTON_NUMBER: u32 = 100;
    pub const BUTTONS_PER_ROW: u32 = 8;
}

/// Limitations for `InputChecklist` / `InputChecklistTask`.
pub mod input_checklist_limit {
    pub const MIN_TITLE_LENGTH: u32 = 1;
    pub const MAX_TITLE_LENGTH: u32 = 255;
    pub const MIN_TEXT_LENGTH: u32 = 1;
    pub const MAX_TEXT_LENGTH: u32 = 100;
    pub const MIN_TASK_NUMBER: u32 = 1;
    pub const MAX_TASK_NUMBER: u32 = 30;
}

/// Limitations for `InputStoryContentPhoto` / `InputStoryContentVideo`.
pub mod input_story_content_limit {
    /// 10 MB (same as `file_size_limit::PHOTOSIZE_UPLOAD`).
    pub const PHOTOSIZE_UPLOAD: u64 = 10_000_000;
    pub const PHOTO_WIDTH: u32 = 1080;
    pub const PHOTO_HEIGHT: u32 = 1920;
    /// 30 MB.
    pub const VIDEOSIZE_UPLOAD: u64 = 30_000_000;
    pub const VIDEO_WIDTH: u32 = 720;
    pub const VIDEO_HEIGHT: u32 = 1080;
    /// 60 seconds.
    pub const MAX_VIDEO_DURATION: u32 = 60;
}

/// Limitations for `InlineQuery` / `Bot::answer_inline_query`.
pub mod inline_query_limit {
    pub const RESULTS: u32 = 50;
    pub const MAX_OFFSET_LENGTH: u32 = 64;
    pub const MAX_QUERY_LENGTH: u32 = 256;
}

/// Limitations for `InlineQueryResult` and its subclasses.
pub mod inline_query_result_limit {
    pub const MIN_ID_LENGTH: u32 = 1;
    pub const MAX_ID_LENGTH: u32 = 64;
}

/// Limitations for `InlineQueryResultsButton`.
pub mod inline_query_results_button_limit {
    pub const MIN_START_PARAMETER_LENGTH: u32 = 1;
    pub const MAX_START_PARAMETER_LENGTH: u32 = 64;
}

/// Limitations for `Invoice` / `Bot::send_invoice` / `Bot::create_invoice_link`.
pub mod invoice_limit {
    pub const MIN_TITLE_LENGTH: u32 = 1;
    pub const MAX_TITLE_LENGTH: u32 = 32;
    pub const MIN_DESCRIPTION_LENGTH: u32 = 1;
    pub const MAX_DESCRIPTION_LENGTH: u32 = 255;
    pub const MIN_PAYLOAD_LENGTH: u32 = 1;
    pub const MAX_PAYLOAD_LENGTH: u32 = 128;
    pub const MAX_TIP_AMOUNTS: u32 = 4;
    pub const MIN_STAR_COUNT: u32 = 1;
    pub const MAX_STAR_COUNT: u32 = 25000;
    /// 30 days in seconds.
    pub const SUBSCRIPTION_PERIOD: f64 = 2_592_000.0;
    pub const SUBSCRIPTION_MAX_PRICE: u32 = 10000;
}

/// Limitations for `KeyboardButtonRequestUsers`.
pub mod keyboard_button_request_users_limit {
    pub const MIN_QUANTITY: u32 = 1;
    pub const MAX_QUANTITY: u32 = 10;
}

/// Limitations for `Location` / `ChatLocation` / `Bot::send_location`.
pub mod location_limit {
    pub const MIN_CHAT_LOCATION_ADDRESS: u32 = 1;
    pub const MAX_CHAT_LOCATION_ADDRESS: u32 = 64;
    pub const HORIZONTAL_ACCURACY: u32 = 1500;
    pub const MIN_HEADING: u32 = 1;
    pub const MAX_HEADING: u32 = 360;
    pub const MIN_LIVE_PERIOD: u32 = 60;
    pub const MAX_LIVE_PERIOD: u32 = 86400;
    /// `0x7FFFFFFF` -- edit indefinitely.
    pub const LIVE_PERIOD_FOREVER: u32 = 0x7FFF_FFFF;
    pub const MIN_PROXIMITY_ALERT_RADIUS: u32 = 1;
    pub const MAX_PROXIMITY_ALERT_RADIUS: u32 = 100_000;
}

/// Limitations for `Bot::send_media_group`.
pub mod media_group_limit {
    pub const MIN_MEDIA_LENGTH: u32 = 2;
    pub const MAX_MEDIA_LENGTH: u32 = 10;
}

/// Limitations for `Message` / `InputTextMessageContent` / `Bot::send_message`.
pub mod message_limit {
    pub const MAX_TEXT_LENGTH: u32 = 4096;
    pub const CAPTION_LENGTH: u32 = 1024;
    pub const MIN_TEXT_LENGTH: u32 = 1;
    pub const DEEP_LINK_LENGTH: u32 = 64;
    pub const MESSAGE_ENTITIES: u32 = 100;
}

/// Nanostar value constant.
pub mod nanostar {
    pub const VALUE: f64 = 1.0 / 1_000_000_000.0;
}

/// Limitations for nanostar amounts.
pub mod nanostar_limit {
    pub const MIN_AMOUNT: i64 = -999_999_999;
    pub const MAX_AMOUNT: i64 = 999_999_999;
}

/// Limitations for `Bot::get_updates.limit`.
pub mod polling_limit {
    pub const MIN_LIMIT: u32 = 1;
    pub const MAX_LIMIT: u32 = 100;
}

/// Limitations for `Poll` / `PollOption` / `Bot::send_poll`.
pub mod poll_limit {
    pub const MIN_QUESTION_LENGTH: u32 = 1;
    pub const MAX_QUESTION_LENGTH: u32 = 300;
    pub const MIN_OPTION_LENGTH: u32 = 1;
    pub const MAX_OPTION_LENGTH: u32 = 100;
    pub const MIN_OPTION_NUMBER: u32 = 2;
    pub const MAX_OPTION_NUMBER: u32 = 12;
    pub const MAX_EXPLANATION_LENGTH: u32 = 200;
    pub const MAX_EXPLANATION_LINE_FEEDS: u32 = 2;
    pub const MIN_OPEN_PERIOD: u32 = 5;
    pub const MAX_OPEN_PERIOD: u32 = 600;
}

/// Limitations for `Bot::gift_premium_subscription`.
pub mod premium_subscription {
    pub const MAX_TEXT_LENGTH: u32 = 128;
    pub const MONTH_COUNT_THREE: u32 = 3;
    pub const MONTH_COUNT_SIX: u32 = 6;
    pub const MONTH_COUNT_TWELVE: u32 = 12;
    pub const STARS_THREE_MONTHS: u32 = 1000;
    pub const STARS_SIX_MONTHS: u32 = 1500;
    pub const STARS_TWELVE_MONTHS: u32 = 2500;
}

/// Limitations for `ForceReply` and `ReplyKeyboardMarkup`.
pub mod reply_limit {
    pub const MIN_INPUT_FIELD_PLACEHOLDER: u32 = 1;
    pub const MAX_INPUT_FIELD_PLACEHOLDER: u32 = 64;
}

/// Limitations for `Bot::get_star_transactions`.
pub mod star_transactions_limit {
    pub const MIN_LIMIT: u32 = 1;
    pub const MAX_LIMIT: u32 = 100;
}

/// Limitations for various sticker methods.
pub mod sticker_limit {
    pub const MIN_NAME_AND_TITLE: u32 = 1;
    pub const MAX_NAME_AND_TITLE: u32 = 64;
    pub const MIN_STICKER_EMOJI: u32 = 1;
    pub const MAX_STICKER_EMOJI: u32 = 20;
    pub const MAX_SEARCH_KEYWORDS: u32 = 20;
    pub const MAX_KEYWORD_LENGTH: u32 = 64;
}

/// Limitations for sticker set methods.
pub mod sticker_set_limit {
    pub const MIN_INITIAL_STICKERS: u32 = 1;
    pub const MAX_INITIAL_STICKERS: u32 = 50;
    pub const MAX_EMOJI_STICKERS: u32 = 200;
    pub const MAX_ANIMATED_STICKERS: u32 = 50;
    pub const MAX_STATIC_STICKERS: u32 = 120;
    pub const MAX_STATIC_THUMBNAIL_SIZE: u32 = 128;
    pub const MAX_ANIMATED_THUMBNAIL_SIZE: u32 = 32;
    pub const STATIC_THUMB_DIMENSIONS: u32 = 100;
}

/// Limitations for `StoryAreaPosition`.
pub mod story_area_position_limit {
    pub const MAX_ROTATION_ANGLE: u32 = 360;
}

/// Limitations for subclasses of `StoryAreaType`.
pub mod story_area_type_limit {
    pub const MAX_LOCATION_AREAS: u32 = 10;
    pub const MAX_SUGGESTED_REACTION_AREAS: u32 = 5;
    pub const MAX_LINK_AREAS: u32 = 3;
    pub const MAX_WEATHER_AREAS: u32 = 3;
    pub const MAX_UNIQUE_GIFT_AREAS: u32 = 1;
}

/// Limitations for `Bot::post_story` and `Bot::edit_story`.
pub mod story_limit {
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
    pub const MIN_PRICE_STARS: u32 = 5;
    pub const MAX_PRICE_STARS: u32 = 100_000;
    pub const MIN_PRICE_NANOTONCOINS: u64 = 10_000_000;
    pub const MAX_PRICE_NANOTONCOINS: u64 = 10_000_000_000_000;
    pub const MIN_SEND_DATE: u32 = 300;
    pub const MAX_SEND_DATE: u32 = 2_678_400;
    pub const MAX_COMMENT_LENGTH: u32 = 128;
}

/// Limitations for `Bot::get_user_profile_photos.limit`.
pub mod user_profile_photos_limit {
    pub const MIN_LIMIT: u32 = 1;
    pub const MAX_LIMIT: u32 = 100;
}

/// Limitations for `Bot::get_user_profile_audios.limit`.
pub mod user_profile_audios_limit {
    pub const MIN_LIMIT: u32 = 1;
    pub const MAX_LIMIT: u32 = 100;
}

/// Limitations for `Bot::set_webhook`.
pub mod webhook_limit {
    pub const MIN_CONNECTIONS_LIMIT: u32 = 1;
    pub const MAX_CONNECTIONS_LIMIT: u32 = 100;
    pub const MIN_SECRET_TOKEN_LENGTH: u32 = 1;
    pub const MAX_SECRET_TOKEN_LENGTH: u32 = 256;
}

/// Limitations for `Bot::verify_chat` and `Bot::verify_user`.
pub mod verify_limit {
    pub const MAX_TEXT_LENGTH: u32 = 70;
}

/// Limitations for `Bot::set_chat_member_tag`.
pub mod tag_limit {
    pub const MAX_TAG_LENGTH: u32 = 16;
}
