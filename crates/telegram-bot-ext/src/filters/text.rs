//! Text and Caption filters.
//!
//! - [`TextAny`] / [`TEXT`] -- any message with text.
//! - [`TextFilter`] -- allow only messages whose text is in a given set.
//! - [`CaptionAny`] / [`CAPTION`] -- any message with a caption.
//! - [`CaptionFilter`] -- allow only messages whose caption is in a given set.
//! - [`CaptionRegexFilter`] -- search caption with a regex pattern.
//! - [`LanguageFilter`] -- filter by the sender's language code.
//! - [`SuccessfulPaymentFilter`] -- filter successful payment messages.

use std::collections::{HashMap, HashSet};

use crate::filters::base::{Filter, FilterResult, Update};

// ---------------------------------------------------------------------------
// TEXT
// ---------------------------------------------------------------------------

/// Matches any message that has a `text` field.
pub struct TextAny;

impl Filter for TextAny {
    fn check_update(&self, update: &Update) -> FilterResult {
        if update
            .effective_message()
            .and_then(|m| m.text.as_ref())
            .is_some()
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        "filters.TEXT"
    }
}

/// Constant instance -- `filters::TEXT`.
pub const TEXT: TextAny = TextAny;

/// Filters messages whose text exactly matches one of the given strings.
///
/// If `strings` is empty, behaves like [`TextAny`].
pub struct TextFilter {
    strings: HashSet<String>,
    display: String,
}

impl TextFilter {
    /// Create a filter that accepts messages whose text is one of `strings`.
    pub fn new(strings: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let set: HashSet<String> = strings.into_iter().map(Into::into).collect();
        let display = if set.is_empty() {
            "filters.TEXT".to_owned()
        } else {
            format!("filters.Text({:?})", set)
        };
        Self {
            strings: set,
            display,
        }
    }
}

impl Filter for TextFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let text = match update.effective_message().and_then(|m| m.text.as_deref()) {
            Some(t) => t,
            None => return FilterResult::NoMatch,
        };
        if self.strings.is_empty() {
            return FilterResult::Match;
        }
        if self.strings.contains(text) {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        &self.display
    }
}

// ---------------------------------------------------------------------------
// CAPTION
// ---------------------------------------------------------------------------

/// Matches any message that has a `caption` field.
pub struct CaptionAny;

impl Filter for CaptionAny {
    fn check_update(&self, update: &Update) -> FilterResult {
        if update
            .effective_message()
            .and_then(|m| m.caption.as_ref())
            .is_some()
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        "filters.CAPTION"
    }
}

/// Constant instance -- `filters::CAPTION`.
pub const CAPTION: CaptionAny = CaptionAny;

/// Filters messages whose caption exactly matches one of the given strings.
pub struct CaptionFilter {
    strings: HashSet<String>,
    display: String,
}

impl CaptionFilter {
    /// Create a filter that accepts messages whose caption is one of `strings`.
    pub fn new(strings: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let set: HashSet<String> = strings.into_iter().map(Into::into).collect();
        let display = if set.is_empty() {
            "filters.CAPTION".to_owned()
        } else {
            format!("filters.Caption({:?})", set)
        };
        Self {
            strings: set,
            display,
        }
    }
}

impl Filter for CaptionFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let caption = match update.effective_message().and_then(|m| m.caption.as_deref()) {
            Some(c) => c,
            None => return FilterResult::NoMatch,
        };
        if self.strings.is_empty() {
            return FilterResult::Match;
        }
        if self.strings.contains(caption) {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        &self.display
    }
}

// ---------------------------------------------------------------------------
// CaptionRegex
// ---------------------------------------------------------------------------

/// Searches the message caption with a compiled regex pattern.
///
/// Returns [`FilterResult::MatchWithData`] with capture groups under `"matches"`.
pub struct CaptionRegexFilter {
    pattern: regex::Regex,
    display: String,
}

impl CaptionRegexFilter {
    /// Create a new caption-regex filter.
    pub fn new(pattern: &str) -> Self {
        let re = regex::Regex::new(pattern).expect("invalid regex pattern");
        let display = format!("filters.CaptionRegex({})", pattern);
        Self {
            pattern: re,
            display,
        }
    }

    /// Create from a pre-compiled regex.
    pub fn from_regex(re: regex::Regex) -> Self {
        let display = format!("filters.CaptionRegex({})", re.as_str());
        Self {
            pattern: re,
            display,
        }
    }
}

impl Filter for CaptionRegexFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let caption = match update.effective_message().and_then(|m| m.caption.as_deref()) {
            Some(c) => c,
            None => return FilterResult::NoMatch,
        };

        match self.pattern.captures(caption) {
            Some(caps) => {
                let captures_vec: Vec<String> = caps
                    .iter()
                    .filter_map(|m| m.map(|mat| mat.as_str().to_owned()))
                    .collect();
                FilterResult::MatchWithData(HashMap::from([("matches".to_owned(), captures_vec)]))
            }
            None => FilterResult::NoMatch,
        }
    }

    fn name(&self) -> &str {
        &self.display
    }
}

// ---------------------------------------------------------------------------
// Language
// ---------------------------------------------------------------------------

/// Filters messages by the sender's IETF language code.
///
/// Uses prefix matching: `"en"` matches both `"en_US"` and `"en_GB"`.
pub struct LanguageFilter {
    langs: Vec<String>,
    display: String,
}

impl LanguageFilter {
    /// Create a filter for one or more language code prefixes.
    pub fn new(langs: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let langs: Vec<String> = langs.into_iter().map(Into::into).collect();
        let display = format!("filters.Language({:?})", langs);
        Self { langs, display }
    }
}

impl Filter for LanguageFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let code = match update
            .effective_message()
            .and_then(|m| m.from_user.as_ref())
            .and_then(|u| u.language_code.as_deref())
        {
            Some(c) => c,
            None => return FilterResult::NoMatch,
        };
        if self
            .langs
            .iter()
            .any(|prefix| code.starts_with(prefix.as_str()))
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        &self.display
    }
}

// ---------------------------------------------------------------------------
// SuccessfulPayment
// ---------------------------------------------------------------------------

/// Filters messages containing a successful payment, optionally restricted to
/// specific invoice payloads.
pub struct SuccessfulPaymentFilter {
    payloads: Option<HashSet<String>>,
    display: String,
}

impl SuccessfulPaymentFilter {
    /// Match any successful payment message.
    pub fn any() -> Self {
        Self {
            payloads: None,
            display: "filters.SUCCESSFUL_PAYMENT".to_owned(),
        }
    }

    /// Match only payments whose `invoice_payload` is in the given set.
    pub fn with_payloads(payloads: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let set: HashSet<String> = payloads.into_iter().map(Into::into).collect();
        let display = format!("filters.SuccessfulPayment({:?})", set);
        Self {
            payloads: Some(set),
            display,
        }
    }
}

impl Filter for SuccessfulPaymentFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let payment = match update
            .effective_message()
            .and_then(|m| m.successful_payment.as_ref())
        {
            Some(p) => p,
            None => return FilterResult::NoMatch,
        };
        match &self.payloads {
            None => FilterResult::Match,
            Some(set) => {
                if set.contains(payment.invoice_payload.as_str()) {
                    FilterResult::Match
                } else {
                    FilterResult::NoMatch
                }
            }
        }
    }

    fn name(&self) -> &str {
        &self.display
    }
}

// ---------------------------------------------------------------------------
// Dice
// ---------------------------------------------------------------------------

/// Dice emoji constants matching the Telegram Bot API.
pub mod dice_emoji {
    pub const BASKETBALL: &str = "\u{1F3C0}";
    pub const BOWLING: &str = "\u{1F3B3}";
    pub const DARTS: &str = "\u{1F3AF}";
    pub const DICE: &str = "\u{1F3B2}";
    pub const FOOTBALL: &str = "\u{26BD}";
    pub const SLOT_MACHINE: &str = "\u{1F3B0}";
}

/// Filter for dice messages, optionally restricted by emoji and/or value.
pub struct DiceFilter {
    emoji: Option<&'static str>,
    values: Option<Vec<i64>>,
    display: String,
}

impl DiceFilter {
    /// Match any dice message.
    pub fn all() -> Self {
        Self {
            emoji: None,
            values: None,
            display: "filters.Dice.ALL".to_owned(),
        }
    }

    /// Match dice messages with specific values (any emoji).
    pub fn with_values(values: impl IntoIterator<Item = i64>) -> Self {
        let vals: Vec<i64> = values.into_iter().collect();
        let display = format!("filters.Dice({:?})", vals);
        Self {
            emoji: None,
            values: Some(vals),
            display,
        }
    }

    /// Match dice messages with a specific emoji.
    pub fn with_emoji(emoji: &'static str) -> Self {
        let display = format!("filters.Dice.{}", emoji_label(emoji));
        Self {
            emoji: Some(emoji),
            values: None,
            display,
        }
    }

    /// Match dice messages with a specific emoji and value set.
    pub fn with_emoji_values(emoji: &'static str, values: impl IntoIterator<Item = i64>) -> Self {
        let vals: Vec<i64> = values.into_iter().collect();
        let display = format!("filters.Dice.{}({:?})", emoji_label(emoji), vals);
        Self {
            emoji: Some(emoji),
            values: Some(vals),
            display,
        }
    }

    // Convenience constructors for each emoji type.

    /// Basketball emoji filter with optional values.
    pub fn basketball(values: Option<Vec<i64>>) -> Self {
        match values {
            Some(v) => Self::with_emoji_values(dice_emoji::BASKETBALL, v),
            None => Self::with_emoji(dice_emoji::BASKETBALL),
        }
    }

    /// Bowling emoji filter.
    pub fn bowling(values: Option<Vec<i64>>) -> Self {
        match values {
            Some(v) => Self::with_emoji_values(dice_emoji::BOWLING, v),
            None => Self::with_emoji(dice_emoji::BOWLING),
        }
    }

    /// Darts emoji filter.
    pub fn darts(values: Option<Vec<i64>>) -> Self {
        match values {
            Some(v) => Self::with_emoji_values(dice_emoji::DARTS, v),
            None => Self::with_emoji(dice_emoji::DARTS),
        }
    }

    /// Dice (cube) emoji filter.
    pub fn dice(values: Option<Vec<i64>>) -> Self {
        match values {
            Some(v) => Self::with_emoji_values(dice_emoji::DICE, v),
            None => Self::with_emoji(dice_emoji::DICE),
        }
    }

    /// Football emoji filter.
    pub fn football(values: Option<Vec<i64>>) -> Self {
        match values {
            Some(v) => Self::with_emoji_values(dice_emoji::FOOTBALL, v),
            None => Self::with_emoji(dice_emoji::FOOTBALL),
        }
    }

    /// Slot machine emoji filter.
    pub fn slot_machine(values: Option<Vec<i64>>) -> Self {
        match values {
            Some(v) => Self::with_emoji_values(dice_emoji::SLOT_MACHINE, v),
            None => Self::with_emoji(dice_emoji::SLOT_MACHINE),
        }
    }
}

fn emoji_label(emoji: &str) -> &'static str {
    match emoji {
        dice_emoji::BASKETBALL => "BASKETBALL",
        dice_emoji::BOWLING => "BOWLING",
        dice_emoji::DARTS => "DARTS",
        dice_emoji::DICE => "DICE",
        dice_emoji::FOOTBALL => "FOOTBALL",
        dice_emoji::SLOT_MACHINE => "SLOT_MACHINE",
        _ => "UNKNOWN",
    }
}

impl Filter for DiceFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let dice = match update.effective_message().and_then(|m| m.dice.as_ref()) {
            Some(d) => d,
            None => return FilterResult::NoMatch,
        };

        let emoji_match = match self.emoji {
            Some(expected) => dice.emoji.as_str() == expected,
            None => true,
        };

        if !emoji_match {
            return FilterResult::NoMatch;
        }

        match &self.values {
            Some(vals) => {
                if vals.contains(&dice.value) {
                    FilterResult::Match
                } else {
                    FilterResult::NoMatch
                }
            }
            None => FilterResult::Match,
        }
    }

    fn name(&self) -> &str {
        &self.display
    }
}

// ---------------------------------------------------------------------------
// Mention
// ---------------------------------------------------------------------------

/// Filters messages that mention specified users / chats.
///
/// Accepts user IDs (i64), usernames (String), or both.
pub struct MentionFilter {
    ids: HashSet<i64>,
    usernames: HashSet<String>,
    display: String,
}

impl MentionFilter {
    /// Create a mention filter from user IDs.
    pub fn from_ids(ids: impl IntoIterator<Item = i64>) -> Self {
        let ids: HashSet<i64> = ids.into_iter().collect();
        let display = format!("filters.Mention({:?})", ids);
        Self {
            ids,
            usernames: HashSet::new(),
            display,
        }
    }

    /// Create a mention filter from usernames (leading `@` stripped).
    pub fn from_usernames(usernames: impl IntoIterator<Item = impl Into<String>>) -> Self {
        let usernames: HashSet<String> = usernames
            .into_iter()
            .map(|u| {
                let s: String = u.into();
                s.strip_prefix('@').unwrap_or(&s).to_owned()
            })
            .collect();
        let display = format!("filters.Mention({:?})", usernames);
        Self {
            ids: HashSet::new(),
            usernames,
            display,
        }
    }

    /// Create a mention filter from mixed IDs and usernames.
    pub fn new(
        ids: impl IntoIterator<Item = i64>,
        usernames: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        let ids: HashSet<i64> = ids.into_iter().collect();
        let usernames: HashSet<String> = usernames
            .into_iter()
            .map(|u| {
                let s: String = u.into();
                s.strip_prefix('@').unwrap_or(&s).to_owned()
            })
            .collect();
        let display = format!("filters.Mention(ids={:?}, usernames={:?})", ids, usernames);
        Self {
            ids,
            usernames,
            display,
        }
    }
}

impl Filter for MentionFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let msg = match update.effective_message() {
            Some(m) => m,
            None => return FilterResult::NoMatch,
        };
        let entities = match msg.entities.as_ref() {
            Some(arr) => arr,
            None => return FilterResult::NoMatch,
        };
        let text = msg.text.as_deref().unwrap_or("");

        for entity in entities {
            // Check by user object (text_mention)
            if entity.entity_type == "text_mention" {
                if let Some(user) = entity.user.as_ref() {
                    if self.ids.contains(&user.id) {
                        return FilterResult::Match;
                    }
                    if let Some(uname) = user.username.as_deref() {
                        if self.usernames.contains(uname) {
                            return FilterResult::Match;
                        }
                    }
                }
            }

            // Check by @mention text
            if entity.entity_type == "mention" {
                let offset = entity.offset as usize;
                let length = entity.length as usize;
                if offset + length <= text.len() {
                    let mention_text = &text[offset..offset + length];
                    let stripped = mention_text.strip_prefix('@').unwrap_or(mention_text);
                    if self.usernames.contains(stripped) {
                        return FilterResult::Match;
                    }
                }
            }
        }
        FilterResult::NoMatch
    }

    fn name(&self) -> &str {
        &self.display
    }
}

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
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "text": text
            }
        }))
        .unwrap()
    }

    #[test]
    fn text_any_matches() {
        assert!(TEXT.check_update(&text_update("hello")).is_match());
    }

    #[test]
    fn text_any_rejects_no_text() {
        let update: Update = serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}}})).unwrap();
        assert!(!TEXT.check_update(&update).is_match());
    }

    #[test]
    fn text_filter_exact_match() {
        let f = TextFilter::new(["Start", "Help"]);
        assert!(f.check_update(&text_update("Start")).is_match());
        assert!(!f.check_update(&text_update("start")).is_match());
    }

    #[test]
    fn text_filter_empty_accepts_all() {
        let f = TextFilter::new(Vec::<String>::new());
        assert!(f.check_update(&text_update("anything")).is_match());
    }

    #[test]
    fn caption_any_matches() {
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "caption": "look at this"
            }
        }))
        .unwrap();
        assert!(CAPTION.check_update(&update).is_match());
    }

    #[test]
    fn caption_filter_exact() {
        let f = CaptionFilter::new(["PTB rocks!"]);
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "caption": "PTB rocks!"
            }
        }))
        .unwrap();
        assert!(f.check_update(&update).is_match());
    }

    #[test]
    fn caption_regex_filter() {
        let f = CaptionRegexFilter::new(r"help");
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "caption": "I need help please"
            }
        }))
        .unwrap();
        assert!(f.check_update(&update).is_match());
    }

    #[test]
    fn language_filter() {
        let f = LanguageFilter::new(["en"]);
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "from": {"id": 1, "is_bot": false, "first_name": "A", "language_code": "en_US"},
                "text": "hi"
            }
        }))
        .unwrap();
        assert!(f.check_update(&update).is_match());
    }

    #[test]
    fn language_filter_no_match() {
        let f = LanguageFilter::new(["de"]);
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "from": {"id": 1, "is_bot": false, "first_name": "A", "language_code": "en_US"},
                "text": "hi"
            }
        }))
        .unwrap();
        assert!(!f.check_update(&update).is_match());
    }

    #[test]
    fn successful_payment_any() {
        let f = SuccessfulPaymentFilter::any();
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "successful_payment": {
                    "currency": "USD",
                    "total_amount": 100,
                    "invoice_payload": "payload-1",
                    "telegram_payment_charge_id": "x",
                    "provider_payment_charge_id": "y"
                }
            }
        }))
        .unwrap();
        assert!(f.check_update(&update).is_match());
    }

    #[test]
    fn successful_payment_by_payload() {
        let f = SuccessfulPaymentFilter::with_payloads(["payload-1"]);
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "successful_payment": {
                    "currency": "USD",
                    "total_amount": 100,
                    "invoice_payload": "payload-1",
                    "telegram_payment_charge_id": "x",
                    "provider_payment_charge_id": "y"
                }
            }
        }))
        .unwrap();
        assert!(f.check_update(&update).is_match());
        let f2 = SuccessfulPaymentFilter::with_payloads(["other"]);
        assert!(!f2.check_update(&update).is_match());
    }

    #[test]
    fn dice_all() {
        let f = DiceFilter::all();
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "dice": {"emoji": "\u{1F3B2}", "value": 3}
            }
        }))
        .unwrap();
        assert!(f.check_update(&update).is_match());
    }

    #[test]
    fn dice_with_values() {
        let f = DiceFilter::with_values([3, 4]);
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "dice": {"emoji": "\u{1F3B2}", "value": 3}
            }
        }))
        .unwrap();
        assert!(f.check_update(&update).is_match());
        let f2 = DiceFilter::with_values([5, 6]);
        assert!(!f2.check_update(&update).is_match());
    }

    #[test]
    fn dice_with_emoji() {
        let f = DiceFilter::with_emoji(dice_emoji::DARTS);
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "dice": {"emoji": "\u{1F3AF}", "value": 6}
            }
        }))
        .unwrap();
        assert!(f.check_update(&update).is_match());
    }

    #[test]
    fn dice_wrong_emoji() {
        let f = DiceFilter::with_emoji(dice_emoji::BASKETBALL);
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "dice": {"emoji": "\u{1F3B2}", "value": 3}
            }
        }))
        .unwrap();
        assert!(!f.check_update(&update).is_match());
    }

    #[test]
    fn mention_by_username() {
        let f = MentionFilter::from_usernames(["testbot"]);
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "text": "Hello @testbot!",
                "entities": [{"type": "mention", "offset": 6, "length": 8}]
            }
        }))
        .unwrap();
        assert!(f.check_update(&update).is_match());
    }

    #[test]
    fn mention_by_id() {
        let f = MentionFilter::from_ids([42]);
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "text": "Hello user",
                "entities": [{"type": "text_mention", "offset": 6, "length": 4, "user": {"id": 42, "is_bot": false, "first_name": "U"}}]
            }
        })).unwrap();
        assert!(f.check_update(&update).is_match());
    }
}
