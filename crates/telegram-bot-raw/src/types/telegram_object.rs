use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Extra fields from the API not yet covered by typed struct fields.
///
/// This is the Rust equivalent of python-telegram-bot's `api_kwargs` — it provides
/// forward-compatibility so that when Telegram adds new fields to an API type,
/// existing library versions don't break. Unknown fields land here automatically
/// via `#[serde(flatten)]`.
///
/// ## Pattern for all Telegram types
///
/// Every Telegram API type in this crate follows this pattern:
///
/// ```rust,ignore
/// #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
/// pub struct SomeType {
///     pub some_field: String,
///     pub optional_field: Option<i64>,
///
///     /// Extra fields not yet covered by this struct.
///     #[serde(flatten)]
///     pub extra: HashMap<String, Value>,
/// }
/// ```
///
/// ### Equality & Hashing
///
/// In python-telegram-bot, each type defines `_id_attrs` for equality comparison.
/// In Rust, we implement `PartialEq` either via derive (comparing all fields) or
/// manually when only specific fields should determine identity (e.g., `Message`
/// is identified by `message_id` + `chat.id`, not all 60+ fields).
///
/// For types that need custom equality based on identity attributes,
/// implement `PartialEq` and `Hash` manually rather than deriving.
///
/// ### Immutability
///
/// Rust structs are immutable by default unless you have `&mut` access,
/// which naturally mirrors python-telegram-bot's frozen object pattern.
///
/// ### Bot Reference
///
/// Unlike python-telegram-bot, types do NOT hold a Bot reference.
/// Instead, the Bot is passed explicitly via the handler context.
/// Shortcut methods (like `message.reply_text()`) take `&Bot` as a parameter.
pub type ApiKwargs = HashMap<String, Value>;

/// Trait implemented by all Telegram API types.
///
/// Provides common serialization and identity behavior.
///
/// ### Date/time fields use `i64` (Unix timestamps)
///
/// All date/time parameters in the Telegram Bot API are Unix timestamps
/// represented as integers. This crate uses `i64` directly rather than
/// a wrapper type (`chrono::DateTime`, `std::time::SystemTime`, etc.) for
/// the following reasons:
///
/// 1. **Zero-cost at the API boundary**: No conversion overhead when
///    serialising/deserialising — the JSON integer maps directly to `i64`.
/// 2. **Raw API layer contract**: This crate is an intentionally thin
///    translation of the Telegram Bot API schema. Higher-level libraries
///    built on top can apply their own date/time abstractions.
/// 3. **Avoid optional dependency on `chrono` or `time`**: Keeping the
///    type as `i64` avoids forcing consumers into a specific date/time
///    crate.
/// 4. **Cross-platform portability**: `i64` works identically everywhere;
///    `std::time::SystemTime` has platform-specific epoch semantics.
pub trait TelegramObject: Serialize + for<'de> Deserialize<'de> + Send + Sync {
    /// Returns the extra/unknown fields from the API response.
    fn api_kwargs(&self) -> &ApiKwargs;

    /// Serialises this object to a JSON string.
    fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(self)
    }

    /// Serialises this object to a `serde_json::Value` (equivalent to `to_dict`).
    fn to_dict(&self) -> serde_json::Result<Value> {
        serde_json::to_value(self)
    }
}
