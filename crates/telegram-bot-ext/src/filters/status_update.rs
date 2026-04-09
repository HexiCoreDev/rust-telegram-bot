//! StatusUpdate namespace -- all ~35+ status update sub-filters.

use crate::filters::base::{Filter, FilterResult, Update};

// ---------------------------------------------------------------------------
// Macro helpers
// ---------------------------------------------------------------------------

/// Generate a status filter that matches when `msg.<field>.is_some()`.
macro_rules! status_filter_option {
    ($(#[$meta:meta])* $struct_name:ident, $field:ident, $display:expr) => {
        $(#[$meta])*
        pub struct $struct_name;
        impl Filter for $struct_name {
            fn check_update(&self, update: &Update) -> FilterResult {
                if update
                    .effective_message()
                    .map(|m| m.$field.is_some())
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

/// Generate a status filter that matches when a `bool` field is `true`.
macro_rules! status_filter_bool {
    ($(#[$meta:meta])* $struct_name:ident, $field:ident, $display:expr) => {
        $(#[$meta])*
        pub struct $struct_name;
        impl Filter for $struct_name {
            fn check_update(&self, update: &Update) -> FilterResult {
                if update
                    .effective_message()
                    .map(|m| m.$field)
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
// Individual status filters
// ---------------------------------------------------------------------------

status_filter_option!(
    /// `chat_background_set` status update.
    ChatBackgroundSet,
    chat_background_set,
    "filters.StatusUpdate.CHAT_BACKGROUND_SET"
);

pub struct ChatCreated;
impl Filter for ChatCreated {
    fn check_update(&self, update: &Update) -> FilterResult {
        let msg = match update.effective_message() {
            Some(m) => m,
            None => return FilterResult::NoMatch,
        };
        if msg.group_chat_created || msg.supergroup_chat_created || msg.channel_chat_created {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }
    fn name(&self) -> &str {
        "filters.StatusUpdate.CHAT_CREATED"
    }
}

status_filter_option!(
    ChatOwnerChanged,
    chat_owner_changed,
    "filters.StatusUpdate.CHAT_OWNER_CHANGED"
);
status_filter_option!(
    ChatOwnerLeft,
    chat_owner_left,
    "filters.StatusUpdate.CHAT_OWNER_LEFT"
);
status_filter_option!(ChatShared, chat_shared, "filters.StatusUpdate.CHAT_SHARED");
status_filter_option!(
    ChecklistTasksAdded,
    checklist_tasks_added,
    "filters.StatusUpdate.CHECKLIST_TASKS_ADDED"
);
status_filter_option!(
    ChecklistTasksDone,
    checklist_tasks_done,
    "filters.StatusUpdate.CHECKLIST_TASKS_DONE"
);
status_filter_option!(
    ConnectedWebsite,
    connected_website,
    "filters.StatusUpdate.CONNECTED_WEBSITE"
);
status_filter_option!(
    DirectMessagePriceChanged,
    direct_message_price_changed,
    "filters.StatusUpdate.DIRECT_MESSAGE_PRICE_CHANGED"
);
status_filter_bool!(
    DeleteChatPhoto,
    delete_chat_photo,
    "filters.StatusUpdate.DELETE_CHAT_PHOTO"
);
status_filter_option!(
    ForumTopicClosed,
    forum_topic_closed,
    "filters.StatusUpdate.FORUM_TOPIC_CLOSED"
);
status_filter_option!(
    ForumTopicCreated,
    forum_topic_created,
    "filters.StatusUpdate.FORUM_TOPIC_CREATED"
);
status_filter_option!(
    ForumTopicEdited,
    forum_topic_edited,
    "filters.StatusUpdate.FORUM_TOPIC_EDITED"
);
status_filter_option!(
    ForumTopicReopened,
    forum_topic_reopened,
    "filters.StatusUpdate.FORUM_TOPIC_REOPENED"
);
status_filter_option!(
    GeneralForumTopicHidden,
    general_forum_topic_hidden,
    "filters.StatusUpdate.GENERAL_FORUM_TOPIC_HIDDEN"
);
status_filter_option!(
    GeneralForumTopicUnhidden,
    general_forum_topic_unhidden,
    "filters.StatusUpdate.GENERAL_FORUM_TOPIC_UNHIDDEN"
);
status_filter_option!(Gift, gift, "filters.StatusUpdate.GIFT");
status_filter_option!(
    GiftUpgradeSent,
    gift_upgrade_sent,
    "filters.StatusUpdate.GIFT_UPGRADE_SENT"
);
status_filter_option!(
    GiveawayCreated,
    giveaway_created,
    "filters.StatusUpdate.GIVEAWAY_CREATED"
);
status_filter_option!(
    GiveawayCompleted,
    giveaway_completed,
    "filters.StatusUpdate.GIVEAWAY_COMPLETED"
);
status_filter_option!(
    LeftChatMember,
    left_chat_member,
    "filters.StatusUpdate.LEFT_CHAT_MEMBER"
);
status_filter_option!(
    MessageAutoDeleteTimerChanged,
    message_auto_delete_timer_changed,
    "filters.StatusUpdate.MESSAGE_AUTO_DELETE_TIMER_CHANGED"
);

pub struct Migrate;
impl Filter for Migrate {
    fn check_update(&self, update: &Update) -> FilterResult {
        let msg = match update.effective_message() {
            Some(m) => m,
            None => return FilterResult::NoMatch,
        };
        if msg.migrate_from_chat_id.is_some() || msg.migrate_to_chat_id.is_some() {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }
    fn name(&self) -> &str {
        "filters.StatusUpdate.MIGRATE"
    }
}

pub struct NewChatMembers;
impl Filter for NewChatMembers {
    fn check_update(&self, update: &Update) -> FilterResult {
        if update
            .effective_message()
            .and_then(|m| m.new_chat_members.as_deref())
            .map(|members| !members.is_empty())
            .unwrap_or(false)
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }
    fn name(&self) -> &str {
        "filters.StatusUpdate.NEW_CHAT_MEMBERS"
    }
}

pub struct NewChatPhoto;
impl Filter for NewChatPhoto {
    fn check_update(&self, update: &Update) -> FilterResult {
        if update
            .effective_message()
            .and_then(|m| m.new_chat_photo.as_deref())
            .map(|photos| !photos.is_empty())
            .unwrap_or(false)
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }
    fn name(&self) -> &str {
        "filters.StatusUpdate.NEW_CHAT_PHOTO"
    }
}

status_filter_option!(
    NewChatTitle,
    new_chat_title,
    "filters.StatusUpdate.NEW_CHAT_TITLE"
);
status_filter_option!(
    PaidMessagePriceChanged,
    paid_message_price_changed,
    "filters.StatusUpdate.PAID_MESSAGE_PRICE_CHANGED"
);
status_filter_option!(
    PinnedMessage,
    pinned_message,
    "filters.StatusUpdate.PINNED_MESSAGE"
);
status_filter_option!(
    ProximityAlertTriggered,
    proximity_alert_triggered,
    "filters.StatusUpdate.PROXIMITY_ALERT_TRIGGERED"
);
status_filter_option!(
    RefundedPayment,
    refunded_payment,
    "filters.StatusUpdate.REFUNDED_PAYMENT"
);
status_filter_option!(
    SuggestedPostApprovalFailed,
    suggested_post_approval_failed,
    "filters.StatusUpdate.SUGGESTED_POST_APPROVAL_FAILED"
);
status_filter_option!(
    SuggestedPostApproved,
    suggested_post_approved,
    "filters.StatusUpdate.SUGGESTED_POST_APPROVED"
);
status_filter_option!(
    SuggestedPostDeclined,
    suggested_post_declined,
    "filters.StatusUpdate.SUGGESTED_POST_DECLINED"
);
status_filter_option!(
    SuggestedPostPaid,
    suggested_post_paid,
    "filters.StatusUpdate.SUGGESTED_POST_PAID"
);
status_filter_option!(
    SuggestedPostRefunded,
    suggested_post_refunded,
    "filters.StatusUpdate.SUGGESTED_POST_REFUNDED"
);
status_filter_option!(UniqueGift, unique_gift, "filters.StatusUpdate.UNIQUE_GIFT");
status_filter_option!(
    UsersShared,
    users_shared,
    "filters.StatusUpdate.USERS_SHARED"
);
status_filter_option!(
    VideoChatEnded,
    video_chat_ended,
    "filters.StatusUpdate.VIDEO_CHAT_ENDED"
);
status_filter_option!(
    VideoChatScheduled,
    video_chat_scheduled,
    "filters.StatusUpdate.VIDEO_CHAT_SCHEDULED"
);
status_filter_option!(
    VideoChatStarted,
    video_chat_started,
    "filters.StatusUpdate.VIDEO_CHAT_STARTED"
);
status_filter_option!(
    VideoChatParticipantsInvited,
    video_chat_participants_invited,
    "filters.StatusUpdate.VIDEO_CHAT_PARTICIPANTS_INVITED"
);
status_filter_option!(
    WebAppData,
    web_app_data,
    "filters.StatusUpdate.WEB_APP_DATA"
);
status_filter_option!(
    WriteAccessAllowed,
    write_access_allowed,
    "filters.StatusUpdate.WRITE_ACCESS_ALLOWED"
);

pub struct StatusUpdateAll;
impl Filter for StatusUpdateAll {
    fn check_update(&self, update: &Update) -> FilterResult {
        if ChatBackgroundSet.check_update(update).is_match()
            || ChatCreated.check_update(update).is_match()
            || ChatOwnerChanged.check_update(update).is_match()
            || ChatOwnerLeft.check_update(update).is_match()
            || ChatShared.check_update(update).is_match()
            || ChecklistTasksAdded.check_update(update).is_match()
            || ChecklistTasksDone.check_update(update).is_match()
            || ConnectedWebsite.check_update(update).is_match()
            || DirectMessagePriceChanged.check_update(update).is_match()
            || DeleteChatPhoto.check_update(update).is_match()
            || ForumTopicClosed.check_update(update).is_match()
            || ForumTopicCreated.check_update(update).is_match()
            || ForumTopicEdited.check_update(update).is_match()
            || ForumTopicReopened.check_update(update).is_match()
            || GeneralForumTopicHidden.check_update(update).is_match()
            || GeneralForumTopicUnhidden.check_update(update).is_match()
            || Gift.check_update(update).is_match()
            || GiftUpgradeSent.check_update(update).is_match()
            || GiveawayCreated.check_update(update).is_match()
            || GiveawayCompleted.check_update(update).is_match()
            || LeftChatMember.check_update(update).is_match()
            || MessageAutoDeleteTimerChanged
                .check_update(update)
                .is_match()
            || Migrate.check_update(update).is_match()
            || NewChatMembers.check_update(update).is_match()
            || NewChatPhoto.check_update(update).is_match()
            || NewChatTitle.check_update(update).is_match()
            || PaidMessagePriceChanged.check_update(update).is_match()
            || PinnedMessage.check_update(update).is_match()
            || ProximityAlertTriggered.check_update(update).is_match()
            || RefundedPayment.check_update(update).is_match()
            || SuggestedPostApprovalFailed.check_update(update).is_match()
            || SuggestedPostApproved.check_update(update).is_match()
            || SuggestedPostDeclined.check_update(update).is_match()
            || SuggestedPostPaid.check_update(update).is_match()
            || SuggestedPostRefunded.check_update(update).is_match()
            || UniqueGift.check_update(update).is_match()
            || UsersShared.check_update(update).is_match()
            || VideoChatEnded.check_update(update).is_match()
            || VideoChatScheduled.check_update(update).is_match()
            || VideoChatStarted.check_update(update).is_match()
            || VideoChatParticipantsInvited.check_update(update).is_match()
            || WebAppData.check_update(update).is_match()
            || WriteAccessAllowed.check_update(update).is_match()
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }
    fn name(&self) -> &str {
        "filters.StatusUpdate.ALL"
    }
}

pub mod presets {
    use super::*;
    pub const ALL: StatusUpdateAll = StatusUpdateAll;
    pub const CHAT_BACKGROUND_SET: ChatBackgroundSet = ChatBackgroundSet;
    pub const CHAT_CREATED: ChatCreated = ChatCreated;
    pub const CHAT_OWNER_CHANGED: ChatOwnerChanged = ChatOwnerChanged;
    pub const CHAT_OWNER_LEFT: ChatOwnerLeft = ChatOwnerLeft;
    pub const CHAT_SHARED: ChatShared = ChatShared;
    pub const CHECKLIST_TASKS_ADDED: ChecklistTasksAdded = ChecklistTasksAdded;
    pub const CHECKLIST_TASKS_DONE: ChecklistTasksDone = ChecklistTasksDone;
    pub const CONNECTED_WEBSITE: ConnectedWebsite = ConnectedWebsite;
    pub const DIRECT_MESSAGE_PRICE_CHANGED: DirectMessagePriceChanged = DirectMessagePriceChanged;
    pub const DELETE_CHAT_PHOTO: DeleteChatPhoto = DeleteChatPhoto;
    pub const FORUM_TOPIC_CLOSED: ForumTopicClosed = ForumTopicClosed;
    pub const FORUM_TOPIC_CREATED: ForumTopicCreated = ForumTopicCreated;
    pub const FORUM_TOPIC_EDITED: ForumTopicEdited = ForumTopicEdited;
    pub const FORUM_TOPIC_REOPENED: ForumTopicReopened = ForumTopicReopened;
    pub const GENERAL_FORUM_TOPIC_HIDDEN: GeneralForumTopicHidden = GeneralForumTopicHidden;
    pub const GENERAL_FORUM_TOPIC_UNHIDDEN: GeneralForumTopicUnhidden = GeneralForumTopicUnhidden;
    pub const GIFT: Gift = Gift;
    pub const GIFT_UPGRADE_SENT: GiftUpgradeSent = GiftUpgradeSent;
    pub const GIVEAWAY_CREATED: GiveawayCreated = GiveawayCreated;
    pub const GIVEAWAY_COMPLETED: GiveawayCompleted = GiveawayCompleted;
    pub const LEFT_CHAT_MEMBER: LeftChatMember = LeftChatMember;
    pub const MESSAGE_AUTO_DELETE_TIMER_CHANGED: MessageAutoDeleteTimerChanged =
        MessageAutoDeleteTimerChanged;
    pub const MIGRATE: Migrate = Migrate;
    pub const NEW_CHAT_MEMBERS: NewChatMembers = NewChatMembers;
    pub const NEW_CHAT_PHOTO: NewChatPhoto = NewChatPhoto;
    pub const NEW_CHAT_TITLE: NewChatTitle = NewChatTitle;
    pub const PAID_MESSAGE_PRICE_CHANGED: PaidMessagePriceChanged = PaidMessagePriceChanged;
    pub const PINNED_MESSAGE: PinnedMessage = PinnedMessage;
    pub const PROXIMITY_ALERT_TRIGGERED: ProximityAlertTriggered = ProximityAlertTriggered;
    pub const REFUNDED_PAYMENT: RefundedPayment = RefundedPayment;
    pub const SUGGESTED_POST_APPROVAL_FAILED: SuggestedPostApprovalFailed =
        SuggestedPostApprovalFailed;
    pub const SUGGESTED_POST_APPROVED: SuggestedPostApproved = SuggestedPostApproved;
    pub const SUGGESTED_POST_DECLINED: SuggestedPostDeclined = SuggestedPostDeclined;
    pub const SUGGESTED_POST_PAID: SuggestedPostPaid = SuggestedPostPaid;
    pub const SUGGESTED_POST_REFUNDED: SuggestedPostRefunded = SuggestedPostRefunded;
    pub const UNIQUE_GIFT: UniqueGift = UniqueGift;
    pub const USERS_SHARED: UsersShared = UsersShared;
    pub const VIDEO_CHAT_ENDED: VideoChatEnded = VideoChatEnded;
    pub const VIDEO_CHAT_SCHEDULED: VideoChatScheduled = VideoChatScheduled;
    pub const VIDEO_CHAT_STARTED: VideoChatStarted = VideoChatStarted;
    pub const VIDEO_CHAT_PARTICIPANTS_INVITED: VideoChatParticipantsInvited =
        VideoChatParticipantsInvited;
    pub const WEB_APP_DATA: WebAppData = WebAppData;
    pub const WRITE_ACCESS_ALLOWED: WriteAccessAllowed = WriteAccessAllowed;
}

pub use presets as status_update;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn status_msg(field: &str, value: serde_json::Value) -> Update {
        let mut msg = json!({"message_id": 1, "date": 0, "chat": {"id": 1, "type": "supergroup"}});
        msg.as_object_mut().unwrap().insert(field.to_owned(), value);
        serde_json::from_value(json!({"update_id": 1, "message": msg})).unwrap()
    }

    fn plain_text_update() -> Update {
        serde_json::from_value(json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}, "text": "just text"}})).unwrap()
    }

    #[test]
    fn new_chat_members() {
        let update = status_msg(
            "new_chat_members",
            json!([{"id": 1, "is_bot": false, "first_name": "A"}]),
        );
        assert!(NewChatMembers.check_update(&update).is_match());
        assert!(StatusUpdateAll.check_update(&update).is_match());
    }
    #[test]
    fn new_chat_members_empty_array() {
        let update = status_msg("new_chat_members", json!([]));
        assert!(!NewChatMembers.check_update(&update).is_match());
    }
    #[test]
    fn left_chat_member() {
        let update = status_msg(
            "left_chat_member",
            json!({"id": 1, "is_bot": false, "first_name": "A"}),
        );
        assert!(LeftChatMember.check_update(&update).is_match());
    }
    #[test]
    fn chat_created_group() {
        let update = status_msg("group_chat_created", json!(true));
        assert!(ChatCreated.check_update(&update).is_match());
    }
    #[test]
    fn chat_created_supergroup() {
        let update = status_msg("supergroup_chat_created", json!(true));
        assert!(ChatCreated.check_update(&update).is_match());
    }
    #[test]
    fn chat_created_channel() {
        let update = status_msg("channel_chat_created", json!(true));
        assert!(ChatCreated.check_update(&update).is_match());
    }
    #[test]
    fn migrate() {
        let update = status_msg("migrate_from_chat_id", json!(-100));
        assert!(Migrate.check_update(&update).is_match());
    }
    #[test]
    fn pinned_message() {
        let update = status_msg(
            "pinned_message",
            json!({"message_id": 42, "date": 0, "chat": {"id": 1, "type": "supergroup"}}),
        );
        assert!(PinnedMessage.check_update(&update).is_match());
    }
    #[test]
    fn delete_chat_photo() {
        let update = status_msg("delete_chat_photo", json!(true));
        assert!(DeleteChatPhoto.check_update(&update).is_match());
    }
    #[test]
    fn new_chat_title() {
        let update = status_msg("new_chat_title", json!("New Title"));
        assert!(NewChatTitle.check_update(&update).is_match());
    }
    #[test]
    fn forum_topic_created() {
        let update = status_msg(
            "forum_topic_created",
            json!({"name": "Topic", "icon_color": 7322096}),
        );
        assert!(ForumTopicCreated.check_update(&update).is_match());
    }
    #[test]
    fn video_chat_started() {
        let update = status_msg("video_chat_started", json!({}));
        assert!(VideoChatStarted.check_update(&update).is_match());
    }
    #[test]
    fn write_access_allowed() {
        let update = status_msg("write_access_allowed", json!({}));
        assert!(WriteAccessAllowed.check_update(&update).is_match());
    }
    #[test]
    fn status_update_all_includes_gift() {
        let update = status_msg(
            "gift",
            json!({"gift": {"id": "g1", "sticker": {"file_id": "x", "file_unique_id": "y", "width": 512, "height": 512, "is_animated": false, "is_video": false, "type": "regular"}, "star_count": 100}}),
        );
        assert!(StatusUpdateAll.check_update(&update).is_match());
    }
    #[test]
    fn status_update_all_rejects_plain_text() {
        let update = plain_text_update();
        assert!(!StatusUpdateAll.check_update(&update).is_match());
    }
}
