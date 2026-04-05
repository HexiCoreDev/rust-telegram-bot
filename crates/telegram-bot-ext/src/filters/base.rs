//! Core [`Filter`] trait and the [`F`] wrapper for bitwise composition.
//!
//! Every filter in the crate implements [`Filter`].  The [`F`] new-type wraps an
//! `Arc`-backed trait object and provides `&`, `|`, `^`, `!` operators so filters
//! can be combined with zero boilerplate:
//!
//! ```ignore
//! let f = F::from(Text::any()) & !F::from(Command::starts());
//! ```

use std::collections::HashMap;
use std::fmt;
use std::ops::{BitAnd, BitOr, BitXor, Not};
use std::sync::Arc;

use serde_json::Value;

// ---------------------------------------------------------------------------
// Update alias
// ---------------------------------------------------------------------------

/// The canonical typed `Update` from the raw Telegram types.
pub type Update = telegram_bot_raw::types::update::Update;

// ---------------------------------------------------------------------------
// Value bridge for filters
// ---------------------------------------------------------------------------

/// Convert a typed [`Update`] reference to a [`serde_json::Value`].
///
/// Filters that still operate on JSON field access call this internally.
/// Individual filters can be migrated to typed access incrementally.
pub fn to_value(update: &Update) -> Value {
    serde_json::to_value(update).unwrap_or_default()
}

// ---------------------------------------------------------------------------
// FilterResult
// ---------------------------------------------------------------------------

/// Result of a filter check, supporting data filters (like Python's `data_filter`).
///
/// Simple filters return [`Match`](FilterResult::Match) or [`NoMatch`](FilterResult::NoMatch).
/// Data filters (e.g. regex) return [`MatchWithData`](FilterResult::MatchWithData) carrying
/// named capture groups or other extracted data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterResult {
    /// The filter did not match.
    NoMatch,
    /// The filter matched but produced no additional data.
    Match,
    /// The filter matched and produced named capture data (e.g. regex groups).
    MatchWithData(HashMap<String, Vec<String>>),
}

impl FilterResult {
    /// Returns `true` when the filter matched (either `Match` or `MatchWithData`).
    #[must_use]
    pub fn is_match(&self) -> bool {
        !matches!(self, FilterResult::NoMatch)
    }

    /// Merge data from two `FilterResult` values. Used by [`AndFilter`].
    ///
    /// - If either side is `NoMatch`, the result is `NoMatch`.
    /// - If both carry data, their maps are merged (values appended).
    /// - If only one carries data, that data is preserved.
    #[must_use]
    pub fn merge(self, other: FilterResult) -> FilterResult {
        match (self, other) {
            (FilterResult::NoMatch, _) | (_, FilterResult::NoMatch) => FilterResult::NoMatch,
            (FilterResult::MatchWithData(mut a), FilterResult::MatchWithData(b)) => {
                for (k, mut v) in b {
                    a.entry(k).or_default().append(&mut v);
                }
                FilterResult::MatchWithData(a)
            }
            (FilterResult::MatchWithData(d), FilterResult::Match)
            | (FilterResult::Match, FilterResult::MatchWithData(d)) => {
                FilterResult::MatchWithData(d)
            }
            (FilterResult::Match, FilterResult::Match) => FilterResult::Match,
        }
    }
}

// ---------------------------------------------------------------------------
// Value-based extraction helpers (for filter submodules)
// ---------------------------------------------------------------------------

/// Extract the effective message from a raw [`Value`] (reference-based).
///
/// Filter submodules call `to_value(update)` once at the top of
/// `check_update`, then pass the resulting `&Value` here.
pub fn effective_message_val(v: &Value) -> Option<&Value> {
    v.get("message")
        .or_else(|| v.get("edited_message"))
        .or_else(|| v.get("channel_post"))
        .or_else(|| v.get("edited_channel_post"))
        .or_else(|| v.get("business_message"))
        .or_else(|| v.get("edited_business_message"))
}

/// Extract the effective user from a raw [`Value`] (reference-based).
pub fn effective_user_val(v: &Value) -> Option<&Value> {
    if let Some(msg) = effective_message_val(v) {
        if let Some(u) = msg.get("from") {
            return Some(u);
        }
    }
    for key in &[
        "callback_query", "inline_query", "chosen_inline_result",
        "shipping_query", "pre_checkout_query", "poll_answer",
        "my_chat_member", "chat_member", "chat_join_request",
    ] {
        if let Some(obj) = v.get(key) {
            if let Some(u) = obj.get("from") {
                return Some(u);
            }
        }
    }
    None
}

/// Extract the effective chat from a raw [`Value`] (reference-based).
pub fn effective_chat_val(v: &Value) -> Option<&Value> {
    if let Some(msg) = effective_message_val(v) {
        if let Some(c) = msg.get("chat") {
            return Some(c);
        }
    }
    for key in &[
        "callback_query", "my_chat_member",
        "chat_member", "chat_join_request",
    ] {
        if let Some(obj) = v.get(key) {
            if let Some(c) = obj.get("chat") {
                return Some(c);
            }
        }
    }
    None
}

// ---------------------------------------------------------------------------
// Message / user / chat extraction helpers
// ---------------------------------------------------------------------------

/// Best-effort extraction of the *effective message* from an [`Update`] as a
/// [`Value`] for filters that still operate on JSON field access.
///
/// Checks, in order: `message`, `edited_message`, `channel_post`,
/// `edited_channel_post`, `business_message`, `edited_business_message`.
pub fn effective_message(update: &Update) -> Option<Value> {
    let v = to_value(update);
    v.get("message")
        .or_else(|| v.get("edited_message"))
        .or_else(|| v.get("channel_post"))
        .or_else(|| v.get("edited_channel_post"))
        .or_else(|| v.get("business_message"))
        .or_else(|| v.get("edited_business_message"))
        .cloned()
}

/// Extract the effective user from an [`Update`] as a [`Value`].
pub fn effective_user(update: &Update) -> Option<Value> {
    if let Some(msg) = effective_message(update) {
        if let Some(u) = msg.get("from") {
            return Some(u.clone());
        }
    }
    let v = to_value(update);
    for key in &[
        "callback_query",
        "inline_query",
        "chosen_inline_result",
        "shipping_query",
        "pre_checkout_query",
        "poll_answer",
        "my_chat_member",
        "chat_member",
        "chat_join_request",
    ] {
        if let Some(obj) = v.get(key) {
            if let Some(u) = obj.get("from") {
                return Some(u.clone());
            }
        }
    }
    None
}

/// Extract the effective chat from an [`Update`] as a [`Value`].
pub fn effective_chat(update: &Update) -> Option<Value> {
    if let Some(msg) = effective_message(update) {
        if let Some(c) = msg.get("chat") {
            return Some(c.clone());
        }
    }
    let v = to_value(update);
    for key in &[
        "callback_query",
        "my_chat_member",
        "chat_member",
        "chat_join_request",
    ] {
        if let Some(obj) = v.get(key) {
            if let Some(c) = obj.get("chat") {
                return Some(c.clone());
            }
        }
    }
    None
}

/// Returns `true` when the update carries a message-like field.
pub fn has_effective_message(update: &Update) -> bool {
    effective_message(update).is_some()
}

// ---------------------------------------------------------------------------
// Filter trait
// ---------------------------------------------------------------------------

/// The single trait every filter must implement.
///
/// Returns a [`FilterResult`] which can be `NoMatch`, `Match`, or
/// `MatchWithData` (for data filters like regex).
pub trait Filter: Send + Sync + 'static {
    /// Check whether the update should be handled, optionally returning
    /// extracted data.
    fn check_update(&self, update: &Update) -> FilterResult;

    /// Human-readable name for debugging / display.
    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}


// ---------------------------------------------------------------------------
// F wrapper
// ---------------------------------------------------------------------------

/// Ergonomic wrapper around `Arc<dyn Filter>` with operator overloads.
///
/// `F` is `Clone`-able: cloning shares the underlying filter via `Arc`.
/// This allows filter constants (like `TEXT`, `COMMAND`) to be used with
/// the bitwise composition operators (`&`, `|`, `!`) without consuming them.
#[derive(Clone)]
pub struct F(pub Arc<dyn Filter>);

impl F {
    /// Wrap any concrete filter into [`F`].
    pub fn new(filter: impl Filter) -> Self {
        Self(Arc::new(filter))
    }
}

impl fmt::Debug for F {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "F({})", self.0.name())
    }
}

impl fmt::Display for F {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.name())
    }
}

impl Filter for F {
    fn check_update(&self, update: &Update) -> FilterResult {
        self.0.check_update(update)
    }

    fn name(&self) -> &str {
        self.0.name()
    }
}


// ---------------------------------------------------------------------------
// Combinators (private structs)
// ---------------------------------------------------------------------------

struct AndFilter {
    left: F,
    right: F,
    display: String,
}

impl Filter for AndFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let left = self.left.check_update(update);
        if !left.is_match() {
            return FilterResult::NoMatch;
        }
        let right = self.right.check_update(update);
        left.merge(right)
    }

    fn name(&self) -> &str {
        &self.display
    }
}

struct OrFilter {
    left: F,
    right: F,
    display: String,
}

impl Filter for OrFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let left = self.left.check_update(update);
        if left.is_match() {
            return left;
        }
        self.right.check_update(update)
    }

    fn name(&self) -> &str {
        &self.display
    }
}

struct XorFilter {
    left: F,
    right: F,
    display: String,
}

impl Filter for XorFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let left = self.left.check_update(update);
        let right = self.right.check_update(update);
        match (left.is_match(), right.is_match()) {
            (true, false) => self.left.check_update(update),
            (false, true) => right,
            _ => FilterResult::NoMatch,
        }
    }

    fn name(&self) -> &str {
        &self.display
    }
}

struct NotFilter {
    inner: F,
    display: String,
}

impl Filter for NotFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        if self.inner.check_update(update).is_match() {
            FilterResult::NoMatch
        } else {
            FilterResult::Match
        }
    }

    fn name(&self) -> &str {
        &self.display
    }
}

// ---------------------------------------------------------------------------
// Operator impls
// ---------------------------------------------------------------------------

impl BitAnd for F {
    type Output = F;

    fn bitand(self, rhs: F) -> F {
        let display = format!("<{} and {}>", self.0.name(), rhs.0.name());
        F(Arc::new(AndFilter {
            left: self,
            right: rhs,
            display,
        }))
    }
}

impl BitOr for F {
    type Output = F;

    fn bitor(self, rhs: F) -> F {
        let display = format!("<{} or {}>", self.0.name(), rhs.0.name());
        F(Arc::new(OrFilter {
            left: self,
            right: rhs,
            display,
        }))
    }
}

impl BitXor for F {
    type Output = F;

    fn bitxor(self, rhs: F) -> F {
        let display = format!("<{} xor {}>", self.0.name(), rhs.0.name());
        F(Arc::new(XorFilter {
            left: self,
            right: rhs,
            display,
        }))
    }
}

impl Not for F {
    type Output = F;

    fn not(self) -> F {
        let display = format!("<not {}>", self.0.name());
        F(Arc::new(NotFilter {
            inner: self,
            display,
        }))
    }
}

// ---------------------------------------------------------------------------
// Closure-backed filter
// ---------------------------------------------------------------------------

/// A filter built from a bare closure. Useful for one-off / ad-hoc filters.
pub struct FnFilter<Func> {
    func: Func,
    label: &'static str,
}

impl<Func> FnFilter<Func>
where
    Func: Fn(&Update) -> bool + Send + Sync + 'static,
{
    /// Create a new closure-backed filter.
    pub fn new(label: &'static str, func: Func) -> Self {
        Self { func, label }
    }
}

impl<Func> Filter for FnFilter<Func>
where
    Func: Fn(&Update) -> bool + Send + Sync + 'static,
{
    fn check_update(&self, update: &Update) -> FilterResult {
        if (self.func)(update) {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        self.label
    }
}

// ---------------------------------------------------------------------------
// ALL filter
// ---------------------------------------------------------------------------

/// Matches every update that carries an effective message.
pub struct All;

impl Filter for All {
    fn check_update(&self, update: &Update) -> FilterResult {
        if has_effective_message(update) {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        "filters.ALL"
    }
}

/// Constant instance -- `filters::ALL`.
pub const ALL: All = All;

// ---------------------------------------------------------------------------
// Macro for simple presence filters
// ---------------------------------------------------------------------------

macro_rules! message_presence_filter {
    (
        $(#[$meta:meta])*
        $struct_name:ident, $field:expr, $display:expr
    ) => {
        $(#[$meta])*
        pub struct $struct_name;

        impl Filter for $struct_name {
            fn check_update(&self, update: &Update) -> FilterResult {
                if effective_message(update)
                    .and_then(|m| m.get($field).cloned())
                    .map(|v| !v.is_null())
                    .unwrap_or(false)
                {
                    FilterResult::Match
                } else {
                    FilterResult::NoMatch
                }
            }

            fn name(&self) -> &str {
                $display
            }
        }
    };
}

// ---------------------------------------------------------------------------
// Presence filter instances
// ---------------------------------------------------------------------------

message_presence_filter!(
    /// Messages containing an animation (GIF).
    AnimationFilter, "animation", "filters.ANIMATION"
);
pub const ANIMATION: AnimationFilter = AnimationFilter;

message_presence_filter!(
    /// Messages containing audio.
    AudioFilter, "audio", "filters.AUDIO"
);
pub const AUDIO: AudioFilter = AudioFilter;

message_presence_filter!(
    /// Messages containing a boost_added notification.
    BoostAdded, "boost_added", "filters.BOOST_ADDED"
);
pub const BOOST_ADDED: BoostAdded = BoostAdded;

message_presence_filter!(
    /// Messages containing a checklist.
    ChecklistFilter, "checklist", "filters.CHECKLIST"
);
pub const CHECKLIST: ChecklistFilter = ChecklistFilter;

message_presence_filter!(
    /// Messages containing a contact.
    ContactFilter, "contact", "filters.CONTACT"
);
pub const CONTACT: ContactFilter = ContactFilter;

message_presence_filter!(
    /// Messages containing an effect_id.
    EffectId, "effect_id", "filters.EFFECT_ID"
);
pub const EFFECT_ID: EffectId = EffectId;

message_presence_filter!(
    /// Messages that have a forward_origin.
    ForwardedPresence, "forward_origin", "filters.FORWARDED"
);
pub const FORWARDED: ForwardedPresence = ForwardedPresence;

message_presence_filter!(
    /// Messages containing a game.
    GameFilter, "game", "filters.GAME"
);
pub const GAME: GameFilter = GameFilter;

message_presence_filter!(
    /// Messages containing a giveaway.
    GiveawayFilter, "giveaway", "filters.GIVEAWAY"
);
pub const GIVEAWAY: GiveawayFilter = GiveawayFilter;

message_presence_filter!(
    /// Messages containing giveaway_winners.
    GiveawayWinners, "giveaway_winners", "filters.GIVEAWAY_WINNERS"
);
pub const GIVEAWAY_WINNERS: GiveawayWinners = GiveawayWinners;

message_presence_filter!(
    /// Messages containing has_media_spoiler.
    HasMediaSpoiler, "has_media_spoiler", "filters.HAS_MEDIA_SPOILER"
);
pub const HAS_MEDIA_SPOILER: HasMediaSpoiler = HasMediaSpoiler;

message_presence_filter!(
    /// Messages containing has_protected_content.
    HasProtectedContent, "has_protected_content", "filters.HAS_PROTECTED_CONTENT"
);
pub const HAS_PROTECTED_CONTENT: HasProtectedContent = HasProtectedContent;

message_presence_filter!(
    /// Messages containing an invoice.
    InvoiceFilter, "invoice", "filters.INVOICE"
);
pub const INVOICE: InvoiceFilter = InvoiceFilter;

message_presence_filter!(
    /// Messages that are automatic forwards.
    IsAutomaticForward, "is_automatic_forward", "filters.IS_AUTOMATIC_FORWARD"
);
pub const IS_AUTOMATIC_FORWARD: IsAutomaticForward = IsAutomaticForward;

message_presence_filter!(
    /// Messages that are topic messages.
    IsTopicMessage, "is_topic_message", "filters.IS_TOPIC_MESSAGE"
);
pub const IS_TOPIC_MESSAGE: IsTopicMessage = IsTopicMessage;

message_presence_filter!(
    /// Messages sent from offline.
    IsFromOffline, "is_from_offline", "filters.IS_FROM_OFFLINE"
);
pub const IS_FROM_OFFLINE: IsFromOffline = IsFromOffline;

message_presence_filter!(
    /// Messages containing a location.
    LocationFilter, "location", "filters.LOCATION"
);
pub const LOCATION: LocationFilter = LocationFilter;

message_presence_filter!(
    /// Messages containing paid media.
    PaidMediaFilter, "paid_media", "filters.PAID_MEDIA"
);
pub const PAID_MEDIA: PaidMediaFilter = PaidMediaFilter;

message_presence_filter!(
    /// Messages containing passport data.
    PassportDataFilter, "passport_data", "filters.PASSPORT_DATA"
);
pub const PASSPORT_DATA: PassportDataFilter = PassportDataFilter;

message_presence_filter!(
    /// Messages containing a poll.
    PollFilter, "poll", "filters.POLL"
);
pub const POLL: PollFilter = PollFilter;

message_presence_filter!(
    /// Messages that are replies.
    ReplyFilter, "reply_to_message", "filters.REPLY"
);
pub const REPLY: ReplyFilter = ReplyFilter;

message_presence_filter!(
    /// Messages that are replies to a story.
    ReplyToStory, "reply_to_story", "filters.REPLY_TO_STORY"
);
pub const REPLY_TO_STORY: ReplyToStory = ReplyToStory;

message_presence_filter!(
    /// Messages with sender_boost_count.
    SenderBoostCount, "sender_boost_count", "filters.SENDER_BOOST_COUNT"
);
pub const SENDER_BOOST_COUNT: SenderBoostCount = SenderBoostCount;

message_presence_filter!(
    /// Messages containing a story.
    StoryFilter, "story", "filters.STORY"
);
pub const STORY: StoryFilter = StoryFilter;

message_presence_filter!(
    /// Messages containing a venue.
    VenueFilter, "venue", "filters.VENUE"
);
pub const VENUE: VenueFilter = VenueFilter;

message_presence_filter!(
    /// Messages containing a video.
    VideoFilter, "video", "filters.VIDEO"
);
pub const VIDEO: VideoFilter = VideoFilter;

message_presence_filter!(
    /// Messages containing a video note.
    VideoNoteFilter, "video_note", "filters.VIDEO_NOTE"
);
pub const VIDEO_NOTE: VideoNoteFilter = VideoNoteFilter;

message_presence_filter!(
    /// Messages containing voice audio.
    VoiceFilter, "voice", "filters.VOICE"
);
pub const VOICE: VoiceFilter = VoiceFilter;

message_presence_filter!(
    /// Messages containing suggested_post_info.
    SuggestedPostInfo, "suggested_post_info", "filters.SUGGESTED_POST_INFO"
);
pub const SUGGESTED_POST_INFO: SuggestedPostInfo = SuggestedPostInfo;

// ---------------------------------------------------------------------------
// Attachment (computed)
// ---------------------------------------------------------------------------

/// Messages containing an effective attachment. Mirrors the computed property
/// from python-telegram-bot.
pub struct AttachmentFilter;

impl Filter for AttachmentFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let msg = match effective_message(update) {
            Some(m) => m,
            None => return FilterResult::NoMatch,
        };
        let has = |key: &str| msg.get(key).map(|v| !v.is_null()).unwrap_or(false);
        let matched = has("animation")
            || has("audio")
            || has("contact")
            || has("dice")
            || has("document")
            || has("game")
            || has("invoice")
            || has("location")
            || has("paid_media")
            || has("passport_data")
            || msg
                .get("photo")
                .and_then(|v| v.as_array())
                .map(|a| !a.is_empty())
                .unwrap_or(false)
            || has("poll")
            || has("sticker")
            || has("story")
            || has("venue")
            || has("video")
            || has("video_note")
            || has("voice");
        if matched {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        "filters.ATTACHMENT"
    }
}

pub const ATTACHMENT: AttachmentFilter = AttachmentFilter;

// ---------------------------------------------------------------------------
// Update-level presence filters
// ---------------------------------------------------------------------------

/// Messages from a forum (topics enabled) chat.
pub struct ForumFilter;

impl Filter for ForumFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        if effective_chat(update)
            .and_then(|c| c.get("is_forum").cloned())
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        "filters.FORUM"
    }
}

pub const FORUM: ForumFilter = ForumFilter;

/// Messages from a direct-messages chat of a channel.
pub struct DirectMessages;

impl Filter for DirectMessages {
    fn check_update(&self, update: &Update) -> FilterResult {
        if effective_chat(update)
            .and_then(|c| c.get("is_direct_messages").cloned())
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        "filters.DIRECT_MESSAGES"
    }
}

pub const DIRECT_MESSAGES: DirectMessages = DirectMessages;

/// Messages that have a `from` (from_user) field.
pub struct UserPresence;

impl Filter for UserPresence {
    fn check_update(&self, update: &Update) -> FilterResult {
        if effective_message(update)
            .and_then(|m| m.get("from").cloned())
            .map(|v| !v.is_null())
            .unwrap_or(false)
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        "filters.USER"
    }
}

pub const USER: UserPresence = UserPresence;

/// Messages from a user who added the bot to the attachment menu.
pub struct UserAttachmentMenu;

impl Filter for UserAttachmentMenu {
    fn check_update(&self, update: &Update) -> FilterResult {
        if effective_user(update)
            .and_then(|u| u.get("added_to_attachment_menu").cloned())
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        "filters.USER_ATTACHMENT"
    }
}

pub const USER_ATTACHMENT: UserAttachmentMenu = UserAttachmentMenu;

/// Messages from a Telegram Premium user.
pub struct PremiumUser;

impl Filter for PremiumUser {
    fn check_update(&self, update: &Update) -> FilterResult {
        if effective_user(update)
            .and_then(|u| u.get("is_premium").cloned())
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        "filters.PREMIUM_USER"
    }
}

pub const PREMIUM_USER: PremiumUser = PremiumUser;

/// Messages with a `sender_chat` field present.
pub struct SenderChatPresence;

impl Filter for SenderChatPresence {
    fn check_update(&self, update: &Update) -> FilterResult {
        if effective_message(update)
            .and_then(|m| m.get("sender_chat").cloned())
            .map(|v| !v.is_null())
            .unwrap_or(false)
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        "filters.SenderChat.ALL"
    }
}

/// Messages with a `via_bot` field present.
pub struct ViaBotPresence;

impl Filter for ViaBotPresence {
    fn check_update(&self, update: &Update) -> FilterResult {
        if effective_message(update)
            .and_then(|m| m.get("via_bot").cloned())
            .map(|v| !v.is_null())
            .unwrap_or(false)
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        "filters.VIA_BOT"
    }
}

pub const VIA_BOT: ViaBotPresence = ViaBotPresence;

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn text_update(text: &str) -> Update {
        serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 0,
                "chat": {"id": 1, "type": "private"},
                "text": text
            }
        })).unwrap()
    }

    fn empty_update() -> Update {
        serde_json::from_value(json!({"update_id": 1})).unwrap()
    }

    #[test]
    fn all_matches_message() {
        assert!(ALL.check_update(&text_update("hello")).is_match());
    }

    #[test]
    fn all_rejects_empty() {
        assert!(!ALL.check_update(&empty_update()).is_match());
    }

    #[test]
    fn and_combinator() {
        let f = F::new(All) & F::new(All);
        assert!(f.check_update(&text_update("hello")).is_match());
    }

    #[test]
    fn or_combinator() {
        let f = F::new(All) | F::new(All);
        assert!(!f.check_update(&empty_update()).is_match());
    }

    #[test]
    fn not_combinator() {
        let f = !F::new(All);
        assert!(!f.check_update(&text_update("hi")).is_match());
    }

    #[test]
    fn xor_both_true_is_false() {
        let f = F::new(All) ^ F::new(All);
        assert!(!f.check_update(&text_update("hi")).is_match());
    }

    #[test]
    fn fn_filter_works() {
        let f = FnFilter::new("always_true", |_| true);
        assert!(f.check_update(&empty_update()).is_match());
    }

    #[test]
    fn presence_animation() {
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "animation": {"file_id": "a", "file_unique_id": "b", "width": 1, "height": 1, "duration": 1}
            }
        })).unwrap();
        assert!(ANIMATION.check_update(&update).is_match());
        assert!(!VIDEO.check_update(&update).is_match());
    }

    #[test]
    fn attachment_computed() {
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "document": {"file_id": "d", "file_unique_id": "e"}
            }
        })).unwrap();
        assert!(ATTACHMENT.check_update(&update).is_match());
    }

    #[test]
    fn effective_message_from_edited() {
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "edited_message": {"message_id": 2, "chat": {"id": 1, "type": "private"}, "date": 0}
        })).unwrap();
        assert!(effective_message(&update).is_some());
    }

    #[test]
    fn effective_user_from_callback() {
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "callback_query": {
                "id": "1",
                "from": {"id": 42, "is_bot": false, "first_name": "Test"},
                "chat_instance": "ci"
            }
        })).unwrap();
        let user = effective_user(&update).unwrap();
        assert_eq!(user.get("id").unwrap().as_i64().unwrap(), 42);
    }

    #[test]
    fn forum_filter() {
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "supergroup", "is_forum": true},
                "text": "hello"
            }
        })).unwrap();
        assert!(FORUM.check_update(&update).is_match());
    }

    #[test]
    fn premium_user_filter() {
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "from": {"id": 1, "is_bot": false, "first_name": "A", "is_premium": true},
                "text": "hi"
            }
        })).unwrap();
        assert!(PREMIUM_USER.check_update(&update).is_match());
    }

    #[test]
    fn filter_result_merge() {
        let a = FilterResult::MatchWithData(HashMap::from([("x".into(), vec!["1".into()])]));
        let b = FilterResult::MatchWithData(HashMap::from([("x".into(), vec!["2".into()])]));
        let merged = a.merge(b);
        if let FilterResult::MatchWithData(m) = merged {
            assert_eq!(m.get("x").unwrap(), &vec!["1".to_owned(), "2".to_owned()]);
        } else {
            panic!("expected MatchWithData");
        }
    }

    #[test]
    fn filter_result_merge_nomatch() {
        let a = FilterResult::Match;
        let b = FilterResult::NoMatch;
        assert_eq!(a.merge(b), FilterResult::NoMatch);
    }

    #[test]
    fn and_combinator_merges_data() {
        // Create two data filters
        let f1 = FnFilter::new("f1", |_| true);
        let f2 = FnFilter::new("f2", |_| true);
        let combined = F::new(f1) & F::new(f2);
        assert!(combined.check_update(&text_update("hi")).is_match());
    }

    #[test]
    fn or_returns_first_match() {
        let f1 = FnFilter::new("f1", |_| true);
        let f2 = FnFilter::new("f2", |_| false);
        let combined = F::new(f1) | F::new(f2);
        assert!(combined.check_update(&text_update("hi")).is_match());
    }

    #[test]
    fn xor_one_true() {
        let f1 = FnFilter::new("f1", |_| true);
        let f2 = FnFilter::new("f2", |_| false);
        let combined = F::new(f1) ^ F::new(f2);
        assert!(combined.check_update(&text_update("hi")).is_match());
    }
}
