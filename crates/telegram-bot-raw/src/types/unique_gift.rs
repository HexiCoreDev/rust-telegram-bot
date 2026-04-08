use serde::{Deserialize, Serialize};

use super::chat::Chat;
use super::files::sticker::Sticker;

// ---------------------------------------------------------------------------
// UniqueGiftColors
// ---------------------------------------------------------------------------

/// Color scheme for a user's name, message replies, and link previews based on a unique gift.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniqueGiftColors {
    /// Custom emoji identifier of the unique gift's model.
    pub model_custom_emoji_id: String,

    /// Custom emoji identifier of the unique gift's symbol.
    pub symbol_custom_emoji_id: String,

    /// Main color used in light themes; RGB format.
    pub light_theme_main_color: i64,

    /// List of 1–3 additional colors used in light themes; RGB format.
    pub light_theme_other_colors: Vec<i64>,

    /// Main color used in dark themes; RGB format.
    pub dark_theme_main_color: i64,

    /// List of 1–3 additional colors used in dark themes; RGB format.
    pub dark_theme_other_colors: Vec<i64>,
}

// ---------------------------------------------------------------------------
// UniqueGiftModel
// ---------------------------------------------------------------------------

/// Model of a unique gift.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniqueGiftModel {
    /// Name of the model.
    pub name: String,

    /// Sticker that represents the unique gift.
    pub sticker: Sticker,

    /// Number of unique gifts receiving this model per 1000 upgrades.
    pub rarity_per_mille: i64,

    /// Rarity of the model if it is a crafted model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rarity: Option<String>,
}

// ---------------------------------------------------------------------------
// UniqueGiftSymbol
// ---------------------------------------------------------------------------

/// Symbol shown on the pattern of a unique gift.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniqueGiftSymbol {
    /// Name of the symbol.
    pub name: String,

    /// Sticker that represents the unique gift.
    pub sticker: Sticker,

    /// Number of unique gifts receiving this symbol per 1000 upgrades.
    pub rarity_per_mille: i64,
}

// ---------------------------------------------------------------------------
// UniqueGiftBackdropColors
// ---------------------------------------------------------------------------

/// Colors of the backdrop of a unique gift.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniqueGiftBackdropColors {
    /// Center color of the backdrop in RGB format.
    pub center_color: i64,

    /// Edge color of the backdrop in RGB format.
    pub edge_color: i64,

    /// Color applied to the symbol in RGB format.
    pub symbol_color: i64,

    /// Text color on the backdrop in RGB format.
    pub text_color: i64,
}

// ---------------------------------------------------------------------------
// UniqueGiftBackdrop
// ---------------------------------------------------------------------------

/// Backdrop of a unique gift.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniqueGiftBackdrop {
    /// Name of the backdrop.
    pub name: String,

    /// Colors of the backdrop.
    pub colors: UniqueGiftBackdropColors,

    /// Number of unique gifts receiving this backdrop per 1000 upgrades.
    pub rarity_per_mille: i64,
}

// ---------------------------------------------------------------------------
// UniqueGift
// ---------------------------------------------------------------------------

/// A unique gift that was upgraded from a regular gift.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniqueGift {
    /// Identifier of the regular gift from which this was upgraded.
    pub gift_id: String,

    /// Human-readable name of the original regular gift.
    pub base_name: String,

    /// Unique name usable in `https://t.me/nft/...` links.
    pub name: String,

    /// Unique number among gifts upgraded from the same regular gift.
    pub number: i64,

    /// Model of the gift.
    pub model: UniqueGiftModel,

    /// Symbol of the gift.
    pub symbol: UniqueGiftSymbol,

    /// Backdrop of the gift.
    pub backdrop: UniqueGiftBackdrop,

    /// Chat that published the gift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publisher_chat: Option<Chat>,

    /// True if the original regular gift was exclusively for Telegram Premium subscribers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_premium: Option<bool>,

    /// True if the gift is assigned from the TON blockchain and cannot be resold in Telegram.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_from_blockchain: Option<bool>,

    /// Color scheme available to the gift's owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<UniqueGiftColors>,

    /// True if the gift was used to craft another gift and is no longer available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_burned: Option<bool>,
}

// ---------------------------------------------------------------------------
// UniqueGiftInfo
// ---------------------------------------------------------------------------

/// Service message about a unique gift that was sent or received.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UniqueGiftInfo {
    /// Information about the gift.
    pub gift: UniqueGift,

    /// Origin of the gift: `"upgrade"`, `"transfer"`, `"resale"`, `"gifted_upgrade"`,
    /// or `"offer"`.
    pub origin: String,

    /// Unique identifier of the received gift for the bot; business accounts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owned_gift_id: Option<String>,

    /// Stars required to transfer the gift; absent if transfer is not possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_star_count: Option<i64>,

    /// Unix timestamp when the gift can be transferred next.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_transfer_date: Option<i64>,

    /// Currency of the last resale payment; for resale gifts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_resale_currency: Option<String>,

    /// Amount paid in the last resale; for resale gifts only.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_resale_amount: Option<i64>,
}
