//! Photo filter and Sticker namespace.

use std::collections::HashSet;

use crate::filters::base::{Filter, FilterResult, Update};

/// Matches messages that contain at least one photo.
pub struct PhotoFilter;
impl Filter for PhotoFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        if update
            .effective_message()
            .and_then(|m| m.photo.as_ref())
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
/// Constant instance of [`PhotoFilter`] -- matches messages containing a photo.
pub const PHOTO: PhotoFilter = PhotoFilter;

/// Matches messages containing any sticker.
pub struct StickerAll;
impl Filter for StickerAll {
    fn check_update(&self, update: &Update) -> FilterResult {
        if update
            .effective_message()
            .and_then(|m| m.sticker.as_ref())
            .is_some()
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

/// Matches messages containing an animated sticker (TGS format).
pub struct StickerAnimated;
impl Filter for StickerAnimated {
    fn check_update(&self, update: &Update) -> FilterResult {
        if update
            .effective_message()
            .and_then(|m| m.sticker.as_ref())
            .map(|s| s.is_animated)
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

/// Matches messages containing a static (non-animated, non-video) sticker.
pub struct StickerStatic;
impl Filter for StickerStatic {
    fn check_update(&self, update: &Update) -> FilterResult {
        let sticker = match update.effective_message().and_then(|m| m.sticker.as_ref()) {
            Some(s) => s,
            None => return FilterResult::NoMatch,
        };
        if !sticker.is_animated && !sticker.is_video {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }
    fn name(&self) -> &str {
        "filters.Sticker.STATIC"
    }
}

/// Matches messages containing a video sticker (WEBM format).
pub struct StickerVideo;
impl Filter for StickerVideo {
    fn check_update(&self, update: &Update) -> FilterResult {
        if update
            .effective_message()
            .and_then(|m| m.sticker.as_ref())
            .map(|s| s.is_video)
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

/// Matches messages containing a sticker with a premium animation.
pub struct StickerPremium;
impl Filter for StickerPremium {
    fn check_update(&self, update: &Update) -> FilterResult {
        if update
            .effective_message()
            .and_then(|m| m.sticker.as_ref())
            .and_then(|s| s.premium_animation.as_ref())
            .is_some()
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
    fn check_update(&self, update: &Update) -> FilterResult {
        let emoji = update
            .effective_message()
            .and_then(|m| m.sticker.as_ref())
            .and_then(|s| s.emoji.as_deref());

        match emoji {
            Some(e) if self.emojis.contains(e) => FilterResult::Match,
            _ => FilterResult::NoMatch,
        }
    }

    fn name(&self) -> &str {
        "filters.Sticker.Emoji"
    }
}

/// Convenience namespace grouping all `Sticker` filter constants and constructors.
pub mod sticker {
    use super::*;
    /// Matches any sticker message.
    pub const ALL: StickerAll = StickerAll;
    /// Matches animated sticker messages (TGS format).
    pub const ANIMATED: StickerAnimated = StickerAnimated;
    /// Matches static (non-animated, non-video) sticker messages.
    pub const STATIC: StickerStatic = StickerStatic;
    /// Matches video sticker messages (WEBM format).
    pub const VIDEO: StickerVideo = StickerVideo;
    /// Matches sticker messages with a premium animation.
    pub const PREMIUM: StickerPremium = StickerPremium;

    /// Create a `StickerEmoji` filter for the given emoji strings.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use rust_tg_bot_ext::filters::photo::sticker;
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
