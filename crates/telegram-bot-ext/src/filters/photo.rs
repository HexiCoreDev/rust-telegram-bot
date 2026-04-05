//! Photo filter and Sticker namespace.

use std::collections::HashSet;

use crate::filters::base::{effective_message_val, to_value, Filter, FilterResult, Update};

pub struct PhotoFilter;
impl Filter for PhotoFilter {
    fn check_update(&self, update: &Update) -> FilterResult { let __v = to_value(update);
        if effective_message_val(&__v)
            .and_then(|m| m.get("photo"))
            .and_then(|v| v.as_array())
            .map(|a| !a.is_empty())
            .unwrap_or(false)
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }
    fn name(&self) -> &str {
        "filters.PHOTO"
    }
}
pub const PHOTO: PhotoFilter = PhotoFilter;

pub struct StickerAll;
impl Filter for StickerAll {
    fn check_update(&self, update: &Update) -> FilterResult { let __v = to_value(update);
        if effective_message_val(&__v)
            .and_then(|m| m.get("sticker"))
            .map(|v| !v.is_null())
            .unwrap_or(false)
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }
    fn name(&self) -> &str {
        "filters.Sticker.ALL"
    }
}

pub struct StickerAnimated;
impl Filter for StickerAnimated {
    fn check_update(&self, update: &Update) -> FilterResult { let __v = to_value(update);
        if effective_message_val(&__v)
            .and_then(|m| m.get("sticker"))
            .filter(|s| !s.is_null())
            .and_then(|s| s.get("is_animated"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }
    fn name(&self) -> &str {
        "filters.Sticker.ANIMATED"
    }
}

pub struct StickerStatic;
impl Filter for StickerStatic {
    fn check_update(&self, update: &Update) -> FilterResult { let __v = to_value(update);
        let sticker = match effective_message_val(&__v).and_then(|m| m.get("sticker")) {
            Some(s) if !s.is_null() => s,
            _ => return FilterResult::NoMatch,
        };
        let is_animated = sticker
            .get("is_animated")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        let is_video = sticker
            .get("is_video")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        if !is_animated && !is_video {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }
    fn name(&self) -> &str {
        "filters.Sticker.STATIC"
    }
}

pub struct StickerVideo;
impl Filter for StickerVideo {
    fn check_update(&self, update: &Update) -> FilterResult { let __v = to_value(update);
        if effective_message_val(&__v)
            .and_then(|m| m.get("sticker"))
            .filter(|s| !s.is_null())
            .and_then(|s| s.get("is_video"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }
    fn name(&self) -> &str {
        "filters.Sticker.VIDEO"
    }
}

pub struct StickerPremium;
impl Filter for StickerPremium {
    fn check_update(&self, update: &Update) -> FilterResult { let __v = to_value(update);
        if effective_message_val(&__v)
            .and_then(|m| m.get("sticker"))
            .filter(|s| !s.is_null())
            .and_then(|s| s.get("premium_animation"))
            .map(|v| !v.is_null())
            .unwrap_or(false)
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }
    fn name(&self) -> &str {
        "filters.Sticker.PREMIUM"
    }
}

/// Filter that matches stickers whose `emoji` field is one of a provided set.
///
/// This mirrors Python's `filters.Sticker.Emoji` filter, which checks
/// `message.sticker.emoji` against a collection of emoji strings.
pub struct StickerEmoji {
    emojis: HashSet<String>,
}

impl StickerEmoji {
    /// Create a new `StickerEmoji` filter from a set of emoji strings.
    pub fn new(emojis: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self {
            emojis: emojis.into_iter().map(Into::into).collect(),
        }
    }
}

impl Filter for StickerEmoji {
    fn check_update(&self, update: &Update) -> FilterResult { let __v = to_value(update);
        let emoji = effective_message_val(&__v)
            .and_then(|m| m.get("sticker"))
            .filter(|s| !s.is_null())
            .and_then(|s| s.get("emoji"))
            .and_then(|v| v.as_str());

        match emoji {
            Some(e) if self.emojis.contains(e) => FilterResult::Match,
            _ => FilterResult::NoMatch,
        }
    }

    fn name(&self) -> &str {
        "filters.Sticker.Emoji"
    }
}

pub mod sticker {
    use super::*;
    pub const ALL: StickerAll = StickerAll;
    pub const ANIMATED: StickerAnimated = StickerAnimated;
    pub const STATIC: StickerStatic = StickerStatic;
    pub const VIDEO: StickerVideo = StickerVideo;
    pub const PREMIUM: StickerPremium = StickerPremium;

    /// Create a `StickerEmoji` filter for the given emoji strings.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use telegram_bot_ext::filters::photo::sticker;
    ///
    /// let emoji_filter = sticker::emoji(vec!["😀", "😂"]);
    /// ```
    pub fn emoji(emojis: impl IntoIterator<Item = impl Into<String>>) -> StickerEmoji {
        StickerEmoji::new(emojis)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn photo_filter_matches() {
        let update: Update = serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "photo": [{"file_id": "a", "file_unique_id": "b", "width": 100, "height": 100}]}})).unwrap();
        assert!(PHOTO.check_update(&update).is_match());
    }

    #[test]
    fn photo_filter_empty_array() {
        let update: Update = serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "photo": []}})).unwrap();
        assert!(!PHOTO.check_update(&update).is_match());
    }

    #[test]
    fn photo_filter_no_photo() {
        let update: Update = serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "text": "hello"}})).unwrap();
        assert!(!PHOTO.check_update(&update).is_match());
    }

    #[test]
    fn sticker_all() {
        let update: Update = serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "sticker": {"file_id": "s1", "file_unique_id": "su1", "type": "regular", "width": 512, "height": 512, "is_animated": false, "is_video": false}}})).unwrap();
        assert!(sticker::ALL.check_update(&update).is_match());
    }

    #[test]
    fn sticker_animated() {
        let update: Update = serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "sticker": {"file_id": "s1", "file_unique_id": "su1", "type": "regular", "width": 512, "height": 512, "is_animated": true, "is_video": false}}})).unwrap();
        assert!(sticker::ANIMATED.check_update(&update).is_match());
        assert!(!sticker::STATIC.check_update(&update).is_match());
        assert!(!sticker::VIDEO.check_update(&update).is_match());
    }

    #[test]
    fn sticker_video() {
        let update: Update = serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "sticker": {"file_id": "s1", "file_unique_id": "su1", "type": "regular", "width": 512, "height": 512, "is_animated": false, "is_video": true}}})).unwrap();
        assert!(sticker::VIDEO.check_update(&update).is_match());
        assert!(!sticker::STATIC.check_update(&update).is_match());
    }

    #[test]
    fn sticker_static() {
        let update: Update = serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "sticker": {"file_id": "s1", "file_unique_id": "su1", "type": "regular", "width": 512, "height": 512, "is_animated": false, "is_video": false}}})).unwrap();
        assert!(sticker::STATIC.check_update(&update).is_match());
    }

    #[test]
    fn sticker_premium() {
        let update: Update = serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "sticker": {"file_id": "s1", "file_unique_id": "su1", "type": "regular", "width": 512, "height": 512, "is_animated": false, "is_video": false, "premium_animation": {"file_id": "pa", "file_unique_id": "pau"}}}})).unwrap();
        assert!(sticker::PREMIUM.check_update(&update).is_match());
    }

    #[test]
    fn sticker_emoji_matches() {
        let filter = sticker::emoji(vec!["😀", "😂"]);
        let update: Update = serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "sticker": {"file_id": "s1", "file_unique_id": "su1", "type": "regular", "width": 512, "height": 512, "is_animated": false, "is_video": false, "emoji": "😀"}}})).unwrap();
        assert!(filter.check_update(&update).is_match());
    }

    #[test]
    fn sticker_emoji_rejects_wrong_emoji() {
        let filter = sticker::emoji(vec!["😀", "😂"]);
        let update: Update = serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "sticker": {"file_id": "s1", "file_unique_id": "su1", "type": "regular", "width": 512, "height": 512, "is_animated": false, "is_video": false, "emoji": "🤔"}}})).unwrap();
        assert!(!filter.check_update(&update).is_match());
    }

    #[test]
    fn sticker_emoji_rejects_no_emoji_field() {
        let filter = sticker::emoji(vec!["😀"]);
        let update: Update = serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "sticker": {"file_id": "s1", "file_unique_id": "su1", "type": "regular", "width": 512, "height": 512, "is_animated": false, "is_video": false}}})).unwrap();
        assert!(!filter.check_update(&update).is_match());
    }

    #[test]
    fn sticker_emoji_rejects_no_sticker() {
        let filter = sticker::emoji(vec!["😀"]);
        let update: Update = serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "text": "hello"}})).unwrap();
        assert!(!filter.check_update(&update).is_match());
    }
}
