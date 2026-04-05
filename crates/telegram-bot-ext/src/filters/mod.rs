//! Filters for routing Telegram updates to handlers.

pub mod base;
pub mod chat;
pub mod command;
pub mod document;
pub mod entity;
pub mod forwarded;
pub mod photo;
pub mod regex;
pub mod status_update;
pub mod text;
pub mod user;
pub mod via_bot;

// ---------------------------------------------------------------------------
// UpdateType namespace
// ---------------------------------------------------------------------------

pub mod update_type {
    use super::base::{to_value, Filter, FilterResult, Update};

    macro_rules! update_type_filter {
        ($struct_name:ident, $key:expr, $display:expr) => {
            pub struct $struct_name;
            impl Filter for $struct_name {
                fn check_update(&self, update: &Update) -> FilterResult {
                    let v = to_value(update);
                    if v.get($key).map(|v| !v.is_null()).unwrap_or(false) { FilterResult::Match } else { FilterResult::NoMatch }
                }
                fn name(&self) -> &str { $display }
            }
        };
    }

    update_type_filter!(Message, "message", "filters.UpdateType.MESSAGE");
    pub const MESSAGE: Message = Message;

    pub struct Messages;
    impl Filter for Messages {
        fn check_update(&self, update: &Update) -> FilterResult {
            let v = to_value(update);
            if v.get("message").map(|v| !v.is_null()).unwrap_or(false)
                || v.get("edited_message").map(|v| !v.is_null()).unwrap_or(false)
            { FilterResult::Match } else { FilterResult::NoMatch }
        }
        fn name(&self) -> &str { "filters.UpdateType.MESSAGES" }
    }
    pub const MESSAGES: Messages = Messages;

    update_type_filter!(EditedMessage, "edited_message", "filters.UpdateType.EDITED_MESSAGE");
    pub const EDITED_MESSAGE: EditedMessage = EditedMessage;

    update_type_filter!(ChannelPost, "channel_post", "filters.UpdateType.CHANNEL_POST");
    pub const CHANNEL_POST: ChannelPost = ChannelPost;

    pub struct ChannelPosts;
    impl Filter for ChannelPosts {
        fn check_update(&self, update: &Update) -> FilterResult {
            let v = to_value(update);
            if v.get("channel_post").map(|v| !v.is_null()).unwrap_or(false)
                || v.get("edited_channel_post").map(|v| !v.is_null()).unwrap_or(false)
            { FilterResult::Match } else { FilterResult::NoMatch }
        }
        fn name(&self) -> &str { "filters.UpdateType.CHANNEL_POSTS" }
    }
    pub const CHANNEL_POSTS: ChannelPosts = ChannelPosts;

    update_type_filter!(EditedChannelPost, "edited_channel_post", "filters.UpdateType.EDITED_CHANNEL_POST");
    pub const EDITED_CHANNEL_POST: EditedChannelPost = EditedChannelPost;

    pub struct Edited;
    impl Filter for Edited {
        fn check_update(&self, update: &Update) -> FilterResult {
            let v = to_value(update);
            let has = |key: &str| v.get(key).map(|v| !v.is_null()).unwrap_or(false);
            if has("edited_message") || has("edited_channel_post") || has("edited_business_message") { FilterResult::Match } else { FilterResult::NoMatch }
        }
        fn name(&self) -> &str { "filters.UpdateType.EDITED" }
    }
    pub const EDITED: Edited = Edited;

    update_type_filter!(BusinessMessage, "business_message", "filters.UpdateType.BUSINESS_MESSAGE");
    pub const BUSINESS_MESSAGE: BusinessMessage = BusinessMessage;

    update_type_filter!(EditedBusinessMessage, "edited_business_message", "filters.UpdateType.EDITED_BUSINESS_MESSAGE");
    pub const EDITED_BUSINESS_MESSAGE: EditedBusinessMessage = EditedBusinessMessage;

    pub struct BusinessMessages;
    impl Filter for BusinessMessages {
        fn check_update(&self, update: &Update) -> FilterResult {
            let v = to_value(update);
            if v.get("business_message").map(|v| !v.is_null()).unwrap_or(false)
                || v.get("edited_business_message").map(|v| !v.is_null()).unwrap_or(false)
            { FilterResult::Match } else { FilterResult::NoMatch }
        }
        fn name(&self) -> &str { "filters.UpdateType.BUSINESS_MESSAGES" }
    }
    pub const BUSINESS_MESSAGES: BusinessMessages = BusinessMessages;

    #[cfg(test)]
    mod tests {
        use super::*;
        use serde_json::json;

        fn make_update(json_val: serde_json::Value) -> Update {
            serde_json::from_value(json_val).unwrap()
        }

        #[test] fn message_filter() { let update = make_update(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}}})); assert!(MESSAGE.check_update(&update).is_match()); assert!(MESSAGES.check_update(&update).is_match()); assert!(!EDITED_MESSAGE.check_update(&update).is_match()); }
        #[test] fn edited_message_filter() { let update = make_update(json!({"update_id": 1, "edited_message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}}})); assert!(EDITED_MESSAGE.check_update(&update).is_match()); assert!(MESSAGES.check_update(&update).is_match()); assert!(EDITED.check_update(&update).is_match()); assert!(!MESSAGE.check_update(&update).is_match()); }
        #[test] fn channel_post_filter() { let update = make_update(json!({"update_id": 1, "channel_post": {"message_id": 1, "date": 0, "chat": {"id": -100, "type": "channel"}}})); assert!(CHANNEL_POST.check_update(&update).is_match()); assert!(CHANNEL_POSTS.check_update(&update).is_match()); }
        #[test] fn edited_channel_post_filter() { let update = make_update(json!({"update_id": 1, "edited_channel_post": {"message_id": 1, "date": 0, "chat": {"id": -100, "type": "channel"}}})); assert!(EDITED_CHANNEL_POST.check_update(&update).is_match()); assert!(CHANNEL_POSTS.check_update(&update).is_match()); assert!(EDITED.check_update(&update).is_match()); }
        #[test] fn business_message_filter() { let update = make_update(json!({"update_id": 1, "business_message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}}})); assert!(BUSINESS_MESSAGE.check_update(&update).is_match()); assert!(BUSINESS_MESSAGES.check_update(&update).is_match()); }
        #[test] fn edited_business_message_filter() { let update = make_update(json!({"update_id": 1, "edited_business_message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}}})); assert!(EDITED_BUSINESS_MESSAGE.check_update(&update).is_match()); assert!(BUSINESS_MESSAGES.check_update(&update).is_match()); assert!(EDITED.check_update(&update).is_match()); }
    }
}

// ---------------------------------------------------------------------------
// Flat re-exports
// ---------------------------------------------------------------------------

pub use base::{
    effective_chat, effective_message, effective_user, has_effective_message, to_value,
    Filter, FilterResult, FnFilter, F, Update,
    ALL, ANIMATION, ATTACHMENT, AUDIO, BOOST_ADDED, CHECKLIST, CONTACT, DIRECT_MESSAGES,
    EFFECT_ID, FORWARDED, FORUM, GAME, GIVEAWAY, GIVEAWAY_WINNERS, HAS_MEDIA_SPOILER,
    HAS_PROTECTED_CONTENT, INVOICE, IS_AUTOMATIC_FORWARD, IS_FROM_OFFLINE, IS_TOPIC_MESSAGE,
    LOCATION, PAID_MEDIA, PASSPORT_DATA, POLL, PREMIUM_USER, REPLY, REPLY_TO_STORY,
    SENDER_BOOST_COUNT, STORY, SUGGESTED_POST_INFO, USER, USER_ATTACHMENT, VENUE, VIA_BOT,
    VIDEO, VIDEO_NOTE, VOICE,
};

pub use text::{
    CaptionAny, CaptionFilter, CaptionRegexFilter, DiceFilter, LanguageFilter,
    MentionFilter, SuccessfulPaymentFilter, TextAny, TextFilter,
    CAPTION, TEXT,
    dice_emoji,
};

pub use command::{CommandFilter, COMMAND};
pub use regex::RegexFilter;
pub use entity::{CaptionEntityFilter, EntityFilter};
pub use user::UserFilter;

pub use chat::{
    ChatFilter, ChatTypeChannel, ChatTypeGroup, ChatTypeGroups,
    ChatTypePrivate, ChatTypeSuperGroup,
    SenderChatChannel, SenderChatFilter, SenderChatSuperGroup,
    chat_type, sender_chat,
};

pub use document::{
    DocumentAll, DocumentCategory, DocumentFileExtension, DocumentMimeType,
    document as document_presets,
};

pub use photo::{
    PhotoFilter, StickerAll, StickerAnimated, StickerEmoji, StickerPremium,
    StickerStatic, StickerVideo,
    sticker, PHOTO,
};

pub use status_update::{
    ChatBackgroundSet, ChatCreated, ChatOwnerChanged, ChatOwnerLeft,
    ChatShared, ChecklistTasksAdded, ChecklistTasksDone, ConnectedWebsite,
    DeleteChatPhoto, DirectMessagePriceChanged, ForumTopicClosed,
    ForumTopicCreated, ForumTopicEdited, ForumTopicReopened,
    GeneralForumTopicHidden, GeneralForumTopicUnhidden, Gift,
    GiftUpgradeSent, GiveawayCompleted, GiveawayCreated, LeftChatMember,
    MessageAutoDeleteTimerChanged, Migrate, NewChatMembers, NewChatPhoto,
    NewChatTitle, PaidMessagePriceChanged, PinnedMessage,
    ProximityAlertTriggered, RefundedPayment,
    SuggestedPostApprovalFailed, SuggestedPostApproved,
    SuggestedPostDeclined, SuggestedPostPaid, SuggestedPostRefunded,
    StatusUpdateAll, UniqueGift, UsersShared,
    VideoChatEnded, VideoChatParticipantsInvited, VideoChatScheduled,
    VideoChatStarted, WebAppData, WriteAccessAllowed,
    status_update as status_update_presets,
};

pub use forwarded::ForwardedFromFilter;
pub use via_bot::ViaBotFilter;
