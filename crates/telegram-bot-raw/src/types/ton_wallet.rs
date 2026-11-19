use serde::{Deserialize, Serialize};

use super::story_area::AreaPosition;
use super::unique_gift::UniqueGift;

// ---------------------------------------------------------------------------
// TonWalletCard
// ---------------------------------------------------------------------------

/// A TON wallet card, which is shown on the profile page of a user to everyone.
///
/// Corresponds to the Bot API
/// [`TonWalletCard`](https://core.telegram.org/bots/api#tonwalletcard) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct TonWalletCard {
    /// Address of the wallet.
    pub address: String,

    /// Balance of the wallet, in the *smallest units* of the cryptocurrency, as a decimal number.
    pub amount: String,

    /// Stickers placed on the card.
    pub stickers: Vec<TonWalletCardSticker>,

    /// Address of the NFT of one of the stickers, whose colors must be used for the card
    /// background; omitted if the default background must be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_nft_address: Option<String>,
}

impl_new!(TonWalletCard {
    address: String,
    amount: String
});

// ---------------------------------------------------------------------------
// TonWalletCardSticker
// ---------------------------------------------------------------------------

/// A sticker on a TON wallet card.
///
/// Corresponds to the Bot API
/// [`TonWalletCardSticker`](https://core.telegram.org/bots/api#tonwalletcardsticker) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct TonWalletCardSticker {
    /// Position of the sticker on the card.
    pub position: AreaPosition,

    /// The NFT shown as the sticker.
    pub nft: TonNft,
}

// ---------------------------------------------------------------------------
// TonNft
// ---------------------------------------------------------------------------

/// An NFT in the TON blockchain.
///
/// Corresponds to the Bot API [`TonNft`](https://core.telegram.org/bots/api#tonnft) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct TonNft {
    /// Address of the NFT.
    pub address: String,

    /// Address of the collection to which the NFT belongs; omitted if unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_address: Option<String>,

    /// Index of the NFT among all NFTs from the collection, as a decimal number.
    pub index: String,

    /// Name of the NFT.
    pub name: String,

    /// Description of the NFT.
    pub description: String,

    /// Attributes of the NFT.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<TonNftAttribute>>,

    /// The unique gift corresponding to the NFT.
    ///
    /// Boxed: a `UniqueGift` carries three nested artwork models and is over 1 KiB, while
    /// this field is almost never present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gift: Option<Box<UniqueGift>>,
}

impl_new!(TonNft {
    address: String,
    index: String,
    name: String,
    description: String
});

// ---------------------------------------------------------------------------
// TonNftAttribute
// ---------------------------------------------------------------------------

/// An attribute of an NFT in the TON blockchain.
///
/// Corresponds to the Bot API
/// [`TonNftAttribute`](https://core.telegram.org/bots/api#tonnftattribute) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct TonNftAttribute {
    /// Trait type of the attribute.
    pub trait_type: String,

    /// Value of the attribute.
    pub value: String,
}

impl_new!(TonNftAttribute {
    trait_type: String,
    value: String
});

// ---------------------------------------------------------------------------
// TonWalletTransfer
// ---------------------------------------------------------------------------

/// Describes a service message about a transfer with a TON wallet.
///
/// Corresponds to the Bot API
/// [`TonWalletTransfer`](https://core.telegram.org/bots/api#tonwallettransfer) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct TonWalletTransfer {
    /// Unique identifier of the transaction; may be empty if the transfer hasn't been sent yet.
    pub transaction_id: String,

    /// Address of the transfer peer in the TON blockchain.
    pub peer_address: String,

    /// The amount of transferred cryptocurrency, in the *smallest units* of the cryptocurrency,
    /// as a decimal number; negative for outgoing transfers.
    pub amount: String,

    /// Comment added to the transfer; omitted if the comment is encrypted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}
