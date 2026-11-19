use serde::{Deserialize, Serialize};

use super::chat::Chat;
use super::ton_wallet::TonWalletCard;
use super::user::User;

// ---------------------------------------------------------------------------
// WidgetQuery
// ---------------------------------------------------------------------------

/// An incoming query from a widget in a message sent by the bot or via the bot.
///
/// Answer it with [`Bot::answer_widget_query`](crate::bot::Bot::answer_widget_query). Each
/// user can send at most 5 queries per 5 seconds to the bot from all its widgets.
///
/// Corresponds to the Bot API [`WidgetQuery`](https://core.telegram.org/bots/api#widgetquery) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WidgetQuery {
    /// Unique identifier for this query.
    pub id: String,

    /// Sender of the query.
    pub from: User,

    /// Chat with the message containing the widget; omitted if the widget is in an inline message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat: Option<Chat>,

    /// Identifier of the message containing the widget; omitted if the widget is in an inline
    /// message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<i64>,

    /// Identifier of the inline message containing the widget.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inline_message_id: Option<String>,

    /// Global identifier, uniquely corresponding to the chat to which the message with the widget
    /// was sent.
    pub chat_instance: String,

    /// For widgets in inline messages, the type of the chat to which the message was sent:
    /// `sender` for a private chat with the inline query sender, `private`, `group`,
    /// `supergroup`, or `channel`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chat_type: Option<String>,

    /// Type of the query.
    #[serde(rename = "type")]
    pub query_type: WidgetQueryType,

    /// Data of the query, 1-64 bytes.
    pub data: String,

    /// Unique identifier of the shared widget which sent the query; omitted if the widget isn't
    /// shared.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub widget_id: Option<String>,
}

// ---------------------------------------------------------------------------
// SharedWidgetQuery
// ---------------------------------------------------------------------------

/// An incoming query from a forwarded message with a shared widget created by the bot.
///
/// Answer it with [`Bot::answer_widget_query`](crate::bot::Bot::answer_widget_query). Each
/// user can send at most 5 queries per 5 seconds to the bot from all its widgets.
///
/// Corresponds to the Bot API
/// [`SharedWidgetQuery`](https://core.telegram.org/bots/api#sharedwidgetquery) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct SharedWidgetQuery {
    /// Unique identifier for this query.
    pub id: String,

    /// Sender of the query.
    pub from: User,

    /// Type of the query.
    #[serde(rename = "type")]
    pub query_type: WidgetQueryType,

    /// Data of the query, 1-64 bytes.
    pub data: String,

    /// Unique identifier of the shared widget which sent the query.
    pub widget_id: String,
}

// ---------------------------------------------------------------------------
// WidgetQueryType
// ---------------------------------------------------------------------------

/// Payload of a `WidgetQueryType` describing a query that contains data for the bot.
///
/// Corresponds to the Bot API
/// [`WidgetQueryTypeData`](https://core.telegram.org/bots/api#widgetquerytypedata) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct WidgetQueryTypeData {
    /// TON wallet card of the user who sent the query, if it was sent to the bot along with the
    /// query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_card: Option<TonWalletCard>,
}

/// Payload of a `WidgetQueryType` describing a query that contains data signed by a TON wallet.
///
/// The data is signed for the domain `bot<bot_id>`, where `<bot_id>` is the identifier of the
/// bot.
///
/// Corresponds to the Bot API
/// [`WidgetQueryTypeSignData`](https://core.telegram.org/bots/api#widgetquerytypesigndata) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WidgetQueryTypeSignData {
    /// The wallet that signed the data.
    pub wallet: WidgetQueryWallet,

    /// JSON-serialized signed data.
    pub sign_data: String,
}

/// Payload of a `WidgetQueryType` describing a query that notifies the bot about a transaction
/// sent from a TON wallet.
///
/// Corresponds to the Bot API
/// [`WidgetQueryTypeSentTransaction`](https://core.telegram.org/bots/api#widgetquerytypesenttransaction) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub struct WidgetQueryTypeSentTransaction {
    /// The wallet from which the transaction was sent.
    pub wallet: WidgetQueryWallet,

    /// Value of `msg_hash` of the transaction.
    pub msg_hash: String,
}

/// Describes the type of a query from a widget.
///
/// Serialized with a `"type"` tag that selects the variant.
///
/// Corresponds to the Bot API
/// [`WidgetQueryType`](https://core.telegram.org/bots/api#widgetquerytype) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
#[non_exhaustive]
pub enum WidgetQueryType {
    /// The query contains data for the bot.
    Data(WidgetQueryTypeData),

    /// The query contains data signed by a TON wallet of the user.
    SignData(WidgetQueryTypeSignData),

    /// The query notifies the bot about a transaction sent from a TON wallet of the user.
    SentTransaction(WidgetQueryTypeSentTransaction),
}

impl WidgetQueryType {
    /// Creates a query type that carries data for the bot.
    pub fn data() -> Self {
        Self::Data(WidgetQueryTypeData::default())
    }

    /// Creates a query type that carries data signed by a TON wallet.
    pub fn sign_data(wallet: WidgetQueryWallet, sign_data: impl Into<String>) -> Self {
        Self::SignData(WidgetQueryTypeSignData {
            wallet,
            sign_data: sign_data.into(),
        })
    }

    /// Creates a query type that notifies about a sent transaction.
    pub fn sent_transaction(wallet: WidgetQueryWallet, msg_hash: impl Into<String>) -> Self {
        Self::SentTransaction(WidgetQueryTypeSentTransaction {
            wallet,
            msg_hash: msg_hash.into(),
        })
    }
}

// ---------------------------------------------------------------------------
// WidgetQueryWallet
// ---------------------------------------------------------------------------

/// A TON wallet of the user, which is used in a query from a widget.
///
/// Corresponds to the Bot API
/// [`WidgetQueryWallet`](https://core.telegram.org/bots/api#widgetquerywallet) object.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
#[non_exhaustive]
pub struct WidgetQueryWallet {
    /// Address of the wallet.
    pub address: String,

    /// Balance of the wallet, in the *smallest units* of the cryptocurrency, as a decimal number.
    pub amount: String,

    /// Hex-encoded public key of the wallet; omitted if unknown.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key: Option<String>,

    /// Base64-encoded serialized StateInit of the wallet contract; omitted if and only if
    /// `public_key` is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_init: Option<String>,
}

impl_new!(WidgetQueryWallet {
    address: String,
    amount: String
});
