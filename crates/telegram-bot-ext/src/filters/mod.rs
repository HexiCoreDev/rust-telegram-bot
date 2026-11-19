//! Filters for routing Telegram updates to handlers.

/// Core filter trait, combinator operators, and presence filters.
pub mod base;
/// Chat identity and chat-type filters.
pub mod chat;
/// Command entity filters.
pub mod command;
/// Document MIME type, category, and file extension filters.
pub mod document;
/// Message entity type filters.
pub mod entity;
/// Forwarded-from identity filters.
pub mod forwarded;
/// Photo and sticker filters.
pub mod photo;
/// Regex-based text matching filter.
pub mod regex;
/// Status update filters (new members, chat created, pinned, etc.).
pub mod status_update;
/// Text, caption, language, payment, dice, and mention filters.
pub mod text;
/// User identity filters.
pub mod user;
/// Via-bot identity filters.
pub mod via_bot;

// ---------------------------------------------------------------------------
// UpdateType namespace
// ---------------------------------------------------------------------------

/// Filters based on the top-level update kind (message, edited message, channel post, etc.).
pub mod update_type {
    use super::base::{Filter, FilterResult, Update};
    use rust_tg_bot_raw::types::update::UpdateKind;

    macro_rules! update_type_filter {
        ($struct_name:ident, $pattern:pat, $display:expr) => {
            /// Matches updates of this specific update kind.
            pub struct $struct_name;
            impl Filter for $struct_name {
                fn check_update(&self, update: &Update) -> FilterResult {
                    if matches!(update.kind, $pattern) {
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

    update_type_filter!(
        Message,
        UpdateKind::Message(_),
        "filters.UpdateType.MESSAGE"
    );
    /// Constant filter matching `Message` updates.
    pub const MESSAGE: Message = Message;

    /// Matches updates that are either a `Message` or an `EditedMessage`.
    pub struct Messages;
    impl Filter for Messages {
        fn check_update(&self, update: &Update) -> FilterResult {
            if matches!(
                update.kind,
                UpdateKind::Message(_) | UpdateKind::EditedMessage(_)
            ) {
                FilterResult::Match
            } else {
                FilterResult::NoMatch
            }
        }
        fn name(&self) -> &str {
            "filters.UpdateType.MESSAGES"
        }
    }
    /// Constant filter matching both `Message` and `EditedMessage` updates.
    pub const MESSAGES: Messages = Messages;

    update_type_filter!(
        EditedMessage,
        UpdateKind::EditedMessage(_),
        "filters.UpdateType.EDITED_MESSAGE"
    );
    /// Constant filter matching `EditedMessage` updates.
    pub const EDITED_MESSAGE: EditedMessage = EditedMessage;

    update_type_filter!(
        ChannelPost,
        UpdateKind::ChannelPost(_),
        "filters.UpdateType.CHANNEL_POST"
    );
    /// Constant filter matching `ChannelPost` updates.
    pub const CHANNEL_POST: ChannelPost = ChannelPost;

    /// Matches updates that are either a `ChannelPost` or an `EditedChannelPost`.
    pub struct ChannelPosts;
    impl Filter for ChannelPosts {
        fn check_update(&self, update: &Update) -> FilterResult {
            if matches!(
                update.kind,
                UpdateKind::ChannelPost(_) | UpdateKind::EditedChannelPost(_)
            ) {
                FilterResult::Match
            } else {
                FilterResult::NoMatch
            }
        }
        fn name(&self) -> &str {
            "filters.UpdateType.CHANNEL_POSTS"
        }
    }
    /// Constant filter matching both `ChannelPost` and `EditedChannelPost` updates.
    pub const CHANNEL_POSTS: ChannelPosts = ChannelPosts;

    update_type_filter!(
        EditedChannelPost,
        UpdateKind::EditedChannelPost(_),
        "filters.UpdateType.EDITED_CHANNEL_POST"
    );
    /// Constant filter matching `EditedChannelPost` updates.
    pub const EDITED_CHANNEL_POST: EditedChannelPost = EditedChannelPost;

    /// Matches any edited update (edited message, edited channel post, or edited business message).
    pub struct Edited;
    impl Filter for Edited {
        fn check_update(&self, update: &Update) -> FilterResult {
            if matches!(
                update.kind,
                UpdateKind::EditedMessage(_)
                    | UpdateKind::EditedChannelPost(_)
                    | UpdateKind::EditedBusinessMessage(_)
            ) {
                FilterResult::Match
            } else {
                FilterResult::NoMatch
            }
        }
        fn name(&self) -> &str {
            "filters.UpdateType.EDITED"
        }
    }
    /// Constant filter matching any edited update type.
    pub const EDITED: Edited = Edited;

    update_type_filter!(
        BusinessMessage,
        UpdateKind::BusinessMessage(_),
        "filters.UpdateType.BUSINESS_MESSAGE"
    );
    /// Constant filter matching `BusinessMessage` updates.
    pub const BUSINESS_MESSAGE: BusinessMessage = BusinessMessage;

    update_type_filter!(
        EditedBusinessMessage,
        UpdateKind::EditedBusinessMessage(_),
        "filters.UpdateType.EDITED_BUSINESS_MESSAGE"
    );
    /// Constant filter matching `EditedBusinessMessage` updates.
    pub const EDITED_BUSINESS_MESSAGE: EditedBusinessMessage = EditedBusinessMessage;

    /// Matches updates that are either a `BusinessMessage` or an `EditedBusinessMessage`.
    pub struct BusinessMessages;
    impl Filter for BusinessMessages {
        fn check_update(&self, update: &Update) -> FilterResult {
            if matches!(
                update.kind,
                UpdateKind::BusinessMessage(_) | UpdateKind::EditedBusinessMessage(_)
            ) {
                FilterResult::Match
            } else {
                FilterResult::NoMatch
            }
        }
        fn name(&self) -> &str {
            "filters.UpdateType.BUSINESS_MESSAGES"
        }
    }
    /// Constant filter matching both `BusinessMessage` and `EditedBusinessMessage` updates.
    pub const BUSINESS_MESSAGES: BusinessMessages = BusinessMessages;

    update_type_filter!(
        GuestMessage,
        UpdateKind::GuestMessage(_),
        "filters.UpdateType.GUEST_MESSAGE"
    );
    /// Constant filter matching `GuestMessage` updates.
    pub const GUEST_MESSAGE: GuestMessage = GuestMessage;

    update_type_filter!(
        Subscription,
        UpdateKind::Subscription(_),
        "filters.UpdateType.SUBSCRIPTION"
    );
    /// Constant filter matching `Subscription` updates.
    pub const SUBSCRIPTION: Subscription = Subscription;

    update_type_filter!(
        StoppedMessageGeneration,
        UpdateKind::StoppedMessageGeneration(_),
        "filters.UpdateType.STOPPED_MESSAGE_GENERATION"
    );
    /// Constant filter matching `StoppedMessageGeneration` updates.
    pub const STOPPED_MESSAGE_GENERATION: StoppedMessageGeneration = StoppedMessageGeneration;

    update_type_filter!(
        WidgetQuery,
        UpdateKind::WidgetQuery(_),
        "filters.UpdateType.WIDGET_QUERY"
    );
    /// Constant filter matching `WidgetQuery` updates (Bot API 10.4).
    pub const WIDGET_QUERY: WidgetQuery = WidgetQuery;

    update_type_filter!(
        SharedWidgetQuery,
        UpdateKind::SharedWidgetQuery(_),
        "filters.UpdateType.SHARED_WIDGET_QUERY"
    );
    /// Constant filter matching `SharedWidgetQuery` updates (Bot API 10.4).
    pub const SHARED_WIDGET_QUERY: SharedWidgetQuery = SharedWidgetQuery;

    #[cfg(test)]
    mod tests {
        use super::*;
        use serde_json::json;

        fn make_update(json_val: serde_json::Value) -> Update {
            serde_json::from_value(json_val).unwrap()
        }

        #[test]
        fn message_filter() {
            let update = make_update(
                json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}}}),
            );
            assert!(MESSAGE.check_update(&update).is_match());
            assert!(MESSAGES.check_update(&update).is_match());
            assert!(!EDITED_MESSAGE.check_update(&update).is_match());
        }
        #[test]
        fn edited_message_filter() {
            let update = make_update(
                json!({"update_id": 1, "edited_message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}}}),
            );
            assert!(EDITED_MESSAGE.check_update(&update).is_match());
            assert!(MESSAGES.check_update(&update).is_match());
            assert!(EDITED.check_update(&update).is_match());
            assert!(!MESSAGE.check_update(&update).is_match());
        }
        #[test]
        fn channel_post_filter() {
            let update = make_update(
                json!({"update_id": 1, "channel_post": {"message_id": 1, "date": 0, "chat": {"id": -100, "type": "channel"}}}),
            );
            assert!(CHANNEL_POST.check_update(&update).is_match());
            assert!(CHANNEL_POSTS.check_update(&update).is_match());
        }
        #[test]
        fn edited_channel_post_filter() {
            let update = make_update(
                json!({"update_id": 1, "edited_channel_post": {"message_id": 1, "date": 0, "chat": {"id": -100, "type": "channel"}}}),
            );
            assert!(EDITED_CHANNEL_POST.check_update(&update).is_match());
            assert!(CHANNEL_POSTS.check_update(&update).is_match());
            assert!(EDITED.check_update(&update).is_match());
        }
        #[test]
        fn business_message_filter() {
            let update = make_update(
                json!({"update_id": 1, "business_message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}}}),
            );
            assert!(BUSINESS_MESSAGE.check_update(&update).is_match());
            assert!(BUSINESS_MESSAGES.check_update(&update).is_match());
        }
        #[test]
        fn edited_business_message_filter() {
            let update = make_update(
                json!({"update_id": 1, "edited_business_message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}}}),
            );
            assert!(EDITED_BUSINESS_MESSAGE.check_update(&update).is_match());
            assert!(BUSINESS_MESSAGES.check_update(&update).is_match());
            assert!(EDITED.check_update(&update).is_match());
        }
        #[test]
        fn guest_message_filter() {
            let update = make_update(
                json!({"update_id": 1, "guest_message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "supergroup"}, "guest_query_id": "gq-1"}}),
            );
            assert!(GUEST_MESSAGE.check_update(&update).is_match());
            assert!(!MESSAGE.check_update(&update).is_match());
        }
        #[test]
        fn subscription_filter() {
            let update = make_update(
                json!({"update_id": 1, "subscription": {"user": {"id": 1, "is_bot": false, "first_name": "S"}, "invoice_payload": "p", "state": "active"}}),
            );
            assert!(SUBSCRIPTION.check_update(&update).is_match());
        }
        #[test]
        fn widget_query_filter() {
            let update = make_update(json!({"update_id": 1, "widget_query": {
                "id": "wq-1",
                "from": {"id": 1, "is_bot": false, "first_name": "W"},
                "chat": {"id": 1, "type": "private"},
                "message_id": 4,
                "chat_instance": "-1",
                "type": {"type": "data"},
                "data": "d"
            }}));
            assert!(WIDGET_QUERY.check_update(&update).is_match());
            assert!(!SHARED_WIDGET_QUERY.check_update(&update).is_match());
            assert!(!MESSAGE.check_update(&update).is_match());
        }

        #[test]
        fn shared_widget_query_filter() {
            let update = make_update(json!({"update_id": 1, "shared_widget_query": {
                "id": "swq-1",
                "from": {"id": 1, "is_bot": false, "first_name": "W"},
                "type": {"type": "data"},
                "data": "d",
                "widget_id": "shared-1"
            }}));
            assert!(SHARED_WIDGET_QUERY.check_update(&update).is_match());
            assert!(!WIDGET_QUERY.check_update(&update).is_match());
        }

        #[test]
        fn stopped_message_generation_filter() {
            let update = make_update(
                json!({"update_id": 1, "stopped_message_generation": {"chat": {"id": 1, "type": "private"}, "draft_id": 9}}),
            );
            assert!(STOPPED_MESSAGE_GENERATION.check_update(&update).is_match());
        }
    }
}

// ---------------------------------------------------------------------------
// Flat re-exports
// ---------------------------------------------------------------------------

pub use base::{
    effective_chat, effective_message, effective_user, has_effective_message, to_value, Filter,
    FilterResult, FnFilter, Update, ALL, ANIMATION, ATTACHMENT, AUDIO, BOOST_ADDED, CHECKLIST,
    CONTACT, DIRECT_MESSAGES, EFFECT_ID, F, FORUM, FORWARDED, GAME, GIVEAWAY, GIVEAWAY_WINNERS,
    HAS_MEDIA_SPOILER, HAS_PROTECTED_CONTENT, INVOICE, IS_AUTOMATIC_FORWARD, IS_FROM_OFFLINE,
    IS_TOPIC_MESSAGE, LOCATION, PAID_MEDIA, PASSPORT_DATA, POLL, PREMIUM_USER, REPLY,
    REPLY_TO_STORY, SENDER_BOOST_COUNT, STORY, SUGGESTED_POST_INFO, USER, USER_ATTACHMENT, VENUE,
    VIA_BOT, VIDEO, VIDEO_NOTE, VOICE,
};

pub use text::{
    dice_emoji, CaptionAny, CaptionFilter, CaptionRegexFilter, DiceFilter, LanguageFilter,
    MentionFilter, SuccessfulPaymentFilter, TextAny, TextFilter, CAPTION, TEXT,
};

pub use command::{CommandFilter, COMMAND};
pub use entity::{CaptionEntityFilter, EntityFilter};
pub use regex::RegexFilter;
pub use user::UserFilter;

pub use chat::{
    chat_type, sender_chat, ChatFilter, ChatTypeChannel, ChatTypeGroup, ChatTypeGroups,
    ChatTypePrivate, ChatTypeSuperGroup, SenderChatChannel, SenderChatFilter, SenderChatSuperGroup,
};

pub use document::{
    document as document_presets, DocumentAll, DocumentCategory, DocumentFileExtension,
    DocumentMimeType,
};

pub use photo::{
    sticker, PhotoFilter, StickerAll, StickerAnimated, StickerEmoji, StickerPremium, StickerStatic,
    StickerVideo, PHOTO,
};

pub use status_update::{
    status_update as status_update_presets, ChatBackgroundSet, ChatCreated, ChatOwnerChanged,
    ChatOwnerLeft, ChatShared, ChecklistTasksAdded, ChecklistTasksDone, ConnectedWebsite,
    DeleteChatPhoto, DirectMessagePriceChanged, ForumTopicClosed, ForumTopicCreated,
    ForumTopicEdited, ForumTopicReopened, GeneralForumTopicHidden, GeneralForumTopicUnhidden, Gift,
    GiftUpgradeSent, GiveawayCompleted, GiveawayCreated, LeftChatMember,
    MessageAutoDeleteTimerChanged, Migrate, NewChatMembers, NewChatPhoto, NewChatTitle,
    PaidMessagePriceChanged, PinnedMessage, ProximityAlertTriggered, RefundedPayment,
    StatusUpdateAll, SuggestedPostApprovalFailed, SuggestedPostApproved, SuggestedPostDeclined,
    SuggestedPostPaid, SuggestedPostRefunded, UniqueGift, UsersShared, VideoChatEnded,
    VideoChatParticipantsInvited, VideoChatScheduled, VideoChatStarted, WebAppData,
    WriteAccessAllowed,
};

pub use forwarded::ForwardedFromFilter;
pub use via_bot::ViaBotFilter;
