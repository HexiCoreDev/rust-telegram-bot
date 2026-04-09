//! API conformance tests for Telegram Bot API 9.6.
//!
//! These tests verify that our crate implements every method and type defined in
//! the official Telegram Bot API specification (version 9.6, April 2025).
//!
//! Method checks are compile-time: if a method is missing from `Bot`, the test
//! file will not compile.  Type checks verify deserialization from minimal JSON.

use std::sync::Arc;

use rust_tg_bot_raw::bot::Bot;
use rust_tg_bot_raw::request::base::{async_trait, BaseRequest, HttpMethod, TimeoutOverride};
use rust_tg_bot_raw::request::request_data::RequestData;
use rust_tg_bot_raw::types::files::input_file::InputFile;
use serde_json::json;

// ---------------------------------------------------------------------------
// Mock request backend (never actually called -- we only construct builders)
// ---------------------------------------------------------------------------

struct NoopRequest;

#[async_trait]
impl BaseRequest for NoopRequest {
    async fn initialize(&self) -> rust_tg_bot_raw::error::Result<()> {
        Ok(())
    }
    async fn shutdown(&self) -> rust_tg_bot_raw::error::Result<()> {
        Ok(())
    }
    fn default_read_timeout(&self) -> Option<std::time::Duration> {
        None
    }
    async fn do_request(
        &self,
        _url: &str,
        _method: HttpMethod,
        _data: Option<&RequestData>,
        _timeouts: TimeoutOverride,
    ) -> rust_tg_bot_raw::error::Result<(u16, bytes::Bytes)> {
        unreachable!("NoopRequest should not be called in conformance tests")
    }
    async fn do_request_json_bytes(
        &self,
        _url: &str,
        _body: &[u8],
        _timeouts: TimeoutOverride,
    ) -> rust_tg_bot_raw::error::Result<(u16, bytes::Bytes)> {
        unreachable!("NoopRequest should not be called in conformance tests")
    }
}

fn make_bot() -> Bot {
    Bot::new(
        "000000000:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
        Arc::new(NoopRequest),
    )
}

fn dummy_file() -> InputFile {
    InputFile::FileId("test_file_id".to_owned())
}

fn make_inline_keyboard_button(
) -> rust_tg_bot_raw::types::inline::inline_keyboard_button::InlineKeyboardButton {
    serde_json::from_value(json!({"text": "btn", "callback_data": "cb"})).unwrap()
}

fn make_input_checklist() -> rust_tg_bot_raw::types::input_checklist::InputChecklist {
    serde_json::from_value(json!({"title": "t", "tasks": []})).unwrap()
}

fn make_accepted_gift_types() -> rust_tg_bot_raw::types::gifts::AcceptedGiftTypes {
    serde_json::from_value(json!({
        "unlimited_gifts": false,
        "limited_gifts": false,
        "unique_gifts": false,
        "premium_subscription": false,
        "gifts_from_channels": false
    }))
    .unwrap()
}

// ===========================================================================
// #3a: All Bot API 9.6 methods exist
// ===========================================================================
//
// For each official method we verify either:
//   (a) A builder factory method exists on Bot (compile-time check), OR
//   (b) A raw async method exists on Bot (compile-time check via closure).
//
// If any method is missing, this test will fail to compile.
// ===========================================================================

/// Verify builder factory methods compile (no runtime needed -- just construction).
#[test]
fn all_bot_api_96_builder_methods_exist() {
    let bot = make_bot();

    // -- Getting updates --
    let _ = bot.set_webhook("https://example.com");
    let _ = bot.delete_webhook();

    // -- Sending messages --
    let _ = bot.send_message(1i64, "text");

    // -- Sending media --
    let _ = bot.send_photo(1i64, dummy_file());
    let _ = bot.send_audio(1i64, dummy_file());
    let _ = bot.send_document(1i64, dummy_file());
    let _ = bot.send_video(1i64, dummy_file());
    let _ = bot.send_animation(1i64, dummy_file());
    let _ = bot.send_voice(1i64, dummy_file());
    let _ = bot.send_video_note(1i64, dummy_file());
    let _ = bot.send_chat_action(1i64, "typing");

    // -- Sending other content --
    let _ = bot.send_location(1i64, 0.0, 0.0);
    let _ = bot.send_venue(1i64, 0.0, 0.0, "Title", "Address");
    let _ = bot.send_contact(1i64, "+1234", "John");
    let _ = bot.send_poll(1i64, "Question?", vec![]);
    let _ = bot.send_dice(1i64);
    let _ = bot.send_sticker(1i64, dummy_file());
    let _ = bot.send_invoice(1i64, "Title", "Desc", "payload", "USD", vec![]);

    // -- Updating messages --
    let _ = bot.edit_message_text("new text");
    let _ = bot.edit_message_caption();
    let _ = bot.edit_message_media(json!({}));
    let _ = bot.edit_message_reply_markup();

    // -- Inline mode --
    let _ = bot.answer_inline_query("query_id", vec![]);
    let _ = bot.answer_callback_query("callback_id");

    // -- Payments --
    let _ = bot.answer_shipping_query("sq_id", true);
    let _ = bot.answer_pre_checkout_query("pcq_id", true);

    // -- Files --
    let _ = bot.get_file("file_id");

    // -- Managed bots --
    let _ = bot.get_managed_bot_token(1i64);
    let _ = bot.replace_managed_bot_token(1i64);

    // -- Keyboard --
    let _ = bot.save_prepared_keyboard_button(1i64, make_inline_keyboard_button());
}

/// Verify raw async methods exist on Bot.
///
/// Each call constructs a future (never polled). If a method is missing or its
/// signature changes, this test will fail to compile. We leak the Bot to obtain
/// a `'&'static Bot` so the returned futures satisfy their lifetime bounds.
#[test]
fn all_bot_api_96_raw_methods_exist() {
    let bot: &'static Bot = Box::leak(Box::new(make_bot()));

    macro_rules! check {
        ($expr:expr) => {
            let _ = $expr;
        };
    }

    // -- Getting updates --
    check!(bot.get_updates(None, None, None, None));
    check!(bot.get_webhook_info());

    // -- Core --
    check!(bot.get_me());
    check!(bot.log_out());
    check!(bot.close());

    // -- Messages --
    check!(bot.forward_message(
        1i64.into(),
        1i64.into(),
        1,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    ));
    check!(bot.forward_messages(1i64.into(), 1i64.into(), vec![1], None, None, None, None));
    check!(bot.copy_message(
        1i64.into(),
        1i64.into(),
        1,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    ));
    check!(bot.copy_messages(
        1i64.into(),
        1i64.into(),
        vec![1],
        None,
        None,
        None,
        None,
        None
    ));
    check!(bot.send_message_draft(1, 1, "t", None, None, None));
    check!(bot.delete_message(1i64.into(), 1));
    check!(bot.delete_messages(1i64.into(), vec![1]));

    // -- Media --
    check!(bot.send_media_group(
        1i64.into(),
        vec![],
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    ));
    check!(bot.send_paid_media(
        1i64.into(),
        1,
        vec![],
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    ));

    // -- Other content --
    check!(bot.send_checklist(
        "bc_id",
        1,
        make_input_checklist(),
        None,
        None,
        None,
        None,
        None
    ));

    // -- Editing --
    check!(bot.edit_message_live_location(
        1.0, 1.0, None, None, None, None, None, None, None, None, None
    ));
    check!(bot.stop_message_live_location(None, None, None, None, None));
    check!(bot.edit_message_checklist("bc_id", 1, 1, make_input_checklist(), None));
    check!(bot.stop_poll(1i64.into(), 1, None, None));

    // -- Chat administration --
    check!(bot.leave_chat(1i64.into()));
    check!(bot.get_chat(1i64.into()));
    check!(bot.get_chat_administrators(1i64.into()));
    check!(bot.get_chat_member_count(1i64.into()));
    check!(bot.get_chat_member(1i64.into(), 1));
    check!(bot.ban_chat_member(1i64.into(), 1, None, None));
    check!(bot.unban_chat_member(1i64.into(), 1, None));
    check!(bot.ban_chat_sender_chat(1i64.into(), 1));
    check!(bot.unban_chat_sender_chat(1i64.into(), 1));
    check!(bot.restrict_chat_member(
        1i64.into(),
        1,
        rust_tg_bot_raw::types::chat_permissions::ChatPermissions::default(),
        None,
        None
    ));
    check!(bot.promote_chat_member(
        1i64.into(),
        1,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    ));
    check!(bot.set_chat_administrator_custom_title(1i64.into(), 1, "title"));
    check!(bot.set_chat_permissions(
        1i64.into(),
        rust_tg_bot_raw::types::chat_permissions::ChatPermissions::default(),
        None
    ));
    check!(bot.set_chat_photo(1i64.into(), dummy_file()));
    check!(bot.delete_chat_photo(1i64.into()));
    check!(bot.set_chat_title(1i64.into(), "t"));
    check!(bot.set_chat_description(1i64.into(), None));
    check!(bot.set_chat_sticker_set(1i64.into(), "name"));
    check!(bot.delete_chat_sticker_set(1i64.into()));
    check!(bot.set_chat_member_tag(1i64.into(), 1, None));
    check!(bot.pin_chat_message(1i64.into(), 1, None, None));
    check!(bot.unpin_chat_message(1i64.into(), None, None));
    check!(bot.unpin_all_chat_messages(1i64.into()));
    check!(bot.export_chat_invite_link(1i64.into()));
    check!(bot.create_chat_invite_link(1i64.into(), None, None, None, None));
    check!(bot.edit_chat_invite_link(1i64.into(), "link", None, None, None, None));
    check!(bot.revoke_chat_invite_link(1i64.into(), "link"));
    check!(bot.create_chat_subscription_invite_link(1i64.into(), 30, 1, None));
    check!(bot.edit_chat_subscription_invite_link(1i64.into(), "link", None));
    check!(bot.approve_chat_join_request(1i64.into(), 1));
    check!(bot.decline_chat_join_request(1i64.into(), 1));

    // -- Forum topics --
    check!(bot.create_forum_topic(1i64.into(), "name", None, None));
    check!(bot.edit_forum_topic(1i64.into(), 1, None, None));
    check!(bot.close_forum_topic(1i64.into(), 1));
    check!(bot.reopen_forum_topic(1i64.into(), 1));
    check!(bot.delete_forum_topic(1i64.into(), 1));
    check!(bot.unpin_all_forum_topic_messages(1i64.into(), 1));
    check!(bot.unpin_all_general_forum_topic_messages(1i64.into()));
    check!(bot.edit_general_forum_topic(1i64.into(), "name"));
    check!(bot.close_general_forum_topic(1i64.into()));
    check!(bot.reopen_general_forum_topic(1i64.into()));
    check!(bot.hide_general_forum_topic(1i64.into()));
    check!(bot.unhide_general_forum_topic(1i64.into()));

    // -- Bot settings --
    check!(bot.set_chat_menu_button(None, None));
    check!(bot.get_chat_menu_button(None));
    check!(bot.set_my_commands(vec![], None, None));
    check!(bot.get_my_commands(None, None));
    check!(bot.delete_my_commands(None, None));
    check!(bot.set_my_default_administrator_rights(None, None));
    check!(bot.get_my_default_administrator_rights(None));
    check!(bot.set_my_description(None, None));
    check!(bot.get_my_description(None));
    check!(bot.set_my_short_description(None, None));
    check!(bot.get_my_short_description(None));
    check!(bot.set_my_name(None, None));
    check!(bot.get_my_name(None));

    // -- User profile --
    check!(bot.get_user_profile_photos(1, None, None));
    check!(bot.get_user_profile_audios(1, None, None));
    check!(bot.set_user_emoji_status(1, None, None));
    check!(bot.set_my_profile_photo(json!({})));
    check!(bot.remove_my_profile_photo());

    // -- Stickers --
    check!(bot.get_sticker_set("name"));
    check!(bot.get_custom_emoji_stickers(vec![]));
    check!(bot.upload_sticker_file(1, dummy_file(), "static"));
    check!(bot.create_new_sticker_set(1, "name", "title", vec![], None, None));
    check!(bot.add_sticker_to_set(1, "name", json!({})));
    check!(bot.set_sticker_position_in_set("sticker", 0));
    check!(bot.delete_sticker_from_set("sticker"));
    check!(bot.replace_sticker_in_set(1, "name", "old", json!({})));
    check!(bot.set_sticker_emoji_list("sticker", vec![]));
    check!(bot.set_sticker_keywords("sticker", None));
    check!(bot.set_sticker_mask_position("sticker", None));
    check!(bot.set_sticker_set_thumbnail("name", 1, "static", None));
    check!(bot.set_sticker_set_title("name", "title"));
    check!(bot.set_custom_emoji_sticker_set_thumbnail("name", None));
    check!(bot.delete_sticker_set("name"));
    check!(bot.get_forum_topic_icon_stickers());

    // -- Inline mode --
    check!(bot.answer_web_app_query("waq_id", json!({})));
    check!(bot.save_prepared_inline_message(1, json!({}), None, None, None, None));

    // -- Payments --
    check!(bot.create_invoice_link(
        "t",
        "d",
        "p",
        "c",
        vec![],
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    ));
    check!(bot.refund_star_payment(1, "charge_id"));
    check!(bot.get_star_transactions(None, None));
    check!(bot.edit_user_star_subscription(1, "tid", false));
    check!(bot.get_my_star_balance());

    // -- Games --
    check!(bot.send_game(1, "game", None, None, None, None, None, None, None, None));
    check!(bot.set_game_score(1, 100, None, None, None, None, None));
    check!(bot.get_game_high_scores(1, None, None, None));

    // -- Reactions --
    check!(bot.set_message_reaction(1i64.into(), 1, None, None));
    check!(bot.get_user_chat_boosts(1i64.into(), 1));

    // -- Passport --
    check!(bot.set_passport_data_errors(1, vec![]));

    // -- Business --
    check!(bot.get_business_connection("bc_id"));
    check!(bot.get_business_account_gifts(
        "bc_id", None, None, None, None, None, None, None, None, None, None
    ));
    check!(bot.get_business_account_star_balance("bc_id"));
    check!(bot.read_business_message("bc_id", 1, 1));
    check!(bot.delete_business_messages("bc_id", vec![1]));
    check!(bot.set_business_account_name("bc_id", "first", None));
    check!(bot.set_business_account_username("bc_id", None));
    check!(bot.set_business_account_bio("bc_id", None));
    check!(bot.set_business_account_gift_settings("bc_id", false, make_accepted_gift_types()));
    check!(bot.set_business_account_profile_photo("bc_id", json!({}), None));
    check!(bot.remove_business_account_profile_photo("bc_id", None));
    check!(bot.convert_gift_to_stars("bc_id", "ogi"));
    check!(bot.upgrade_gift("bc_id", "ogi", None, None));
    check!(bot.transfer_gift("bc_id", "ogi", 1, None));
    check!(bot.transfer_business_account_stars("bc_id", 100));

    // -- Gifts --
    check!(bot.get_available_gifts());
    check!(bot.send_gift("gift_id", None, None, None, None, None, None));
    check!(bot.gift_premium_subscription(1, 3, 100, None, None, None));
    check!(bot.get_user_gifts(1, None, None, None, None, None, None, None, None));
    check!(bot.get_chat_gifts(
        1i64.into(),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None
    ));

    // -- Verification --
    check!(bot.verify_chat(1i64.into(), None));
    check!(bot.verify_user(1, None));
    check!(bot.remove_chat_verification(1i64.into()));
    check!(bot.remove_user_verification(1));

    // -- Stories --
    check!(bot.post_story(
        "bc_id",
        json!({}),
        86400,
        None,
        None,
        None,
        None,
        None,
        None
    ));
    check!(bot.edit_story("bc_id", 1, json!({}), None, None, None, None));
    check!(bot.delete_story("bc_id", 1));
    check!(bot.repost_story("bc_id", 1, 1, 86400, None, None));

    // -- Suggested posts --
    check!(bot.approve_suggested_post(1, 1, None));
    check!(bot.decline_suggested_post(1, 1, None));
}

// ===========================================================================
// Official Telegram Bot API 9.6 method list (169 methods)
// ===========================================================================

/// The complete list of Bot API 9.6 methods mapped to our Rust snake_case names.
/// This test verifies the count matches the official spec.
#[test]
fn method_count_matches_api_96() {
    let methods: Vec<(&str, &str)> = vec![
        // -- Getting updates --
        ("getUpdates", "get_updates"),
        ("setWebhook", "set_webhook"),
        ("deleteWebhook", "delete_webhook"),
        ("getWebhookInfo", "get_webhook_info"),
        // -- Available methods --
        ("getMe", "get_me"),
        ("logOut", "log_out"),
        ("close", "close"),
        ("sendMessage", "send_message"),
        ("forwardMessage", "forward_message"),
        ("forwardMessages", "forward_messages"),
        ("copyMessage", "copy_message"),
        ("copyMessages", "copy_messages"),
        ("sendPhoto", "send_photo"),
        ("sendAudio", "send_audio"),
        ("sendDocument", "send_document"),
        ("sendVideo", "send_video"),
        ("sendAnimation", "send_animation"),
        ("sendVoice", "send_voice"),
        ("sendVideoNote", "send_video_note"),
        ("sendPaidMedia", "send_paid_media"),
        ("sendMediaGroup", "send_media_group"),
        ("sendLocation", "send_location"),
        ("sendVenue", "send_venue"),
        ("sendContact", "send_contact"),
        ("sendPoll", "send_poll"),
        ("sendDice", "send_dice"),
        ("sendChatAction", "send_chat_action"),
        ("setMessageReaction", "set_message_reaction"),
        ("getUserProfilePhotos", "get_user_profile_photos"),
        ("getFile", "get_file"),
        ("banChatMember", "ban_chat_member"),
        ("unbanChatMember", "unban_chat_member"),
        ("restrictChatMember", "restrict_chat_member"),
        ("promoteChatMember", "promote_chat_member"),
        (
            "setChatAdministratorCustomTitle",
            "set_chat_administrator_custom_title",
        ),
        ("banChatSenderChat", "ban_chat_sender_chat"),
        ("unbanChatSenderChat", "unban_chat_sender_chat"),
        ("setChatPermissions", "set_chat_permissions"),
        ("exportChatInviteLink", "export_chat_invite_link"),
        ("createChatInviteLink", "create_chat_invite_link"),
        ("editChatInviteLink", "edit_chat_invite_link"),
        (
            "createChatSubscriptionInviteLink",
            "create_chat_subscription_invite_link",
        ),
        (
            "editChatSubscriptionInviteLink",
            "edit_chat_subscription_invite_link",
        ),
        ("revokeChatInviteLink", "revoke_chat_invite_link"),
        ("approveChatJoinRequest", "approve_chat_join_request"),
        ("declineChatJoinRequest", "decline_chat_join_request"),
        ("setChatPhoto", "set_chat_photo"),
        ("deleteChatPhoto", "delete_chat_photo"),
        ("setChatTitle", "set_chat_title"),
        ("setChatDescription", "set_chat_description"),
        ("pinChatMessage", "pin_chat_message"),
        ("unpinChatMessage", "unpin_chat_message"),
        ("unpinAllChatMessages", "unpin_all_chat_messages"),
        ("leaveChat", "leave_chat"),
        ("getChat", "get_chat"),
        ("getChatAdministrators", "get_chat_administrators"),
        ("getChatMemberCount", "get_chat_member_count"),
        ("getChatMember", "get_chat_member"),
        ("setChatStickerSet", "set_chat_sticker_set"),
        ("deleteChatStickerSet", "delete_chat_sticker_set"),
        ("getForumTopicIconStickers", "get_forum_topic_icon_stickers"),
        ("createForumTopic", "create_forum_topic"),
        ("editForumTopic", "edit_forum_topic"),
        ("closeForumTopic", "close_forum_topic"),
        ("reopenForumTopic", "reopen_forum_topic"),
        ("deleteForumTopic", "delete_forum_topic"),
        (
            "unpinAllForumTopicMessages",
            "unpin_all_forum_topic_messages",
        ),
        ("editGeneralForumTopic", "edit_general_forum_topic"),
        ("closeGeneralForumTopic", "close_general_forum_topic"),
        ("reopenGeneralForumTopic", "reopen_general_forum_topic"),
        ("hideGeneralForumTopic", "hide_general_forum_topic"),
        ("unhideGeneralForumTopic", "unhide_general_forum_topic"),
        (
            "unpinAllGeneralForumTopicMessages",
            "unpin_all_general_forum_topic_messages",
        ),
        ("answerCallbackQuery", "answer_callback_query"),
        ("getUserChatBoosts", "get_user_chat_boosts"),
        ("getBusinessConnection", "get_business_connection"),
        ("setMyCommands", "set_my_commands"),
        ("deleteMyCommands", "delete_my_commands"),
        ("getMyCommands", "get_my_commands"),
        ("setMyName", "set_my_name"),
        ("getMyName", "get_my_name"),
        ("setMyDescription", "set_my_description"),
        ("getMyDescription", "get_my_description"),
        ("setMyShortDescription", "set_my_short_description"),
        ("getMyShortDescription", "get_my_short_description"),
        ("setChatMenuButton", "set_chat_menu_button"),
        ("getChatMenuButton", "get_chat_menu_button"),
        (
            "setMyDefaultAdministratorRights",
            "set_my_default_administrator_rights",
        ),
        (
            "getMyDefaultAdministratorRights",
            "get_my_default_administrator_rights",
        ),
        // -- Updating messages --
        ("editMessageText", "edit_message_text"),
        ("editMessageCaption", "edit_message_caption"),
        ("editMessageMedia", "edit_message_media"),
        ("editMessageLiveLocation", "edit_message_live_location"),
        ("stopMessageLiveLocation", "stop_message_live_location"),
        ("editMessageReplyMarkup", "edit_message_reply_markup"),
        ("stopPoll", "stop_poll"),
        ("deleteMessage", "delete_message"),
        ("deleteMessages", "delete_messages"),
        // -- Stickers --
        ("sendSticker", "send_sticker"),
        ("getStickerSet", "get_sticker_set"),
        ("getCustomEmojiStickers", "get_custom_emoji_stickers"),
        ("uploadStickerFile", "upload_sticker_file"),
        ("createNewStickerSet", "create_new_sticker_set"),
        ("addStickerToSet", "add_sticker_to_set"),
        ("setStickerPositionInSet", "set_sticker_position_in_set"),
        ("deleteStickerFromSet", "delete_sticker_from_set"),
        ("replaceStickerInSet", "replace_sticker_in_set"),
        ("setStickerEmojiList", "set_sticker_emoji_list"),
        ("setStickerKeywords", "set_sticker_keywords"),
        ("setStickerMaskPosition", "set_sticker_mask_position"),
        ("setStickerSetThumbnail", "set_sticker_set_thumbnail"),
        ("setStickerSetTitle", "set_sticker_set_title"),
        (
            "setCustomEmojiStickerSetThumbnail",
            "set_custom_emoji_sticker_set_thumbnail",
        ),
        ("deleteStickerSet", "delete_sticker_set"),
        // -- Inline mode --
        ("answerInlineQuery", "answer_inline_query"),
        ("answerWebAppQuery", "answer_web_app_query"),
        ("savePreparedInlineMessage", "save_prepared_inline_message"),
        // -- Payments --
        ("sendInvoice", "send_invoice"),
        ("createInvoiceLink", "create_invoice_link"),
        ("answerShippingQuery", "answer_shipping_query"),
        ("answerPreCheckoutQuery", "answer_pre_checkout_query"),
        ("getStarTransactions", "get_star_transactions"),
        ("refundStarPayment", "refund_star_payment"),
        ("editUserStarSubscription", "edit_user_star_subscription"),
        ("getMyStarBalance", "get_my_star_balance"),
        // -- Telegram Passport --
        ("setPassportDataErrors", "set_passport_data_errors"),
        // -- Games --
        ("sendGame", "send_game"),
        ("setGameScore", "set_game_score"),
        ("getGameHighScores", "get_game_high_scores"),
        // -- Checklists --
        ("sendChecklist", "send_checklist"),
        ("editMessageChecklist", "edit_message_checklist"),
        ("setChatMemberTag", "set_chat_member_tag"),
        // -- Gifts --
        ("getAvailableGifts", "get_available_gifts"),
        ("sendGift", "send_gift"),
        ("giftPremiumSubscription", "gift_premium_subscription"),
        ("getUserGifts", "get_user_gifts"),
        ("getChatGifts", "get_chat_gifts"),
        ("convertGiftToStars", "convert_gift_to_stars"),
        ("upgradeGift", "upgrade_gift"),
        ("transferGift", "transfer_gift"),
        // -- Business --
        ("getBusinessAccountGifts", "get_business_account_gifts"),
        (
            "getBusinessAccountStarBalance",
            "get_business_account_star_balance",
        ),
        ("readBusinessMessage", "read_business_message"),
        ("deleteBusinessMessages", "delete_business_messages"),
        ("setBusinessAccountName", "set_business_account_name"),
        (
            "setBusinessAccountUsername",
            "set_business_account_username",
        ),
        ("setBusinessAccountBio", "set_business_account_bio"),
        (
            "setBusinessAccountGiftSettings",
            "set_business_account_gift_settings",
        ),
        (
            "setBusinessAccountProfilePhoto",
            "set_business_account_profile_photo",
        ),
        (
            "removeBusinessAccountProfilePhoto",
            "remove_business_account_profile_photo",
        ),
        (
            "transferBusinessAccountStars",
            "transfer_business_account_stars",
        ),
        // -- Verification --
        ("verifyChat", "verify_chat"),
        ("verifyUser", "verify_user"),
        ("removeChatVerification", "remove_chat_verification"),
        ("removeUserVerification", "remove_user_verification"),
        // -- User profile --
        ("getUserProfileAudios", "get_user_profile_audios"),
        ("setUserEmojiStatus", "set_user_emoji_status"),
        ("setMyProfilePhoto", "set_my_profile_photo"),
        ("removeMyProfilePhoto", "remove_my_profile_photo"),
        // -- Stories --
        ("postStory", "post_story"),
        ("editStory", "edit_story"),
        ("deleteStory", "delete_story"),
        ("repostStory", "repost_story"),
        // -- Managed bots --
        ("getManagedBotToken", "get_managed_bot_token"),
        ("replaceManagedBotToken", "replace_managed_bot_token"),
        // -- Prepared keyboard --
        (
            "savePreparedKeyboardButton",
            "save_prepared_keyboard_button",
        ),
        // -- Suggested posts --
        ("approveSuggestedPost", "approve_suggested_post"),
        ("declineSuggestedPost", "decline_suggested_post"),
        // -- Draft messages --
        ("sendMessageDraft", "send_message_draft"),
    ];

    assert_eq!(
        methods.len(),
        169,
        "Expected 169 Bot API 9.6 methods, found {}. \
         Update this list if the spec has changed.",
        methods.len()
    );

    // Verify no duplicates
    let mut seen = std::collections::HashSet::new();
    for (api_name, rust_name) in &methods {
        assert!(
            seen.insert(api_name),
            "Duplicate API method: {api_name} -> {rust_name}"
        );
    }
}

// ===========================================================================
// #3b: All Bot API 9.6 types exist and deserialize from minimal JSON
// ===========================================================================

fn with_large_stack<F, R>(f: F) -> R
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    std::thread::Builder::new()
        .stack_size(8 * 1024 * 1024)
        .spawn(f)
        .expect("thread creation should succeed")
        .join()
        .expect("thread should not panic")
}

fn assert_deserializes<T: serde::de::DeserializeOwned>(json: serde_json::Value, label: &str) {
    serde_json::from_value::<T>(json).unwrap_or_else(|e| {
        panic!("{label} deserialization failed: {e}");
    });
}

#[test]
fn all_bot_api_96_types_deserialize() {
    with_large_stack(|| {
        use rust_tg_bot_raw::types::*;

        // -- Core types --
        assert_deserializes::<update::Update>(
            json!({"update_id": 1, "message": {"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}}}),
            "Update",
        );
        assert_deserializes::<user::User>(
            json!({"id": 1, "is_bot": false, "first_name": "Test"}),
            "User",
        );
        assert_deserializes::<chat::Chat>(json!({"id": 1, "type": "private"}), "Chat");
        assert_deserializes::<message::Message>(
            json!({"message_id": 1, "date": 0, "chat": {"id": 1, "type": "private"}}),
            "Message",
        );
        assert_deserializes::<message_id::MessageId>(json!({"message_id": 42}), "MessageId");
        assert_deserializes::<message_entity::MessageEntity>(
            json!({"type": "bold", "offset": 0, "length": 4}),
            "MessageEntity",
        );

        // -- Chat-related types --
        assert_deserializes::<chat_full_info::ChatFullInfo>(
            json!({"id": 1, "type": "private", "accent_color_id": 0, "max_reaction_count": 3, "accepted_gift_types": {"unlimited_gifts": false, "limited_gifts": false, "unique_gifts": false, "premium_subscription": false, "gifts_from_channels": false}}),
            "ChatFullInfo",
        );
        assert_deserializes::<chat_permissions::ChatPermissions>(json!({}), "ChatPermissions");
        assert_deserializes::<chat_administrator_rights::ChatAdministratorRights>(
            json!({"is_anonymous": false, "can_manage_chat": true, "can_delete_messages": false, "can_manage_video_chats": false, "can_restrict_members": false, "can_promote_members": false, "can_change_info": false, "can_invite_users": false, "can_post_stories": false, "can_edit_stories": false, "can_delete_stories": false}),
            "ChatAdministratorRights",
        );
        assert_deserializes::<files::chat_photo::ChatPhoto>(
            json!({"small_file_id": "a", "small_file_unique_id": "b", "big_file_id": "c", "big_file_unique_id": "d"}),
            "ChatPhoto",
        );
        assert_deserializes::<chat_invite_link::ChatInviteLink>(
            json!({"invite_link": "https://t.me/+abc", "creator": {"id": 1, "is_bot": false, "first_name": "T"}, "creates_join_request": false, "is_primary": false, "is_revoked": false}),
            "ChatInviteLink",
        );
        assert_deserializes::<chat_join_request::ChatJoinRequest>(
            json!({"chat": {"id": 1, "type": "supergroup"}, "from": {"id": 1, "is_bot": false, "first_name": "T"}, "user_chat_id": 1, "date": 0}),
            "ChatJoinRequest",
        );
        assert_deserializes::<chat_member_updated::ChatMemberUpdated>(
            json!({
                "chat": {"id": 1, "type": "supergroup"},
                "from": {"id": 1, "is_bot": false, "first_name": "T"},
                "date": 0,
                "old_chat_member": {"status": "left", "user": {"id": 1, "is_bot": false, "first_name": "T"}},
                "new_chat_member": {"status": "member", "user": {"id": 1, "is_bot": false, "first_name": "T"}}
            }),
            "ChatMemberUpdated",
        );
        assert_deserializes::<chat_location::ChatLocation>(
            json!({"location": {"latitude": 0.0, "longitude": 0.0}, "address": "addr"}),
            "ChatLocation",
        );

        // -- Media types --
        assert_deserializes::<files::photo_size::PhotoSize>(
            json!({"file_id": "f", "file_unique_id": "u", "width": 100, "height": 100}),
            "PhotoSize",
        );
        assert_deserializes::<files::animation::Animation>(
            json!({"file_id": "f", "file_unique_id": "u", "width": 320, "height": 240, "duration": 5}),
            "Animation",
        );
        assert_deserializes::<files::audio::Audio>(
            json!({"file_id": "f", "file_unique_id": "u", "duration": 120}),
            "Audio",
        );
        assert_deserializes::<files::document::Document>(
            json!({"file_id": "f", "file_unique_id": "u"}),
            "Document",
        );
        assert_deserializes::<files::video::Video>(
            json!({"file_id": "f", "file_unique_id": "u", "width": 1920, "height": 1080, "duration": 60}),
            "Video",
        );
        assert_deserializes::<files::video_note::VideoNote>(
            json!({"file_id": "f", "file_unique_id": "u", "length": 240, "duration": 10}),
            "VideoNote",
        );
        assert_deserializes::<files::voice::Voice>(
            json!({"file_id": "f", "file_unique_id": "u", "duration": 5}),
            "Voice",
        );
        assert_deserializes::<files::contact::Contact>(
            json!({"phone_number": "+1234", "first_name": "John"}),
            "Contact",
        );
        assert_deserializes::<files::location::Location>(
            json!({"latitude": 51.5, "longitude": -0.1}),
            "Location",
        );
        assert_deserializes::<files::venue::Venue>(
            json!({"location": {"latitude": 51.5, "longitude": -0.1}, "title": "T", "address": "A"}),
            "Venue",
        );
        assert_deserializes::<files::file::File>(
            json!({"file_id": "f", "file_unique_id": "u"}),
            "File",
        );

        // -- Sticker types --
        assert_deserializes::<files::sticker::Sticker>(
            json!({"file_id": "f", "file_unique_id": "u", "type": "regular", "width": 512, "height": 512, "is_animated": false, "is_video": false}),
            "Sticker",
        );
        assert_deserializes::<files::sticker::StickerSet>(
            json!({"name": "set", "title": "T", "sticker_type": "regular", "stickers": []}),
            "StickerSet",
        );

        // -- Dice, Poll, Story --
        assert_deserializes::<dice::Dice>(json!({"emoji": "\u{1F3B2}", "value": 3}), "Dice");
        assert_deserializes::<poll::Poll>(
            json!({"id": "p", "question": "Q?", "options": [], "total_voter_count": 0, "is_closed": false, "is_anonymous": true, "type": "regular", "allows_multiple_answers": false}),
            "Poll",
        );
        assert_deserializes::<poll::PollAnswer>(
            json!({"poll_id": "p", "option_ids": [0], "user": {"id": 1, "is_bot": false, "first_name": "T"}}),
            "PollAnswer",
        );
        assert_deserializes::<story::Story>(
            json!({"chat": {"id": 1, "type": "channel"}, "id": 1}),
            "Story",
        );

        // -- Callback query --
        assert_deserializes::<callback_query::CallbackQuery>(
            json!({"id": "q", "from": {"id": 1, "is_bot": false, "first_name": "T"}, "chat_instance": "ci"}),
            "CallbackQuery",
        );

        // -- Inline types --
        assert_deserializes::<inline::inline_query::InlineQuery>(
            json!({"id": "iq", "from": {"id": 1, "is_bot": false, "first_name": "T"}, "query": "hello", "offset": ""}),
            "InlineQuery",
        );
        assert_deserializes::<inline::inline_keyboard_button::InlineKeyboardButton>(
            json!({"text": "Click"}),
            "InlineKeyboardButton",
        );
        assert_deserializes::<inline::inline_keyboard_markup::InlineKeyboardMarkup>(
            json!({"inline_keyboard": [[{"text": "A"}]]}),
            "InlineKeyboardMarkup",
        );
        assert_deserializes::<chosen_inline_result::ChosenInlineResult>(
            json!({"result_id": "r", "from": {"id": 1, "is_bot": false, "first_name": "T"}, "query": "q"}),
            "ChosenInlineResult",
        );

        // -- Payment types --
        assert_deserializes::<payment::invoice::Invoice>(
            json!({"title": "T", "description": "D", "start_parameter": "s", "currency": "USD", "total_amount": 100}),
            "Invoice",
        );
        assert_deserializes::<payment::successful_payment::SuccessfulPayment>(
            json!({"currency": "USD", "total_amount": 100, "invoice_payload": "p", "telegram_payment_charge_id": "tpc", "provider_payment_charge_id": "ppc"}),
            "SuccessfulPayment",
        );
        assert_deserializes::<payment::shipping_query::ShippingQuery>(
            json!({"id": "sq", "from": {"id": 1, "is_bot": false, "first_name": "T"}, "invoice_payload": "p", "shipping_address": {"country_code": "US", "state": "CA", "city": "SF", "street_line1": "1", "street_line2": "", "post_code": "94102"}}),
            "ShippingQuery",
        );
        assert_deserializes::<payment::pre_checkout_query::PreCheckoutQuery>(
            json!({"id": "pcq", "from": {"id": 1, "is_bot": false, "first_name": "T"}, "currency": "USD", "total_amount": 100, "invoice_payload": "p"}),
            "PreCheckoutQuery",
        );
        assert_deserializes::<payment::shipping_address::ShippingAddress>(
            json!({"country_code": "US", "state": "CA", "city": "SF", "street_line1": "1", "street_line2": "", "post_code": "94102"}),
            "ShippingAddress",
        );
        assert_deserializes::<payment::order_info::OrderInfo>(json!({}), "OrderInfo");
        assert_deserializes::<payment::labeled_price::LabeledPrice>(
            json!({"label": "L", "amount": 100}),
            "LabeledPrice",
        );
        assert_deserializes::<payment::shipping_option::ShippingOption>(
            json!({"id": "so", "title": "T", "prices": []}),
            "ShippingOption",
        );
        assert_deserializes::<payment::refunded_payment::RefundedPayment>(
            json!({"currency": "XTR", "total_amount": 100, "invoice_payload": "p", "telegram_payment_charge_id": "tpc"}),
            "RefundedPayment",
        );

        // -- Star types --
        assert_deserializes::<payment::stars::star_amount::StarAmount>(
            json!({"amount": 100, "nanostar_amount": 0}),
            "StarAmount",
        );
        assert_deserializes::<payment::stars::star_transactions::StarTransactions>(
            json!({"transactions": []}),
            "StarTransactions",
        );

        // -- Passport types --
        assert_deserializes::<passport::passport_data::PassportData>(
            json!({"data": [], "credentials": {"data": "enc", "hash": "h", "secret": "s"}}),
            "PassportData",
        );
        assert_deserializes::<passport::passport_file::PassportFile>(
            json!({"file_id": "f", "file_unique_id": "u", "file_size": 1024, "file_date": 0}),
            "PassportFile",
        );
        assert_deserializes::<passport::encrypted_passport_element::EncryptedPassportElement>(
            json!({"type": "personal_details", "hash": "h"}),
            "EncryptedPassportElement",
        );

        // -- Game types --
        assert_deserializes::<games::game::Game>(
            json!({"title": "T", "description": "D", "photo": []}),
            "Game",
        );
        assert_deserializes::<games::game_high_score::GameHighScore>(
            json!({"position": 1, "user": {"id": 1, "is_bot": false, "first_name": "T"}, "score": 100}),
            "GameHighScore",
        );

        // -- Keyboard types --
        assert_deserializes::<keyboard_button::KeyboardButton>(
            json!({"text": "Click"}),
            "KeyboardButton",
        );
        assert_deserializes::<reply_keyboard_markup::ReplyKeyboardMarkup>(
            json!({"keyboard": [[{"text": "A"}]]}),
            "ReplyKeyboardMarkup",
        );
        assert_deserializes::<reply_keyboard_remove::ReplyKeyboardRemove>(
            json!({"remove_keyboard": true}),
            "ReplyKeyboardRemove",
        );
        assert_deserializes::<force_reply::ForceReply>(json!({"force_reply": true}), "ForceReply");

        // -- Bot description types --
        assert_deserializes::<bot_command::BotCommand>(
            json!({"command": "start", "description": "Start the bot"}),
            "BotCommand",
        );
        assert_deserializes::<bot_description::BotDescription>(
            json!({"description": "A bot"}),
            "BotDescription",
        );
        assert_deserializes::<bot_name::BotName>(json!({"name": "TestBot"}), "BotName");

        // -- WebApp types --
        assert_deserializes::<web_app_info::WebAppInfo>(
            json!({"url": "https://example.com"}),
            "WebAppInfo",
        );
        assert_deserializes::<web_app_data::WebAppData>(
            json!({"data": "d", "button_text": "bt"}),
            "WebAppData",
        );
        assert_deserializes::<sent_web_app_message::SentWebAppMessage>(
            json!({}),
            "SentWebAppMessage",
        );

        // -- Forum types --
        assert_deserializes::<forum_topic::ForumTopic>(
            json!({"message_thread_id": 1, "name": "Topic", "icon_color": 0}),
            "ForumTopic",
        );

        // -- Login URL --
        assert_deserializes::<login_url::LoginUrl>(
            json!({"url": "https://example.com"}),
            "LoginUrl",
        );

        // -- Webhook info --
        assert_deserializes::<webhook_info::WebhookInfo>(
            json!({"url": "", "has_custom_certificate": false, "pending_update_count": 0}),
            "WebhookInfo",
        );

        // -- Response parameters --
        assert_deserializes::<response_parameters::ResponseParameters>(
            json!({}),
            "ResponseParameters",
        );

        // -- Business types --
        assert_deserializes::<business::BusinessConnection>(
            json!({"id": "bc", "user": {"id": 1, "is_bot": false, "first_name": "T"}, "user_chat_id": 1, "date": 0, "can_reply": true, "is_enabled": true}),
            "BusinessConnection",
        );
        assert_deserializes::<business::BusinessMessagesDeleted>(
            json!({"business_connection_id": "bc", "chat": {"id": 1, "type": "private"}, "message_ids": [1]}),
            "BusinessMessagesDeleted",
        );

        // -- Gift types --
        assert_deserializes::<gifts::Gifts>(json!({"gifts": []}), "Gifts");
        assert_deserializes::<gifts::Gift>(
            json!({"id": "g", "sticker": {"file_id": "f", "file_unique_id": "u", "type": "regular", "width": 512, "height": 512, "is_animated": false, "is_video": false}, "star_count": 100}),
            "Gift",
        );
        assert_deserializes::<gifts::AcceptedGiftTypes>(
            json!({"unlimited_gifts": true, "limited_gifts": false, "unique_gifts": false, "premium_subscription": false, "gifts_from_channels": false}),
            "AcceptedGiftTypes",
        );

        // -- Giveaway types --
        assert_deserializes::<giveaway::Giveaway>(
            json!({"chats": [{"id": 1, "type": "channel"}], "winners_selection_date": 0, "winner_count": 1}),
            "Giveaway",
        );
        assert_deserializes::<giveaway::GiveawayWinners>(
            json!({"chat": {"id": 1, "type": "channel"}, "giveaway_message_id": 1, "winners_selection_date": 0, "winner_count": 1, "winners": []}),
            "GiveawayWinners",
        );

        // -- Reaction types --
        assert_deserializes::<reaction::ReactionType>(
            json!({"type": "emoji", "emoji": "\u{1F44D}"}),
            "ReactionType",
        );
        assert_deserializes::<message_reaction_updated::MessageReactionUpdated>(
            json!({"chat": {"id": 1, "type": "private"}, "message_id": 1, "date": 0, "old_reaction": [], "new_reaction": []}),
            "MessageReactionUpdated",
        );
        assert_deserializes::<message_reaction_updated::MessageReactionCountUpdated>(
            json!({"chat": {"id": 1, "type": "private"}, "message_id": 1, "date": 0, "reactions": []}),
            "MessageReactionCountUpdated",
        );

        // -- Chat boost types --
        assert_deserializes::<chat_boost::ChatBoostUpdated>(
            json!({"chat": {"id": 1, "type": "channel"}, "boost": {"boost_id": "b", "add_date": 0, "expiration_date": 0, "source": {"source": "premium", "user": {"id": 1, "is_bot": false, "first_name": "T"}}}}),
            "ChatBoostUpdated",
        );

        // -- Birthdate --
        assert_deserializes::<birthdate::Birthdate>(json!({"day": 1, "month": 1}), "Birthdate");

        // -- User profile --
        assert_deserializes::<user_profile_photos::UserProfilePhotos>(
            json!({"total_count": 0, "photos": []}),
            "UserProfilePhotos",
        );

        // -- Link preview --
        assert_deserializes::<link_preview_options::LinkPreviewOptions>(
            json!({}),
            "LinkPreviewOptions",
        );

        // -- Reply parameters --
        assert_deserializes::<reply::ReplyParameters>(json!({"message_id": 1}), "ReplyParameters");

        // -- Message origin --
        assert_deserializes::<message_origin::MessageOrigin>(
            json!({"type": "user", "date": 0, "sender_user": {"id": 1, "is_bot": false, "first_name": "T"}}),
            "MessageOrigin",
        );

        // -- Inline query results button --
        assert_deserializes::<inline::inline_query_results_button::InlineQueryResultsButton>(
            json!({"text": "btn"}),
            "InlineQueryResultsButton",
        );

        // -- Prepared types --
        assert_deserializes::<prepared_keyboard_button::PreparedKeyboardButton>(
            json!({"id": "r"}),
            "PreparedKeyboardButton",
        );
        assert_deserializes::<inline::prepared_inline_message::PreparedInlineMessage>(
            json!({"id": "pim", "expiration_date": 0}),
            "PreparedInlineMessage",
        );

        // -- Paid media types --
        assert_deserializes::<paid_media::PaidMediaInfo>(
            json!({"star_count": 5, "paid_media": []}),
            "PaidMediaInfo",
        );
        assert_deserializes::<paid_media::PaidMediaPurchased>(
            json!({"from": {"id": 1, "is_bot": false, "first_name": "T"}, "paid_media_payload": "p"}),
            "PaidMediaPurchased",
        );

        // -- Switch inline query chosen chat --
        assert_deserializes::<switch_inline_query_chosen_chat::SwitchInlineQueryChosenChat>(
            json!({}),
            "SwitchInlineQueryChosenChat",
        );

        // -- Copy text button --
        assert_deserializes::<copy_text_button::CopyTextButton>(
            json!({"text": "copy me"}),
            "CopyTextButton",
        );

        // -- Chat background --
        assert_deserializes::<chat_background::ChatBackground>(
            json!({"type": {"type": "fill", "fill": {"type": "solid", "color": 0}, "dark_theme_dimming": 0}}),
            "ChatBackground",
        );

        // -- Write access allowed --
        assert_deserializes::<write_access_allowed::WriteAccessAllowed>(
            json!({}),
            "WriteAccessAllowed",
        );

        // -- Proximity alert --
        assert_deserializes::<proximity_alert_triggered::ProximityAlertTriggered>(
            json!({"traveler": {"id": 1, "is_bot": false, "first_name": "T"}, "watcher": {"id": 2, "is_bot": false, "first_name": "W"}, "distance": 100}),
            "ProximityAlertTriggered",
        );

        // -- Message auto-delete timer changed --
        assert_deserializes::<message_auto_delete_timer_changed::MessageAutoDeleteTimerChanged>(
            json!({"message_auto_delete_time": 86400}),
            "MessageAutoDeleteTimerChanged",
        );

        // -- Video chat types --
        assert_deserializes::<video_chat::VideoChatStarted>(json!({}), "VideoChatStarted");
        assert_deserializes::<video_chat::VideoChatEnded>(
            json!({"duration": 120}),
            "VideoChatEnded",
        );
        assert_deserializes::<video_chat::VideoChatScheduled>(
            json!({"start_date": 0}),
            "VideoChatScheduled",
        );

        // -- Menu button --
        assert_deserializes::<menu_button::MenuButton>(json!({"type": "default"}), "MenuButton");

        // -- Keyboard button poll type --
        assert_deserializes::<keyboard_button_poll_type::KeyboardButtonPollType>(
            json!({}),
            "KeyboardButtonPollType",
        );

        // -- Shared types --
        assert_deserializes::<shared::SharedUser>(json!({"user_id": 1}), "SharedUser");

        // -- Checklist types --
        assert_deserializes::<checklists::Checklist>(
            json!({"title": "T", "tasks": []}),
            "Checklist",
        );
        assert_deserializes::<input_checklist::InputChecklist>(
            json!({"title": "T", "tasks": []}),
            "InputChecklist",
        );

        // -- Direct messages topic --
        assert_deserializes::<direct_messages_topic::DirectMessagesTopic>(
            json!({"topic_id": 1}),
            "DirectMessagesTopic",
        );

        // -- Suggested post --
        assert_deserializes::<suggested_post::SuggestedPostParameters>(
            json!({}),
            "SuggestedPostParameters",
        );

        // -- Managed bot --
        assert_deserializes::<managed_bot::ManagedBotUpdated>(
            json!({"user": {"id": 2, "is_bot": false, "first_name": "U"}, "bot": {"id": 1, "is_bot": true, "first_name": "B"}}),
            "ManagedBotUpdated",
        );

        // -- Story area --
        assert_deserializes::<story_area::StoryArea>(
            json!({"position": {"x_percentage": 0.0, "y_percentage": 0.0, "width_percentage": 100.0, "height_percentage": 100.0, "rotation_angle": 0.0, "corner_radius_percentage": 0.0}, "type": {"type": "suggested_reaction", "reaction_type": {"type": "emoji", "emoji": "\u{1F44D}"}}}),
            "StoryArea",
        );

        // -- User rating --
        assert_deserializes::<user_rating::UserRating>(
            json!({"level": 1, "rating": 50, "current_level_rating": 0}),
            "UserRating",
        );

        // -- Owned gift --
        assert_deserializes::<owned_gift::OwnedGifts>(
            json!({"total_count": 0, "gifts": []}),
            "OwnedGifts",
        );

        // -- Unique gift --
        assert_deserializes::<unique_gift::UniqueGift>(
            json!({"gift_id": "gid", "base_name": "g", "name": "n", "number": 1, "model": {"name": "m", "sticker": {"file_id": "f", "file_unique_id": "u", "type": "regular", "width": 512, "height": 512, "is_animated": false, "is_video": false}, "rarity_per_mille": 100}, "symbol": {"name": "s", "sticker": {"file_id": "f", "file_unique_id": "u", "type": "regular", "width": 512, "height": 512, "is_animated": false, "is_video": false}, "rarity_per_mille": 100}, "backdrop": {"name": "b", "colors": {"center_color": 0, "edge_color": 0, "symbol_color": 0, "text_color": 0}, "rarity_per_mille": 100}}),
            "UniqueGift",
        );

        // -- Paid message price changed --
        assert_deserializes::<paid_message_price_changed::PaidMessagePriceChanged>(
            json!({"paid_message_star_count": 5}),
            "PaidMessagePriceChanged",
        );
    });
}
