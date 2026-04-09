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

/// Matches updates where a new chat was created (group, supergroup, or channel).
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
    /// Matches updates containing a `chat_owner_changed` status update.
    ChatOwnerChanged,
    chat_owner_changed,
    "filters.StatusUpdate.CHAT_OWNER_CHANGED"
);
status_filter_option!(
    /// Matches updates containing a `chat_owner_left` status update.
    ChatOwnerLeft,
    chat_owner_left,
    "filters.StatusUpdate.CHAT_OWNER_LEFT"
);
status_filter_option!(
    /// Matches updates containing a `chat_shared` status update.
    ChatShared,
    chat_shared,
    "filters.StatusUpdate.CHAT_SHARED"
);
status_filter_option!(
    /// Matches updates containing a `checklist_tasks_added` status update.
    ChecklistTasksAdded,
    checklist_tasks_added,
    "filters.StatusUpdate.CHECKLIST_TASKS_ADDED"
);
status_filter_option!(
    /// Matches updates containing a `checklist_tasks_done` status update.
    ChecklistTasksDone,
    checklist_tasks_done,
    "filters.StatusUpdate.CHECKLIST_TASKS_DONE"
);
status_filter_option!(
    /// Matches updates containing a `connected_website` status update.
    ConnectedWebsite,
    connected_website,
    "filters.StatusUpdate.CONNECTED_WEBSITE"
);
status_filter_option!(
    /// Matches updates containing a `direct_message_price_changed` status update.
    DirectMessagePriceChanged,
    direct_message_price_changed,
    "filters.StatusUpdate.DIRECT_MESSAGE_PRICE_CHANGED"
);
status_filter_bool!(
    /// Matches updates where the chat photo was deleted.
    DeleteChatPhoto,
    delete_chat_photo,
    "filters.StatusUpdate.DELETE_CHAT_PHOTO"
);
status_filter_option!(
    /// Matches updates containing a `forum_topic_closed` status update.
    ForumTopicClosed,
    forum_topic_closed,
    "filters.StatusUpdate.FORUM_TOPIC_CLOSED"
);
status_filter_option!(
    /// Matches updates containing a `forum_topic_created` status update.
    ForumTopicCreated,
    forum_topic_created,
    "filters.StatusUpdate.FORUM_TOPIC_CREATED"
);
status_filter_option!(
    /// Matches updates containing a `forum_topic_edited` status update.
    ForumTopicEdited,
    forum_topic_edited,
    "filters.StatusUpdate.FORUM_TOPIC_EDITED"
);
status_filter_option!(
    /// Matches updates containing a `forum_topic_reopened` status update.
    ForumTopicReopened,
    forum_topic_reopened,
    "filters.StatusUpdate.FORUM_TOPIC_REOPENED"
);
status_filter_option!(
    /// Matches updates containing a `general_forum_topic_hidden` status update.
    GeneralForumTopicHidden,
    general_forum_topic_hidden,
    "filters.StatusUpdate.GENERAL_FORUM_TOPIC_HIDDEN"
);
status_filter_option!(
    /// Matches updates containing a `general_forum_topic_unhidden` status update.
    GeneralForumTopicUnhidden,
    general_forum_topic_unhidden,
    "filters.StatusUpdate.GENERAL_FORUM_TOPIC_UNHIDDEN"
);
status_filter_option!(
    /// Matches updates containing a `gift` status update.
    Gift,
    gift,
    "filters.StatusUpdate.GIFT"
);
status_filter_option!(
    /// Matches updates containing a `gift_upgrade_sent` status update.
    GiftUpgradeSent,
    gift_upgrade_sent,
    "filters.StatusUpdate.GIFT_UPGRADE_SENT"
);
status_filter_option!(
    /// Matches updates containing a `giveaway_created` status update.
    GiveawayCreated,
    giveaway_created,
    "filters.StatusUpdate.GIVEAWAY_CREATED"
);
status_filter_option!(
    /// Matches updates containing a `giveaway_completed` status update.
    GiveawayCompleted,
    giveaway_completed,
    "filters.StatusUpdate.GIVEAWAY_COMPLETED"
);
status_filter_option!(
    /// Matches updates containing a `left_chat_member` status update.
    LeftChatMember,
    left_chat_member,
    "filters.StatusUpdate.LEFT_CHAT_MEMBER"
);
status_filter_option!(
    /// Matches updates containing a `message_auto_delete_timer_changed` status update.
    MessageAutoDeleteTimerChanged,
    message_auto_delete_timer_changed,
    "filters.StatusUpdate.MESSAGE_AUTO_DELETE_TIMER_CHANGED"
);

/// Matches updates where a chat migration occurred (from or to a different chat ID).
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

/// Matches updates where new members were added to the chat.
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

/// Matches updates where a new chat photo was set.
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
    /// Matches updates containing a `new_chat_title` status update.
    NewChatTitle,
    new_chat_title,
    "filters.StatusUpdate.NEW_CHAT_TITLE"
);
status_filter_option!(
    /// Matches updates containing a `paid_message_price_changed` status update.
    PaidMessagePriceChanged,
    paid_message_price_changed,
    "filters.StatusUpdate.PAID_MESSAGE_PRICE_CHANGED"
);
status_filter_option!(
    /// Matches updates containing a `pinned_message` status update.
    PinnedMessage,
    pinned_message,
    "filters.StatusUpdate.PINNED_MESSAGE"
);
status_filter_option!(
    /// Matches updates containing a `proximity_alert_triggered` status update.
    ProximityAlertTriggered,
    proximity_alert_triggered,
    "filters.StatusUpdate.PROXIMITY_ALERT_TRIGGERED"
);
status_filter_option!(
    /// Matches updates containing a `refunded_payment` status update.
    RefundedPayment,
    refunded_payment,
    "filters.StatusUpdate.REFUNDED_PAYMENT"
);
status_filter_option!(
    /// Matches updates containing a `suggested_post_approval_failed` status update.
    SuggestedPostApprovalFailed,
    suggested_post_approval_failed,
    "filters.StatusUpdate.SUGGESTED_POST_APPROVAL_FAILED"
);
status_filter_option!(
    /// Matches updates containing a `suggested_post_approved` status update.
    SuggestedPostApproved,
    suggested_post_approved,
    "filters.StatusUpdate.SUGGESTED_POST_APPROVED"
);
status_filter_option!(
    /// Matches updates containing a `suggested_post_declined` status update.
    SuggestedPostDeclined,
    suggested_post_declined,
    "filters.StatusUpdate.SUGGESTED_POST_DECLINED"
);
status_filter_option!(
    /// Matches updates containing a `suggested_post_paid` status update.
    SuggestedPostPaid,
    suggested_post_paid,
    "filters.StatusUpdate.SUGGESTED_POST_PAID"
);
status_filter_option!(
    /// Matches updates containing a `suggested_post_refunded` status update.
    SuggestedPostRefunded,
    suggested_post_refunded,
    "filters.StatusUpdate.SUGGESTED_POST_REFUNDED"
);
status_filter_option!(
    /// Matches updates containing a `unique_gift` status update.
    UniqueGift,
    unique_gift,
    "filters.StatusUpdate.UNIQUE_GIFT"
);
status_filter_option!(
    /// Matches updates containing a `users_shared` status update.
    UsersShared,
    users_shared,
    "filters.StatusUpdate.USERS_SHARED"
);
status_filter_option!(
    /// Matches updates containing a `video_chat_ended` status update.
    VideoChatEnded,
    video_chat_ended,
    "filters.StatusUpdate.VIDEO_CHAT_ENDED"
);
status_filter_option!(
    /// Matches updates containing a `video_chat_scheduled` status update.
    VideoChatScheduled,
    video_chat_scheduled,
    "filters.StatusUpdate.VIDEO_CHAT_SCHEDULED"
);
status_filter_option!(
    /// Matches updates containing a `video_chat_started` status update.
    VideoChatStarted,
    video_chat_started,
    "filters.StatusUpdate.VIDEO_CHAT_STARTED"
);
status_filter_option!(
    /// Matches updates containing a `video_chat_participants_invited` status update.
    VideoChatParticipantsInvited,
    video_chat_participants_invited,
    "filters.StatusUpdate.VIDEO_CHAT_PARTICIPANTS_INVITED"
);
status_filter_option!(
    /// Matches updates containing a `web_app_data` status update.
    WebAppData,
    web_app_data,
    "filters.StatusUpdate.WEB_APP_DATA"
);
status_filter_option!(
    /// Matches updates containing a `write_access_allowed` status update.
    WriteAccessAllowed,
    write_access_allowed,
    "filters.StatusUpdate.WRITE_ACCESS_ALLOWED"
);

/// Matches any status update message.
///
/// This is the union of all individual status update filters: if any single
/// status update sub-filter matches, this filter also matches.
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

/// Pre-built constant instances for all status update filters.
pub mod presets {
    use super::*;
    /// Matches any status update message.
    pub const ALL: StatusUpdateAll = StatusUpdateAll;
    /// Matches updates where the chat background was set.
    pub const CHAT_BACKGROUND_SET: ChatBackgroundSet = ChatBackgroundSet;
    /// Matches updates where a chat was created.
    pub const CHAT_CREATED: ChatCreated = ChatCreated;
    /// Matches updates where the chat owner changed.
    pub const CHAT_OWNER_CHANGED: ChatOwnerChanged = ChatOwnerChanged;
    /// Matches updates where the chat owner left.
    pub const CHAT_OWNER_LEFT: ChatOwnerLeft = ChatOwnerLeft;
    /// Matches updates where a chat was shared.
    pub const CHAT_SHARED: ChatShared = ChatShared;
    /// Matches updates where checklist tasks were added.
    pub const CHECKLIST_TASKS_ADDED: ChecklistTasksAdded = ChecklistTasksAdded;
    /// Matches updates where checklist tasks were completed.
    pub const CHECKLIST_TASKS_DONE: ChecklistTasksDone = ChecklistTasksDone;
    /// Matches updates where a website was connected.
    pub const CONNECTED_WEBSITE: ConnectedWebsite = ConnectedWebsite;
    /// Matches updates where the direct message price changed.
    pub const DIRECT_MESSAGE_PRICE_CHANGED: DirectMessagePriceChanged = DirectMessagePriceChanged;
    /// Matches updates where the chat photo was deleted.
    pub const DELETE_CHAT_PHOTO: DeleteChatPhoto = DeleteChatPhoto;
    /// Matches updates where a forum topic was closed.
    pub const FORUM_TOPIC_CLOSED: ForumTopicClosed = ForumTopicClosed;
    /// Matches updates where a forum topic was created.
    pub const FORUM_TOPIC_CREATED: ForumTopicCreated = ForumTopicCreated;
    /// Matches updates where a forum topic was edited.
    pub const FORUM_TOPIC_EDITED: ForumTopicEdited = ForumTopicEdited;
    /// Matches updates where a forum topic was reopened.
    pub const FORUM_TOPIC_REOPENED: ForumTopicReopened = ForumTopicReopened;
    /// Matches updates where the general forum topic was hidden.
    pub const GENERAL_FORUM_TOPIC_HIDDEN: GeneralForumTopicHidden = GeneralForumTopicHidden;
    /// Matches updates where the general forum topic was unhidden.
    pub const GENERAL_FORUM_TOPIC_UNHIDDEN: GeneralForumTopicUnhidden = GeneralForumTopicUnhidden;
    /// Matches updates containing a gift.
    pub const GIFT: Gift = Gift;
    /// Matches updates where a gift upgrade was sent.
    pub const GIFT_UPGRADE_SENT: GiftUpgradeSent = GiftUpgradeSent;
    /// Matches updates where a giveaway was created.
    pub const GIVEAWAY_CREATED: GiveawayCreated = GiveawayCreated;
    /// Matches updates where a giveaway was completed.
    pub const GIVEAWAY_COMPLETED: GiveawayCompleted = GiveawayCompleted;
    /// Matches updates where a member left the chat.
    pub const LEFT_CHAT_MEMBER: LeftChatMember = LeftChatMember;
    /// Matches updates where the auto-delete timer changed.
    pub const MESSAGE_AUTO_DELETE_TIMER_CHANGED: MessageAutoDeleteTimerChanged =
        MessageAutoDeleteTimerChanged;
    /// Matches updates where a chat migration occurred.
    pub const MIGRATE: Migrate = Migrate;
    /// Matches updates where new members joined the chat.
    pub const NEW_CHAT_MEMBERS: NewChatMembers = NewChatMembers;
    /// Matches updates where a new chat photo was set.
    pub const NEW_CHAT_PHOTO: NewChatPhoto = NewChatPhoto;
    /// Matches updates where the chat title changed.
    pub const NEW_CHAT_TITLE: NewChatTitle = NewChatTitle;
    /// Matches updates where the paid message price changed.
    pub const PAID_MESSAGE_PRICE_CHANGED: PaidMessagePriceChanged = PaidMessagePriceChanged;
    /// Matches updates where a message was pinned.
    pub const PINNED_MESSAGE: PinnedMessage = PinnedMessage;
    /// Matches updates where a proximity alert was triggered.
    pub const PROXIMITY_ALERT_TRIGGERED: ProximityAlertTriggered = ProximityAlertTriggered;
    /// Matches updates containing a refunded payment.
    pub const REFUNDED_PAYMENT: RefundedPayment = RefundedPayment;
    /// Matches updates where a suggested post approval failed.
    pub const SUGGESTED_POST_APPROVAL_FAILED: SuggestedPostApprovalFailed =
        SuggestedPostApprovalFailed;
    /// Matches updates where a suggested post was approved.
    pub const SUGGESTED_POST_APPROVED: SuggestedPostApproved = SuggestedPostApproved;
    /// Matches updates where a suggested post was declined.
    pub const SUGGESTED_POST_DECLINED: SuggestedPostDeclined = SuggestedPostDeclined;
    /// Matches updates where a suggested post was paid.
    pub const SUGGESTED_POST_PAID: SuggestedPostPaid = SuggestedPostPaid;
    /// Matches updates where a suggested post was refunded.
    pub const SUGGESTED_POST_REFUNDED: SuggestedPostRefunded = SuggestedPostRefunded;
    /// Matches updates containing a unique gift.
    pub const UNIQUE_GIFT: UniqueGift = UniqueGift;
    /// Matches updates where users were shared.
    pub const USERS_SHARED: UsersShared = UsersShared;
    /// Matches updates where a video chat ended.
    pub const VIDEO_CHAT_ENDED: VideoChatEnded = VideoChatEnded;
    /// Matches updates where a video chat was scheduled.
    pub const VIDEO_CHAT_SCHEDULED: VideoChatScheduled = VideoChatScheduled;
    /// Matches updates where a video chat started.
    pub const VIDEO_CHAT_STARTED: VideoChatStarted = VideoChatStarted;
    /// Matches updates where participants were invited to a video chat.
    pub const VIDEO_CHAT_PARTICIPANTS_INVITED: VideoChatParticipantsInvited =
        VideoChatParticipantsInvited;
    /// Matches updates containing web app data.
    pub const WEB_APP_DATA: WebAppData = WebAppData;
    /// Matches updates where write access was allowed.
    pub const WRITE_ACCESS_ALLOWED: WriteAccessAllowed = WriteAccessAllowed;
}

/// Re-export of [`presets`] for ergonomic `status_update::ALL` access.
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
