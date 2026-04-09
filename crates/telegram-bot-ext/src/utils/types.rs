//! Internal type aliases used across the extension crate.
//!
//! These correspond to the Python `telegram.ext._utils.types` module.
//! They are library-internal and not part of the public API stability guarantee.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

// ---------------------------------------------------------------------------
// Conversation types
// ---------------------------------------------------------------------------

/// A conversation key: a tuple of user/chat IDs and optional string identifiers
/// serialised as a `Vec` because Rust tuples are fixed-length.
pub type ConversationKey = Vec<ConversationKeyPart>;

/// A single element inside a [`ConversationKey`].
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ConversationKeyPart {
    Int(i64),
    Str(String),
}

/// The state map maintained by a `ConversationHandler`.
pub type ConversationDict = HashMap<ConversationKey, Option<Value>>;

// ---------------------------------------------------------------------------
// Callback-data cache types
// ---------------------------------------------------------------------------

/// A single entry in the callback-data cache:
/// `(callback_uuid, timestamp, keyboard_data)`.
pub type CallbackDataEntry = (String, f64, HashMap<String, Value>);

/// The full callback-data cache payload as persisted.
/// `(list_of_entries, uuid_to_callback_data_mapping)`.
pub type CdcData = (Vec<CallbackDataEntry>, HashMap<String, String>);

// ---------------------------------------------------------------------------
// Persistence data map
// ---------------------------------------------------------------------------

/// Convenience alias for the JSON-like maps used as user/chat/bot data.
pub type JsonMap = HashMap<String, Value>;

// ---------------------------------------------------------------------------
// Filter data
// ---------------------------------------------------------------------------

/// Data extracted by a filter for downstream handler consumption.
pub type FilterDataDict = HashMap<String, Vec<Value>>;

// ---------------------------------------------------------------------------
// Job callback
// ---------------------------------------------------------------------------

/// A boxed, `Send + Sync` future that resolves to `()`.
pub type BoxFuture<'a> = Pin<Box<dyn Future<Output = ()> + Send + 'a>>;

/// The signature of a job callback: receives *something* (the context) and
/// returns a future. Concrete context types are supplied at the application
/// level; here we erase them behind `Value` to keep the utils crate agnostic.
pub type JobCallback = Arc<dyn Fn(Value) -> BoxFuture<'static> + Send + Sync>;

// ---------------------------------------------------------------------------
// Rate-limiter argument
// ---------------------------------------------------------------------------

/// Opaque payload that a caller can attach to a rate-limited request.
/// Mirrors Python's `RLARGS` type-variable.
pub type RateLimitArgs = Value;

// ---------------------------------------------------------------------------
// Handler callback (type-erased)
// ---------------------------------------------------------------------------

/// Type-erased handler callback: `async fn(update, context) -> RT`.
pub type HandlerCallback =
    Arc<dyn Fn(Value, Value) -> Pin<Box<dyn Future<Output = Value> + Send>> + Send + Sync>;
