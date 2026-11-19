//! Round-trip serialization tests against realistic Telegram Bot API JSON payloads.
//!
//! Each test: deserialize from JSON string -> serialize back -> compare that all
//! important fields survive the round-trip. We do not require byte-exact JSON
//! (field ordering may differ), but verify all critical fields match.

use rust_tg_bot_raw::types::callback_query::CallbackQuery;
use rust_tg_bot_raw::types::chat::Chat;
use rust_tg_bot_raw::types::chat_full_info::ChatFullInfo;
use rust_tg_bot_raw::types::chat_member::ChatMember;
use rust_tg_bot_raw::types::chat_permissions::ChatPermissions;
use rust_tg_bot_raw::types::guest::SentGuestMessage;
use rust_tg_bot_raw::types::inline::inline_query::InlineQuery;
use rust_tg_bot_raw::types::managed_bot::BotAccessSettings;
use rust_tg_bot_raw::types::message::Message;
use rust_tg_bot_raw::types::poll::Poll;
use rust_tg_bot_raw::types::update::Update;
use rust_tg_bot_raw::types::user::User;
use serde_json::json;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Deserialize T from a serde_json::Value, then serialize back and compare
/// that the two Value trees agree on all fields present in the original.
fn roundtrip_check<T>(original: serde_json::Value) -> T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    let deserialized: T =
        serde_json::from_value(original.clone()).expect("deserialization should succeed");
    let reserialized = serde_json::to_value(&deserialized).expect("reserialization should succeed");

    // Every key in the original must exist in the reserialized output.
    if let (Some(orig_obj), Some(reser_obj)) = (original.as_object(), reserialized.as_object()) {
        for (key, orig_val) in orig_obj {
            let reser_val = reser_obj.get(key);
            assert!(
                reser_val.is_some(),
                "Key '{}' missing after round-trip. Original: {:?}",
                key,
                orig_val
            );
            assert_eq!(
                orig_val,
                reser_val.unwrap(),
                "Mismatch for key '{}' after round-trip",
                key
            );
        }
    }

    deserialized
}

/// Run a closure on a thread with a large stack (8 MiB) to avoid stack
/// overflows when serializing the very large Update/Message types in debug
/// mode.
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

// ===========================================================================
// User
// ===========================================================================

#[test]
fn roundtrip_user_minimal() {
    let json = json!({
        "id": 123456789,
        "is_bot": false,
        "first_name": "John"
    });
    let user: User = roundtrip_check(json);
    assert_eq!(user.id, 123456789);
    assert!(!user.is_bot);
    assert_eq!(user.first_name, "John");
    assert!(user.last_name.is_none());
    assert!(user.username.is_none());
}

#[test]
fn roundtrip_user_all_fields() {
    let json = json!({
        "id": 987654321,
        "is_bot": true,
        "first_name": "TestBot",
        "last_name": "Bot",
        "username": "test_bot",
        "language_code": "en",
        "can_join_groups": true,
        "can_read_all_group_messages": false,
        "supports_inline_queries": true,
        "is_premium": true,
        "added_to_attachment_menu": false,
        "can_connect_to_business": true,
        "has_main_web_app": false
    });
    let user: User = roundtrip_check(json);
    assert_eq!(user.id, 987654321);
    assert!(user.is_bot);
    assert_eq!(user.first_name, "TestBot");
    assert_eq!(user.last_name.as_deref(), Some("Bot"));
    assert_eq!(user.username.as_deref(), Some("test_bot"));
    assert_eq!(user.language_code.as_deref(), Some("en"));
    assert_eq!(user.can_join_groups, Some(true));
    assert_eq!(user.can_read_all_group_messages, Some(false));
    assert_eq!(user.supports_inline_queries, Some(true));
    assert_eq!(user.is_premium, Some(true));
    assert_eq!(user.added_to_attachment_menu, Some(false));
    assert_eq!(user.can_connect_to_business, Some(true));
    assert_eq!(user.has_main_web_app, Some(false));
}

// ===========================================================================
// Chat
// ===========================================================================

#[test]
fn roundtrip_chat_private() {
    let json = json!({
        "id": 123456789,
        "type": "private",
        "first_name": "John",
        "last_name": "Doe",
        "username": "johndoe"
    });
    let chat: Chat = roundtrip_check(json);
    assert_eq!(chat.id, 123456789);
    assert_eq!(chat.chat_type, "private");
    assert_eq!(chat.first_name.as_deref(), Some("John"));
    assert_eq!(chat.last_name.as_deref(), Some("Doe"));
    assert_eq!(chat.username.as_deref(), Some("johndoe"));
    assert!(chat.title.is_none());
}

#[test]
fn roundtrip_chat_supergroup() {
    let json = json!({
        "id": -1001234567890i64,
        "type": "supergroup",
        "title": "Test Group",
        "username": "testgroup",
        "is_forum": true
    });
    let chat: Chat = roundtrip_check(json);
    assert_eq!(chat.id, -1001234567890);
    assert_eq!(chat.chat_type, "supergroup");
    assert_eq!(chat.title.as_deref(), Some("Test Group"));
    assert_eq!(chat.is_forum, Some(true));
}

#[test]
fn roundtrip_chat_channel() {
    let json = json!({
        "id": -1001987654321i64,
        "type": "channel",
        "title": "Test Channel",
        "username": "testchannel"
    });
    let chat: Chat = roundtrip_check(json);
    assert_eq!(chat.id, -1001987654321);
    assert_eq!(chat.chat_type, "channel");
    assert_eq!(chat.title.as_deref(), Some("Test Channel"));
}

// ===========================================================================
// ChatFullInfo
// ===========================================================================

#[test]
fn roundtrip_chat_full_info() {
    let json = json!({
        "id": -1001234567890i64,
        "type": "supergroup",
        "title": "Dev Chat",
        "username": "devchat",
        "is_forum": true,
        "max_reaction_count": 11,
        "accent_color_id": 2,
        "accepted_gift_types": {
            "unlimited_gifts": true,
            "limited_gifts": false,
            "unique_gifts": true,
            "premium_subscription": false,
            "gifts_from_channels": true
        }
    });
    let info: ChatFullInfo = roundtrip_check(json);
    assert_eq!(info.id, -1001234567890);
    assert_eq!(info.chat_type, "supergroup");
    assert_eq!(info.title.as_deref(), Some("Dev Chat"));
    assert_eq!(info.is_forum, Some(true));
    assert_eq!(info.max_reaction_count, 11);
    assert_eq!(info.accent_color_id, 2);
    assert!(info.accepted_gift_types.unlimited_gifts);
    assert!(!info.accepted_gift_types.limited_gifts);
    assert!(info.accepted_gift_types.unique_gifts);
}

// ===========================================================================
// Message -- text with entities
// ===========================================================================

#[test]
fn roundtrip_message_text_with_entities() {
    with_large_stack(|| {
        let json = json!({
            "message_id": 42,
            "date": 1700000000,
            "chat": {
                "id": 123456789,
                "type": "private",
                "first_name": "John"
            },
            "from": {
                "id": 123456789,
                "is_bot": false,
                "first_name": "John"
            },
            "text": "Hello @bot /start",
            "entities": [
                {
                    "type": "mention",
                    "offset": 6,
                    "length": 4
                },
                {
                    "type": "bot_command",
                    "offset": 11,
                    "length": 6
                }
            ]
        });
        let msg: Message = roundtrip_check(json);
        assert_eq!(msg.message_id, 42);
        assert_eq!(msg.date, 1700000000);
        assert_eq!(msg.chat.id, 123456789);
        assert_eq!(msg.chat.chat_type, "private");
        assert_eq!(msg.from_user.as_ref().unwrap().id, 123456789);
        assert_eq!(msg.text.as_deref(), Some("Hello @bot /start"));

        let entities = msg.entities.as_ref().unwrap();
        assert_eq!(entities.len(), 2);
        assert_eq!(entities[0].entity_type, "mention");
        assert_eq!(entities[0].offset, 6);
        assert_eq!(entities[0].length, 4);
        assert_eq!(entities[1].entity_type, "bot_command");
    });
}

// ===========================================================================
// Message -- photo
// ===========================================================================

#[test]
fn roundtrip_message_photo() {
    with_large_stack(|| {
        let json = json!({
            "message_id": 100,
            "date": 1700001000,
            "chat": {
                "id": 123456789,
                "type": "private",
                "first_name": "John"
            },
            "from": {
                "id": 123456789,
                "is_bot": false,
                "first_name": "John"
            },
            "photo": [
                {
                    "file_id": "AgACAgIAAxkBAAIBZ2abc",
                    "file_unique_id": "AQADAgATabc",
                    "width": 90,
                    "height": 90,
                    "file_size": 1234
                },
                {
                    "file_id": "AgACAgIAAxkBAAIBZ2def",
                    "file_unique_id": "AQADAgATdef",
                    "width": 320,
                    "height": 320,
                    "file_size": 12345
                }
            ],
            "caption": "Check this out!",
            "caption_entities": [
                {
                    "type": "bold",
                    "offset": 0,
                    "length": 5
                }
            ]
        });
        let msg: Message = roundtrip_check(json);
        assert_eq!(msg.message_id, 100);
        let photos = msg.photo.as_ref().unwrap();
        assert_eq!(photos.len(), 2);
        assert_eq!(photos[0].width, 90);
        assert_eq!(photos[0].height, 90);
        assert_eq!(photos[0].file_size, Some(1234));
        assert_eq!(photos[1].width, 320);
        assert_eq!(msg.caption.as_deref(), Some("Check this out!"));
        let cap_entities = msg.caption_entities.as_ref().unwrap();
        assert_eq!(cap_entities.len(), 1);
        assert_eq!(cap_entities[0].entity_type, "bold");
    });
}

// ===========================================================================
// Message -- document
// ===========================================================================

#[test]
fn roundtrip_message_document() {
    with_large_stack(|| {
        let json = json!({
            "message_id": 200,
            "date": 1700002000,
            "chat": {
                "id": 123456789,
                "type": "private",
                "first_name": "John"
            },
            "from": {
                "id": 123456789,
                "is_bot": false,
                "first_name": "John"
            },
            "document": {
                "file_id": "BQACAgIAAxkBAAIBZ2abc",
                "file_unique_id": "AgADabc",
                "file_name": "report.pdf",
                "mime_type": "application/pdf",
                "file_size": 1048576,
                "thumbnail": {
                    "file_id": "AAMCAgADGQEAAgFnZabc",
                    "file_unique_id": "AQADAgATxyz",
                    "width": 320,
                    "height": 320,
                    "file_size": 5000
                }
            },
            "caption": "Here is the report"
        });
        let msg: Message = roundtrip_check(json);
        assert_eq!(msg.message_id, 200);
        let doc = msg.document.as_ref().unwrap();
        assert_eq!(doc.file_id, "BQACAgIAAxkBAAIBZ2abc");
        assert_eq!(doc.file_name.as_deref(), Some("report.pdf"));
        assert_eq!(doc.mime_type.as_deref(), Some("application/pdf"));
        assert_eq!(doc.file_size, Some(1048576));
        let thumb = doc.thumbnail.as_ref().unwrap();
        assert_eq!(thumb.width, 320);
        assert_eq!(thumb.height, 320);
    });
}

// ===========================================================================
// Message -- forward_origin
// ===========================================================================

#[test]
fn roundtrip_message_forward_origin() {
    with_large_stack(|| {
        let json = json!({
            "message_id": 300,
            "date": 1700003000,
            "chat": {
                "id": 123456789,
                "type": "private",
                "first_name": "John"
            },
            "from": {
                "id": 123456789,
                "is_bot": false,
                "first_name": "John"
            },
            "forward_origin": {
                "type": "user",
                "date": 1699999000,
                "sender_user": {
                    "id": 999888777,
                    "is_bot": false,
                    "first_name": "Alice"
                }
            },
            "text": "Forwarded message content"
        });
        let msg: Message = roundtrip_check(json);
        assert_eq!(msg.message_id, 300);
        let origin = msg.forward_origin.as_ref().unwrap();
        match origin {
            rust_tg_bot_raw::types::message_origin::MessageOrigin::User(data) => {
                assert_eq!(data.date, 1699999000);
                assert_eq!(data.sender_user.id, 999888777);
                assert_eq!(data.sender_user.first_name, "Alice");
            }
            other => panic!("Expected MessageOrigin::User, got {:?}", other),
        }
    });
}

// ===========================================================================
// Message -- reply_to_message
// ===========================================================================

#[test]
fn roundtrip_message_reply_to_message() {
    with_large_stack(|| {
        let json = json!({
            "message_id": 400,
            "date": 1700004000,
            "chat": {
                "id": 123456789,
                "type": "private",
                "first_name": "John"
            },
            "from": {
                "id": 123456789,
                "is_bot": false,
                "first_name": "John"
            },
            "text": "This is a reply",
            "reply_to_message": {
                "message_id": 399,
                "date": 1700003900,
                "chat": {
                    "id": 123456789,
                    "type": "private",
                    "first_name": "John"
                },
                "from": {
                    "id": 123456789,
                    "is_bot": false,
                    "first_name": "John"
                },
                "text": "Original message"
            }
        });
        let msg: Message = roundtrip_check(json);
        assert_eq!(msg.message_id, 400);
        assert_eq!(msg.text.as_deref(), Some("This is a reply"));
        let reply = msg.reply_to_message.as_ref().unwrap();
        assert_eq!(reply.message_id, 399);
        assert_eq!(reply.text.as_deref(), Some("Original message"));
    });
}

// ===========================================================================
// Message -- all common fields combined
// ===========================================================================

#[test]
fn roundtrip_message_common_fields() {
    with_large_stack(|| {
        let json = json!({
            "message_id": 500,
            "date": 1700005000,
            "chat": {
                "id": -1001234567890i64,
                "type": "supergroup",
                "title": "Test Group"
            },
            "from": {
                "id": 123456789,
                "is_bot": false,
                "first_name": "John",
                "username": "johndoe",
                "language_code": "en"
            },
            "message_thread_id": 42,
            "is_topic_message": true,
            "text": "Hello world",
            "entities": [],
            "edit_date": 1700005100,
            "has_protected_content": true,
            "media_group_id": "media-group-123",
            "author_signature": "Admin"
        });
        let msg: Message = roundtrip_check(json);
        assert_eq!(msg.message_id, 500);
        assert_eq!(msg.chat.id, -1001234567890);
        assert_eq!(msg.chat.chat_type, "supergroup");
        assert_eq!(
            msg.from_user.as_ref().unwrap().username.as_deref(),
            Some("johndoe")
        );
        assert_eq!(msg.message_thread_id, Some(42));
        assert!(msg.is_topic_message);
        assert_eq!(msg.edit_date, Some(1700005100));
        assert!(msg.has_protected_content);
        assert_eq!(msg.media_group_id.as_deref(), Some("media-group-123"));
        assert_eq!(msg.author_signature.as_deref(), Some("Admin"));
    });
}

// ===========================================================================
// Update -- with message (text)
// ===========================================================================

#[test]
fn roundtrip_update_with_text_message() {
    with_large_stack(|| {
        let json = json!({
            "update_id": 100000001,
            "message": {
                "message_id": 42,
                "date": 1700000000,
                "chat": {
                    "id": 123456789,
                    "type": "private",
                    "first_name": "John"
                },
                "from": {
                    "id": 123456789,
                    "is_bot": false,
                    "first_name": "John"
                },
                "text": "/start",
                "entities": [
                    {
                        "type": "bot_command",
                        "offset": 0,
                        "length": 6
                    }
                ]
            }
        });
        let update: Update = roundtrip_check(json);
        assert_eq!(update.update_id, 100000001);
        let msg = update.message().unwrap();
        assert_eq!(msg.message_id, 42);
        assert_eq!(msg.text.as_deref(), Some("/start"));

        // Test effective_* helpers
        let user = update.effective_user().unwrap();
        assert_eq!(user.id, 123456789);
        let chat = update.effective_chat().unwrap();
        assert_eq!(chat.id, 123456789);
        let eff_msg = update.effective_message().unwrap();
        assert_eq!(eff_msg.message_id, 42);
    });
}

// ===========================================================================
// Update -- with callback_query (with message)
// ===========================================================================

#[test]
fn roundtrip_update_with_callback_query() {
    with_large_stack(|| {
        let json = json!({
            "update_id": 100000002,
            "callback_query": {
                "id": "4382bfdwdsb323b2d9",
                "from": {
                    "id": 123456789,
                    "is_bot": false,
                    "first_name": "John",
                    "username": "johndoe"
                },
                "chat_instance": "-1234567890",
                "message": {
                    "message_id": 55,
                    "date": 1700000100,
                    "chat": {
                        "id": 123456789,
                        "type": "private",
                        "first_name": "John"
                    },
                    "from": {
                        "id": 987654321,
                        "is_bot": true,
                        "first_name": "TestBot"
                    },
                    "text": "Choose an option:",
                    "reply_markup": {
                        "inline_keyboard": [
                            [
                                {"text": "Option 1", "callback_data": "opt1"},
                                {"text": "Option 2", "callback_data": "opt2"}
                            ]
                        ]
                    }
                },
                "data": "opt1"
            }
        });
        let update: Update = roundtrip_check(json);
        assert_eq!(update.update_id, 100000002);
        let cbq = update.callback_query().unwrap();
        assert_eq!(cbq.id, "4382bfdwdsb323b2d9");
        assert_eq!(cbq.from_user.id, 123456789);
        assert_eq!(cbq.from_user.username.as_deref(), Some("johndoe"));
        assert_eq!(cbq.chat_instance, "-1234567890");
        assert_eq!(cbq.data.as_deref(), Some("opt1"));

        // The message inside the callback query
        let msg = cbq.message.as_ref().unwrap();
        assert_eq!(msg.message_id(), 55);
        let inner_msg = msg.as_message().unwrap();
        assert_eq!(inner_msg.text.as_deref(), Some("Choose an option:"));

        // effective_user should be the callback query sender
        let user = update.effective_user().unwrap();
        assert_eq!(user.id, 123456789);
    });
}

// ===========================================================================
// Update -- with inline_query
// ===========================================================================

#[test]
fn roundtrip_update_with_inline_query() {
    with_large_stack(|| {
        let json = json!({
            "update_id": 100000003,
            "inline_query": {
                "id": "12345678901234567",
                "from": {
                    "id": 123456789,
                    "is_bot": false,
                    "first_name": "John",
                    "username": "johndoe",
                    "language_code": "en"
                },
                "query": "search term",
                "offset": "",
                "chat_type": "sender"
            }
        });
        let update: Update = roundtrip_check(json);
        assert_eq!(update.update_id, 100000003);
        let iq = update.inline_query().unwrap();
        assert_eq!(iq.id, "12345678901234567");
        assert_eq!(iq.from_user.id, 123456789);
        assert_eq!(iq.query, "search term");
        assert_eq!(iq.offset, "");
        assert_eq!(iq.chat_type.as_deref(), Some("sender"));

        // effective_user
        let user = update.effective_user().unwrap();
        assert_eq!(user.id, 123456789);

        // inline queries have no effective_chat
        assert!(update.effective_chat().is_none());
    });
}

// ===========================================================================
// CallbackQuery -- standalone
// ===========================================================================

#[test]
fn roundtrip_callback_query_with_data() {
    with_large_stack(|| {
        let json = json!({
            "id": "unique_cbq_id_999",
            "from": {
                "id": 111222333,
                "is_bot": false,
                "first_name": "Alice"
            },
            "chat_instance": "567890123",
            "message": {
                "message_id": 77,
                "date": 1700000200,
                "chat": {
                    "id": 111222333,
                    "type": "private",
                    "first_name": "Alice"
                },
                "text": "Pick something"
            },
            "data": "button_callback_data"
        });
        let cbq: CallbackQuery = roundtrip_check(json);
        assert_eq!(cbq.id, "unique_cbq_id_999");
        assert_eq!(cbq.from_user.id, 111222333);
        assert_eq!(cbq.from_user.first_name, "Alice");
        assert_eq!(cbq.chat_instance, "567890123");
        assert_eq!(cbq.data.as_deref(), Some("button_callback_data"));
        assert!(cbq.inline_message_id.is_none());
        assert!(cbq.game_short_name.is_none());

        let msg = cbq.message.as_ref().unwrap();
        assert_eq!(msg.message_id(), 77);
    });
}

#[test]
fn roundtrip_callback_query_inline_mode() {
    let json = json!({
        "id": "inline_cbq_id",
        "from": {
            "id": 444555666,
            "is_bot": false,
            "first_name": "Bob"
        },
        "chat_instance": "998877665544",
        "inline_message_id": "BQAAAAAAAAAAAA",
        "data": "inline_btn_data"
    });
    let cbq: CallbackQuery = roundtrip_check(json);
    assert_eq!(cbq.id, "inline_cbq_id");
    assert_eq!(cbq.inline_message_id.as_deref(), Some("BQAAAAAAAAAAAA"));
    assert!(cbq.message.is_none());
}

// ===========================================================================
// InlineQuery
// ===========================================================================

#[test]
fn roundtrip_inline_query_minimal() {
    let json = json!({
        "id": "query_abc123",
        "from": {
            "id": 999888777,
            "is_bot": false,
            "first_name": "Charlie"
        },
        "query": "test query",
        "offset": "10"
    });
    let iq: InlineQuery = roundtrip_check(json);
    assert_eq!(iq.id, "query_abc123");
    assert_eq!(iq.from_user.id, 999888777);
    assert_eq!(iq.query, "test query");
    assert_eq!(iq.offset, "10");
    assert!(iq.chat_type.is_none());
    assert!(iq.location.is_none());
}

#[test]
fn roundtrip_inline_query_with_location() {
    let json = json!({
        "id": "query_loc_456",
        "from": {
            "id": 111000111,
            "is_bot": false,
            "first_name": "Dave"
        },
        "query": "nearby restaurants",
        "offset": "",
        "chat_type": "private",
        "location": {
            "latitude": 40.7128,
            "longitude": -74.0060
        }
    });
    let iq: InlineQuery = roundtrip_check(json);
    assert_eq!(iq.id, "query_loc_456");
    assert_eq!(iq.query, "nearby restaurants");
    assert_eq!(iq.chat_type.as_deref(), Some("private"));
    let loc = iq.location.as_ref().unwrap();
    assert!((loc.latitude - 40.7128).abs() < 0.001);
    assert!((loc.longitude - (-74.0060)).abs() < 0.001);
}

// ===========================================================================
// Update -- with edited_message
// ===========================================================================

#[test]
fn roundtrip_update_edited_message() {
    with_large_stack(|| {
        let json = json!({
            "update_id": 100000004,
            "edited_message": {
                "message_id": 60,
                "date": 1700000500,
                "chat": {
                    "id": 123456789,
                    "type": "private",
                    "first_name": "John"
                },
                "from": {
                    "id": 123456789,
                    "is_bot": false,
                    "first_name": "John"
                },
                "text": "Edited text",
                "edit_date": 1700000600
            }
        });
        let update: Update = roundtrip_check(json);
        assert_eq!(update.update_id, 100000004);
        let msg = update.edited_message().unwrap();
        assert_eq!(msg.message_id, 60);
        assert_eq!(msg.text.as_deref(), Some("Edited text"));
        assert_eq!(msg.edit_date, Some(1700000600));

        // effective_message should return edited_message
        let eff = update.effective_message().unwrap();
        assert_eq!(eff.message_id, 60);
    });
}

// ===========================================================================
// Update -- channel_post
// ===========================================================================

#[test]
fn roundtrip_update_channel_post() {
    with_large_stack(|| {
        let json = json!({
            "update_id": 100000005,
            "channel_post": {
                "message_id": 70,
                "date": 1700001000,
                "chat": {
                    "id": -1001987654321i64,
                    "type": "channel",
                    "title": "News Channel"
                },
                "text": "Breaking news!",
                "author_signature": "Editor"
            }
        });
        let update: Update = roundtrip_check(json);
        assert_eq!(update.update_id, 100000005);
        let post = update.channel_post().unwrap();
        assert_eq!(post.message_id, 70);
        assert_eq!(post.chat.chat_type, "channel");
        assert_eq!(post.text.as_deref(), Some("Breaking news!"));
        assert_eq!(post.author_signature.as_deref(), Some("Editor"));

        // Channel posts have no effective_user
        assert!(update.effective_user().is_none());
        // But have an effective_chat
        let chat = update.effective_chat().unwrap();
        assert_eq!(chat.id, -1001987654321);
    });
}

// ===========================================================================
// Update -- callback_query with inaccessible message
// ===========================================================================

#[test]
fn roundtrip_update_callback_query_inaccessible_message() {
    with_large_stack(|| {
        let json = json!({
            "update_id": 100000006,
            "callback_query": {
                "id": "cbq_inaccessible",
                "from": {
                    "id": 123456789,
                    "is_bot": false,
                    "first_name": "John"
                },
                "chat_instance": "999",
                "message": {
                    "message_id": 80,
                    "date": 0,
                    "chat": {
                        "id": 123456789,
                        "type": "private",
                        "first_name": "John"
                    }
                },
                "data": "old_button"
            }
        });
        let update: Update = roundtrip_check(json);
        let cbq = update.callback_query().unwrap();
        let msg = cbq.message.as_ref().unwrap();
        assert_eq!(msg.message_id(), 80);
    });
}

// ===========================================================================
// Update -- unknown fields are silently dropped (serde default behaviour)
// ===========================================================================

#[test]
fn roundtrip_update_unknown_fields_dropped() {
    with_large_stack(|| {
        let json = json!({
            "update_id": 100000007,
            "message": {
                "message_id": 90,
                "date": 1700009000,
                "chat": {
                    "id": 123456789,
                    "type": "private",
                    "first_name": "John"
                },
                "text": "test"
            },
            "some_future_field": {"nested": true}
        });
        // Deserialization succeeds even with unknown fields.
        let update: Update = serde_json::from_value(json)
            .expect("deserialization should succeed with unknown fields");
        assert_eq!(update.update_id, 100000007);
        // Unknown fields are silently dropped -- no extra HashMap.
        let reserialized = serde_json::to_value(&update).expect("reserialization should succeed");
        assert!(
            reserialized.get("some_future_field").is_none(),
            "Unknown field 'some_future_field' should be dropped during deserialization"
        );
    });
}

// ===========================================================================
// Deserialization of empty/null optional fields
// ===========================================================================

#[test]
fn roundtrip_update_minimal_message() {
    with_large_stack(|| {
        let json = json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 0,
                "chat": {
                    "id": 1,
                    "type": "private"
                }
            }
        });
        let update: Update = roundtrip_check(json);
        assert_eq!(update.update_id, 1);
        let msg = update.message().unwrap();
        assert_eq!(msg.message_id, 1);
        assert!(msg.from_user.is_none());
        assert!(msg.text.is_none());
        assert!(msg.entities.is_none());
        assert!(msg.photo.is_none());
        assert!(msg.document.is_none());
    });
}

// ===========================================================================
// Large Update with multiple nested objects
// ===========================================================================

#[test]
fn roundtrip_complex_message_with_reply_markup() {
    with_large_stack(|| {
        let json = json!({
            "update_id": 100000010,
            "message": {
                "message_id": 999,
                "date": 1700010000,
                "chat": {
                    "id": -1001234567890i64,
                    "type": "supergroup",
                    "title": "Test Group",
                    "is_forum": true
                },
                "from": {
                    "id": 123456789,
                    "is_bot": false,
                    "first_name": "John",
                    "last_name": "Doe",
                    "username": "johndoe",
                    "language_code": "en",
                    "is_premium": true
                },
                "message_thread_id": 42,
                "is_topic_message": true,
                "text": "Hello with keyboard",
                "reply_markup": {
                    "inline_keyboard": [
                        [
                            {"text": "Button 1", "callback_data": "btn1"},
                            {"text": "Button 2", "url": "https://example.com"}
                        ],
                        [
                            {"text": "Button 3", "callback_data": "btn3"}
                        ]
                    ]
                }
            }
        });
        let update: Update = roundtrip_check(json);
        let msg = update.message().unwrap();
        assert_eq!(msg.message_id, 999);
        assert_eq!(msg.message_thread_id, Some(42));
        assert!(msg.is_topic_message);
        let from = msg.from_user.as_ref().unwrap();
        assert_eq!(from.is_premium, Some(true));
        assert!(msg.reply_markup.is_some());
    });
}

// ===========================================================================
// Guest mode, live photos, poll media, reactions (10.0 surface)
// ===========================================================================

#[test]
fn roundtrip_message_live_photo_and_guest_fields() {
    with_large_stack(|| {
        let json = json!({
            "message_id": 7,
            "date": 1780000000,
            "chat": {"id": -1001, "type": "supergroup", "title": "Club"},
            "from": {"id": 11, "is_bot": true, "first_name": "GuestBot"},
            "guest_query_id": "gq-1",
            "guest_bot_caller_user": {"id": 22, "is_bot": false, "first_name": "Pat"},
            "guest_bot_caller_chat": {"id": -1001, "type": "supergroup"},
            "live_photo": {
                "file_id": "lp_vid",
                "file_unique_id": "lp_uniq",
                "width": 1080,
                "height": 1920,
                "duration": 3,
                "photo": [{"file_id": "lp_img", "file_unique_id": "lp_img_u", "width": 1080, "height": 1920}]
            },
            "photo": [{"file_id": "lp_img", "file_unique_id": "lp_img_u", "width": 1080, "height": 1920}]
        });
        let msg: Message = roundtrip_check(json);
        assert_eq!(msg.guest_query_id.as_deref(), Some("gq-1"));
        assert_eq!(msg.guest_bot_caller_user.as_ref().unwrap().id, 22);
        assert_eq!(msg.guest_bot_caller_chat.as_ref().unwrap().id, -1001);
        let lp = msg.live_photo.as_ref().unwrap();
        assert_eq!(lp.file_id, "lp_vid");
        assert_eq!(lp.duration, 3);
        assert_eq!(lp.photo.as_ref().unwrap().len(), 1);
    });
}

#[test]
fn roundtrip_poll_with_media() {
    let json = json!({
        "id": "poll1",
        "question": "Pick one",
        "options": [
            {"text": "A", "voter_count": 3, "media": {"photo": [{"file_id": "p", "file_unique_id": "pu", "width": 1, "height": 1}]}},
            {"text": "B", "voter_count": 0}
        ],
        "total_voter_count": 3,
        "is_closed": false,
        "is_anonymous": false,
        "type": "regular",
        "allows_multiple_answers": false,
        "members_only": true,
        "country_codes": ["US", "FT"],
        "media": {"video": {"file_id": "v", "file_unique_id": "vu", "width": 640, "height": 480, "duration": 10}}
    });
    let poll: Poll = roundtrip_check(json);
    assert!(poll.members_only == Some(true));
    assert_eq!(
        poll.country_codes.as_ref().unwrap(),
        &vec!["US".to_string(), "FT".to_string()]
    );
    assert!(poll.media.as_ref().unwrap().video.is_some());
    assert!(poll.options[0].media.as_ref().unwrap().photo.is_some());
    assert!(poll.options[1].media.is_none());
}

#[test]
fn roundtrip_update_guest_message() {
    with_large_stack(|| {
        let json = json!({
            "update_id": 9001,
            "guest_message": {
                "message_id": 1,
                "date": 1780000001,
                "chat": {"id": -1001, "type": "supergroup"},
                "from": {"id": 11, "is_bot": true, "first_name": "GuestBot"},
                "guest_query_id": "gq-9",
                "text": "At your service"
            }
        });
        let update: Update = roundtrip_check(json);
        let msg = update.guest_message().unwrap();
        assert_eq!(msg.guest_query_id.as_deref(), Some("gq-9"));
        assert_eq!(update.message().unwrap().message_id, 1);
        assert_eq!(update.effective_chat().unwrap().id, -1001);
        assert_eq!(update.effective_user().unwrap().id, 11);
        assert_eq!(
            update.effective_message().unwrap().text.as_deref(),
            Some("At your service")
        );
    });
}

#[test]
fn roundtrip_bot_access_settings_and_sent_guest_message() {
    let settings_json = json!({
        "is_access_restricted": true,
        "added_users": [{"id": 5, "is_bot": false, "first_name": "Ann"}]
    });
    let settings: BotAccessSettings = roundtrip_check(settings_json);
    assert!(settings.is_access_restricted);
    assert_eq!(settings.added_users.as_ref().unwrap().len(), 1);

    let sent_json = json!({"inline_message_id": "im-1"});
    let sent: SentGuestMessage = roundtrip_check(sent_json);
    assert_eq!(sent.inline_message_id, "im-1");
}

#[test]
fn roundtrip_react_permissions() {
    let json = json!({
        "status": "restricted",
        "user": {"id": 33, "is_bot": false, "first_name": "Sam"},
        "is_member": true,
        "can_change_info": false,
        "can_invite_users": true,
        "can_pin_messages": false,
        "can_send_messages": true,
        "can_send_polls": true,
        "can_send_other_messages": false,
        "can_add_web_page_previews": true,
        "can_react_to_messages": true,
        "can_send_audios": true,
        "can_send_documents": true,
        "can_send_photos": true,
        "can_send_videos": true,
        "can_send_video_notes": false,
        "can_send_voice_notes": true,
        "can_edit_tag": false,
        "can_manage_topics": false,
        "until_date": 0
    });
    let member: ChatMember = roundtrip_check(json);
    match &member {
        ChatMember::Restricted(r) => assert!(r.can_react_to_messages),
        other => panic!("expected restricted member, got {:?}", other),
    }

    let perms_json = json!({"can_send_messages": true, "can_react_to_messages": false});
    let perms: ChatPermissions = roundtrip_check(perms_json);
    assert_eq!(perms.can_react_to_messages, Some(false));
    assert!(ChatPermissions::all().can_react_to_messages == Some(true));
    assert!(ChatPermissions::none().can_react_to_messages == Some(false));
}

#[test]
fn poll_media_input_serializes_with_type_tag() {
    use rust_tg_bot_raw::types::files::input_file::InputFile;
    use rust_tg_bot_raw::types::files::input_media::InputMediaPhoto;
    use rust_tg_bot_raw::types::poll::InputPollMedia;

    let media = InputPollMedia::Photo(InputMediaPhoto::new(InputFile::file_id("photo_file_id")));
    let value = serde_json::to_value(&media).unwrap();
    assert_eq!(value["type"], "photo");
    assert_eq!(value["media"], "photo_file_id");
}

// ===========================================================================
// Rich text
// ===========================================================================

#[test]
fn rich_text_plain_string_and_array() {
    use rust_tg_bot_raw::types::rich_text::RichText;

    let plain: RichText = serde_json::from_value(json!("hello")).unwrap();
    assert!(matches!(plain, RichText::Text(_)));
    assert_eq!(serde_json::to_value(&plain).unwrap(), json!("hello"));

    let array: RichText =
        serde_json::from_value(json!(["a", {"type": "bold", "text": "b"}])).unwrap();
    match &array {
        RichText::Array(parts) => {
            assert_eq!(parts.len(), 2);
            assert!(matches!(parts[0], RichText::Text(_)));
            assert!(matches!(parts[1], RichText::Formatted(_)));
        }
        other => panic!("expected array, got {:?}", other),
    }
    // Full round-trip preserves the untagged shape.
    let reserialized = serde_json::to_value(&array).unwrap();
    assert_eq!(reserialized, json!(["a", {"type": "bold", "text": "b"}]));
}

#[test]
fn rich_text_nested_formatting_dispatches_on_tag() {
    use rust_tg_bot_raw::types::rich_text::{RichText, RichTextFormatted};

    // Bold containing italic must not collapse into the first untagged variant.
    let nested: RichText = serde_json::from_value(json!({
        "type": "bold",
        "text": {"type": "italic", "text": "deep"}
    }))
    .unwrap();
    match &nested {
        RichText::Formatted(inner) => match inner.as_ref() {
            RichTextFormatted::Bold(bold) => match bold.text.as_ref() {
                RichText::Formatted(inner2) => {
                    assert!(matches!(inner2.as_ref(), RichTextFormatted::Italic(_)));
                }
                other => panic!("expected nested formatted, got {:?}", other),
            },
            other => panic!("expected bold, got {:?}", other),
        },
        other => panic!("expected formatted, got {:?}", other),
    }
    // Tag survives the round-trip.
    let reserialized = serde_json::to_value(&nested).unwrap();
    assert_eq!(reserialized["type"], "bold");
    assert_eq!(reserialized["text"]["type"], "italic");
}

#[test]
fn rich_text_all_member_shapes() {
    use rust_tg_bot_raw::types::rich_text::{RichText, RichTextFormatted};

    let cases = [
        (json!({"type": "underline", "text": "u"}), "underline"),
        (
            json!({"type": "strikethrough", "text": "s"}),
            "strikethrough",
        ),
        (json!({"type": "spoiler", "text": "s"}), "spoiler"),
        (json!({"type": "subscript", "text": "s"}), "subscript"),
        (json!({"type": "superscript", "text": "s"}), "superscript"),
        (json!({"type": "marked", "text": "m"}), "marked"),
        (json!({"type": "code", "text": "c"}), "code"),
        (
            json!({"type": "mathematical_expression", "expression": "x^2"}),
            "mathematical_expression",
        ),
        (json!({"type": "anchor", "name": "ch1"}), "anchor"),
        (
            json!({"type": "custom_emoji", "custom_emoji_id": "e1", "alternative_text": "🙂"}),
            "custom_emoji",
        ),
        (
            json!({"type": "date_time", "text": "now", "unix_time": 1780000000, "date_time_format": "wDT"}),
            "date_time",
        ),
        (
            json!({"type": "url", "text": "t", "url": "https://t.me"}),
            "url",
        ),
        (
            json!({"type": "mention", "text": "t", "username": "durov"}),
            "mention",
        ),
        (
            json!({"type": "bot_command", "text": "/start", "bot_command": "/start"}),
            "bot_command",
        ),
        (
            json!({"type": "anchor_link", "text": "t", "anchor_name": "ch1"}),
            "anchor_link",
        ),
        (
            json!({"type": "reference", "text": "t", "name": "n1"}),
            "reference",
        ),
        (
            json!({"type": "reference_link", "text": "t", "reference_name": "n1"}),
            "reference_link",
        ),
        (json!({"type": "button", "button": {"text": "b"}}), "button"),
    ];
    for (payload, tag) in cases {
        let parsed: RichText = serde_json::from_value(payload).unwrap_or_else(|e| {
            panic!("{} failed to deserialize: {}", tag, e);
        });
        assert!(
            matches!(parsed, RichText::Formatted(_)),
            "{} did not dispatch to Formatted",
            tag
        );
        let reserialized = serde_json::to_value(&parsed).unwrap();
        assert_eq!(reserialized["type"], tag, "tag lost for {}", tag);
    }

    // User-carrying member.
    let mention: RichText = serde_json::from_value(json!({
        "type": "text_mention",
        "text": "Pat",
        "user": {"id": 22, "is_bot": false, "first_name": "Pat"}
    }))
    .unwrap();
    match mention {
        RichText::Formatted(inner) => match *inner {
            RichTextFormatted::TextMention(m) => assert_eq!(m.user.id, 22),
            other => panic!("expected text mention, got {:?}", other),
        },
        other => panic!("expected formatted, got {:?}", other),
    }
}

// ===========================================================================
// Rich blocks
// ===========================================================================

#[test]
fn rich_block_tags_dispatch_including_renames() {
    use rust_tg_bot_raw::types::rich_block::RichBlock;

    // (payload, expected wire tag); covers every renamed tag.
    let cases = [
        (json!({"type": "paragraph", "text": "p"}), "paragraph"),
        (
            json!({"type": "heading", "text": "h", "size": 2}),
            "heading",
        ),
        (json!({"type": "pre", "text": "c"}), "pre"),
        (json!({"type": "footer", "text": "f"}), "footer"),
        (json!({"type": "divider"}), "divider"),
        (
            json!({"type": "mathematical_expression", "expression": "x"}),
            "mathematical_expression",
        ),
        (json!({"type": "anchor", "name": "a"}), "anchor"),
        (json!({"type": "list", "items": []}), "list"),
        (json!({"type": "blockquote", "blocks": []}), "blockquote"),
        (
            json!({"type": "expandable_blockquote", "text": "q"}),
            "expandable_blockquote",
        ),
        (json!({"type": "pullquote", "text": "q"}), "pullquote"),
        (json!({"type": "collage", "blocks": []}), "collage"),
        (json!({"type": "slideshow", "blocks": []}), "slideshow"),
        (json!({"type": "table", "cells": []}), "table"),
        (
            json!({"type": "details", "summary": "s", "blocks": []}),
            "details",
        ),
        (
            json!({"type": "map", "location": {"latitude": 1.0, "longitude": 2.0}, "zoom": 14, "width": 600, "height": 400}),
            "map",
        ),
        (json!({"type": "buttons", "buttons": []}), "buttons"),
        (json!({"type": "thinking", "text": "..."}), "thinking"),
    ];
    for (payload, tag) in cases {
        let parsed: RichBlock = serde_json::from_value(payload).unwrap_or_else(|e| {
            panic!("{} failed to deserialize: {}", tag, e);
        });
        let reserialized = serde_json::to_value(&parsed).unwrap();
        assert_eq!(reserialized["type"], tag, "tag lost for {}", tag);
    }
}

#[test]
fn rich_block_table_list_item_and_details() {
    use rust_tg_bot_raw::types::rich_block::RichBlock;

    // Compact table with a header cell and a list item carrying checkbox state.
    let table: RichBlock = serde_json::from_value(json!({
        "type": "table",
        "cells": [[
            {"text": "H", "is_header": true, "align": "left", "valign": "top"},
            {"align": "center", "valign": "middle"}
        ]],
        "is_bordered": true,
        "is_compact": true,
        "caption": "cap"
    }))
    .unwrap();
    match &table {
        RichBlock::Table(t) => {
            assert!(t.is_compact == Some(true));
            assert_eq!(t.cells[0][0].align, "left");
            assert!(t.cells[0][1].text.is_none());
        }
        other => panic!("expected table, got {:?}", other),
    }
    let reserialized = serde_json::to_value(&table).unwrap();
    assert_eq!(reserialized["type"], "table");

    // List item: JSON "type" maps to item_type; checkbox state survives.
    let list: RichBlock = serde_json::from_value(json!({
        "type": "list",
        "items": [{
            "label": "task",
            "blocks": [{"type": "paragraph", "text": "do it"}],
            "has_checkbox": true,
            "is_checked": true,
            "type": "1"
        }]
    }))
    .unwrap();
    match &list {
        RichBlock::List(l) => {
            let item = &l.items[0];
            assert_eq!(item.item_type.as_deref(), Some("1"));
            assert!(item.has_checkbox == Some(true));
            assert!(matches!(item.blocks[0], RichBlock::Paragraph(_)));
        }
        other => panic!("expected list, got {:?}", other),
    }
    let reserialized = serde_json::to_value(&list).unwrap();
    assert_eq!(reserialized["items"][0]["type"], "1");

    // Details with nested quotation.
    let details: RichBlock = serde_json::from_value(json!({
        "type": "details",
        "summary": "more",
        "blocks": [{"type": "blockquote", "blocks": [], "credit": "anon"}],
        "is_open": true
    }))
    .unwrap();
    match &details {
        RichBlock::Details(d) => {
            assert!(d.is_open == Some(true));
            assert!(matches!(d.blocks[0], RichBlock::BlockQuotation(_)));
        }
        other => panic!("expected details, got {:?}", other),
    }
}

// ===========================================================================
// Rich message, inputs, join-request and link types
// ===========================================================================

#[test]
fn rich_message_nested_blocks_roundtrip() {
    use rust_tg_bot_raw::types::rich_message::RichMessage;

    with_large_stack(|| {
        let json = json!({
            "blocks": [
                {"type": "heading", "text": "Title", "size": 1},
                {"type": "paragraph", "text": [
                    "Hello ",
                    {"type": "bold", "text": "world"},
                    {"type": "mention", "text": "@durov", "username": "durov"}
                ]},
                {"type": "list", "items": [
                    {"label": "one", "blocks": [{"type": "paragraph", "text": "1"}]}
                ]}
            ],
            "is_rtl": false
        });
        let msg: RichMessage = roundtrip_check(json);
        assert_eq!(msg.blocks.len(), 3);
        let reserialized = serde_json::to_value(&msg).unwrap();
        assert_eq!(reserialized["blocks"][0]["type"], "heading");
        assert_eq!(reserialized["blocks"][1]["text"][1]["type"], "bold");
    });
}

#[test]
fn input_rich_message_and_content_variants() {
    use rust_tg_bot_raw::types::inline::input_message_content::InputMessageContent;
    use rust_tg_bot_raw::types::inline::input_rich_message_content::InputRichMessageContent;
    use rust_tg_bot_raw::types::input_rich_message::InputRichMessage;

    // HTML input with entity-detection skip.
    let html_input = InputRichMessage::html("<b>Hi</b>");
    let value = serde_json::to_value(&html_input).unwrap();
    assert_eq!(value["html"], "<b>Hi</b>");
    assert!(value.get("markdown").is_none());

    let md_input = InputRichMessage::markdown("**Hi**");
    let value = serde_json::to_value(&md_input).unwrap();
    assert_eq!(value["markdown"], "**Hi**");

    // Rich content dispatches inside the untagged InputMessageContent union.
    let content = InputMessageContent::Rich(InputRichMessageContent::new(html_input));
    let value = serde_json::to_value(&content).unwrap();
    assert_eq!(value["rich_message"]["html"], "<b>Hi</b>");

    // And deserializes back to the Rich variant (unique key, no ambiguity).
    let back: InputMessageContent = serde_json::from_value(value).unwrap();
    assert!(matches!(back, InputMessageContent::Rich(_)));
}

#[test]
fn link_and_input_media_link() {
    use rust_tg_bot_raw::types::files::input_media::InputMediaLink;
    use rust_tg_bot_raw::types::link::Link;
    use rust_tg_bot_raw::types::poll::InputPollOptionMedia;

    let link: Link = roundtrip_check(json!({"url": "https://t.me"}));
    assert_eq!(link.url, "https://t.me");

    let media = InputPollOptionMedia::Link(InputMediaLink::new("https://t.me"));
    let value = serde_json::to_value(&media).unwrap();
    assert_eq!(value["type"], "link");
    assert_eq!(value["url"], "https://t.me");
}

#[test]
fn join_request_query_and_guard_bot_fields() {
    with_large_stack(|| {
        // ChatJoinRequest with a join request query id.
        let json = json!({
            "update_id": 4242,
            "chat_join_request": {
                "chat": {"id": -1005, "type": "supergroup"},
                "from": {"id": 77, "is_bot": false, "first_name": "Jo"},
                "user_chat_id": 77,
                "date": 1780000000,
                "query_id": "jrq-1"
            }
        });
        let update: Update = roundtrip_check(json);
        let req = update.chat_join_request().unwrap();
        assert_eq!(req.query_id.as_deref(), Some("jrq-1"));

        // ChatFullInfo carrying the guard bot.
        let full_json = json!({
            "id": -1005,
            "type": "supergroup",
            "accent_color_id": 1,
            "max_reaction_count": 20,
            "accepted_gift_types": {
                "unlimited_gifts": true,
                "limited_gifts": true,
                "unique_gifts": true,
                "premium_subscription": true,
                "gifts_from_channels": false
            },
            "guard_bot": {"id": 99, "is_bot": true, "first_name": "Guard"}
        });
        let full: ChatFullInfo = roundtrip_check(full_json);
        assert_eq!(full.guard_bot.as_ref().unwrap().id, 99);

        // getMe-style user flags.
        let user: User = roundtrip_check(json!({
            "id": 99, "is_bot": true, "first_name": "Guard",
            "supports_join_request_queries": true
        }));
        assert_eq!(user.supports_join_request_queries, Some(true));
    });
}

#[test]
fn message_rich_message_field() {
    with_large_stack(|| {
        let json = json!({
            "message_id": 3,
            "date": 1780000002,
            "chat": {"id": 1, "type": "private"},
            "rich_message": {"blocks": [{"type": "paragraph", "text": "hi"}]}
        });
        let msg: Message = roundtrip_check(json);
        let rich = msg.rich_message.as_ref().unwrap();
        assert_eq!(rich.blocks.len(), 1);
    });
}

// ===========================================================================
// Rich input blocks
// ===========================================================================

#[test]
fn input_rich_block_tags_dispatch() {
    use rust_tg_bot_raw::types::input_rich_block::InputRichBlock;

    let cases = [
        (json!({"type": "paragraph", "text": "p"}), "paragraph"),
        (
            json!({"type": "heading", "text": "h", "size": 1}),
            "heading",
        ),
        (
            json!({"type": "pre", "text": "c", "language": "rust"}),
            "pre",
        ),
        (json!({"type": "footer", "text": "f"}), "footer"),
        (json!({"type": "divider"}), "divider"),
        (
            json!({"type": "mathematical_expression", "expression": "x"}),
            "mathematical_expression",
        ),
        (json!({"type": "anchor", "name": "a"}), "anchor"),
        (json!({"type": "list", "items": []}), "list"),
        (json!({"type": "blockquote", "blocks": []}), "blockquote"),
        (
            json!({"type": "expandable_blockquote", "text": "q"}),
            "expandable_blockquote",
        ),
        (json!({"type": "pullquote", "text": "q"}), "pullquote"),
        (json!({"type": "collage", "blocks": []}), "collage"),
        (json!({"type": "slideshow", "blocks": []}), "slideshow"),
        (json!({"type": "table", "cells": []}), "table"),
        (
            json!({"type": "details", "summary": "s", "blocks": []}),
            "details",
        ),
        (
            json!({"type": "map", "location": {"latitude": 1.0, "longitude": 2.0}}),
            "map",
        ),
        (
            json!({"type": "animation", "animation": {"type": "animation", "media": "f"}}),
            "animation",
        ),
        (
            json!({"type": "audio", "audio": {"type": "audio", "media": "f"}}),
            "audio",
        ),
        (
            json!({"type": "photo", "photo": {"type": "photo", "media": "f"}}),
            "photo",
        ),
        (
            json!({"type": "video", "video": {"type": "video", "media": "f"}}),
            "video",
        ),
        (
            json!({"type": "voice_note", "voice_note": {"type": "voice_note", "media": "f"}}),
            "voice_note",
        ),
        (
            json!({"type": "document", "document": {"type": "document", "media": "f"}}),
            "document",
        ),
        (json!({"type": "thinking", "text": "..."}), "thinking"),
    ];
    for (payload, tag) in cases {
        let parsed: InputRichBlock = serde_json::from_value(payload).unwrap_or_else(|e| {
            panic!("{} failed to deserialize: {}", tag, e);
        });
        let reserialized = serde_json::to_value(&parsed).unwrap();
        assert_eq!(reserialized["type"], tag, "tag lost for {}", tag);
    }
}

#[test]
fn input_rich_message_blocks_and_media() {
    use rust_tg_bot_raw::types::input_rich_block::InputRichBlock;
    use rust_tg_bot_raw::types::input_rich_message::InputRichMessage;

    // Block-based message with embedded media and a labelless list item.
    let json = json!({
        "blocks": [
            {"type": "list", "items": [
                {"blocks": [{"type": "paragraph", "text": "do it"}], "is_checked": true, "type": "1"}
            ]},
            {"type": "map", "location": {"latitude": 41.9, "longitude": 12.5}, "zoom": 14}
        ],
        "media": [
            {"id": "pic1", "media": {"type": "photo", "media": "file_id_1"}}
        ],
        "is_rtl": false
    });
    let msg: InputRichMessage = roundtrip_check(json);
    let blocks = msg.blocks.as_ref().unwrap();
    assert_eq!(blocks.len(), 2);
    match &blocks[0] {
        InputRichBlock::List(list) => {
            assert_eq!(list.items[0].item_type.as_deref(), Some("1"));
            assert!(list.items[0].is_checked == Some(true));
        }
        other => panic!("expected list, got {:?}", other),
    }
    let media = msg.media.as_ref().unwrap();
    assert_eq!(media[0].id, "pic1");
    let reserialized = serde_json::to_value(&msg).unwrap();
    assert_eq!(reserialized["media"][0]["media"]["type"], "photo");
    assert_eq!(reserialized["blocks"][1]["type"], "map");
}

// ===========================================================================
// Ephemeral messages, communities, subscriptions
// ===========================================================================

#[test]
fn ephemeral_message_fields_and_reply() {
    use rust_tg_bot_raw::types::reply::ReplyParameters;

    with_large_stack(|| {
        let json = json!({
            "message_id": 0,
            "date": 1780000000,
            "chat": {"id": -1002, "type": "supergroup"},
            "receiver_user": {"id": 44, "is_bot": false, "first_name": "Ep"},
            "ephemeral_message_id": 5,
            "text": "only you see this"
        });
        let msg: Message = roundtrip_check(json);
        assert_eq!(msg.message_id, 0);
        assert_eq!(msg.receiver_user.as_ref().unwrap().id, 44);
        assert_eq!(msg.ephemeral_message_id, Some(5));

        // Ephemeral reply form: message_id absent, ephemeral id present.
        let reply: ReplyParameters = roundtrip_check(json!({"ephemeral_message_id": 5}));
        assert_eq!(reply.message_id, None);
        assert_eq!(reply.ephemeral_message_id, Some(5));
        let regular = ReplyParameters::new(10);
        assert_eq!(regular.message_id, Some(10));
        let value = serde_json::to_value(&regular).unwrap();
        assert!(value.get("ephemeral_message_id").is_none());
    });
}

#[test]
fn community_service_messages() {
    with_large_stack(|| {
        let json = json!({
            "message_id": 11,
            "date": 1780000001,
            "chat": {"id": -1002, "type": "supergroup"},
            "community_chat_added": {"community": {"id": 7, "name": "Fans"}}
        });
        let msg: Message = roundtrip_check(json);
        let added = msg.community_chat_added.as_ref().unwrap();
        assert_eq!(added.community.name, "Fans");

        // Fieldless removal marker.
        let removed_json = json!({
            "message_id": 12,
            "date": 1780000002,
            "chat": {"id": -1002, "type": "supergroup"},
            "community_chat_removed": {}
        });
        let removed: Message = roundtrip_check(removed_json);
        assert!(removed.community_chat_removed.is_some());

        // Community on full chat info.
        let full: ChatFullInfo = roundtrip_check(json!({
            "id": -1002,
            "type": "supergroup",
            "accent_color_id": 1,
            "max_reaction_count": 20,
            "accepted_gift_types": {
                "unlimited_gifts": true,
                "limited_gifts": true,
                "unique_gifts": true,
                "premium_subscription": true,
                "gifts_from_channels": false
            },
            "community": {"id": 7, "name": "Fans"}
        }));
        assert_eq!(full.community.as_ref().unwrap().id, 7);
    });
}

#[test]
fn subscription_update_plumbing() {
    with_large_stack(|| {
        let json = json!({
            "update_id": 777,
            "subscription": {
                "user": {"id": 55, "is_bot": false, "first_name": "Sub"},
                "invoice_payload": "sub-1",
                "state": "active"
            }
        });
        let update: Update = roundtrip_check(json);
        let sub = update.subscription().unwrap();
        assert_eq!(sub.state, "active");
        assert_eq!(update.effective_user().unwrap().id, 55);
        assert!(update.effective_chat().is_none());
        assert!(update.effective_message().is_none());
        assert!(update.message().is_none());
    });
}

#[test]
fn ephemeral_bot_command_flag() {
    use rust_tg_bot_raw::types::bot_command::BotCommand;

    let cmd: BotCommand =
        roundtrip_check(json!({"command": "secret", "description": "d", "is_ephemeral": true}));
    assert_eq!(cmd.is_ephemeral, Some(true));
    // Absent flag still deserializes.
    let plain: BotCommand = roundtrip_check(json!({"command": "start", "description": "d"}));
    assert_eq!(plain.is_ephemeral, None);
}

// ===========================================================================
// Rich buttons, ephemeral params, generation-stopped, communities, misc 10.3
// ===========================================================================

#[test]
fn rich_message_button_and_typed_wrappers() {
    use rust_tg_bot_raw::types::rich_block::{RichBlock, RichBlockButtons};
    use rust_tg_bot_raw::types::rich_message::RichMessageButton;
    use rust_tg_bot_raw::types::rich_text::{RichText, RichTextFormatted};

    let json = json!({
        "type": "buttons",
        "buttons": [
            {"text": "Open", "url": "https://t.me", "style": "success"},
            {"text": "Off", "disabled": {}}
        ],
        "align": "center"
    });
    let block: RichBlock = roundtrip_check(json);
    match &block {
        RichBlock::Buttons(b) => {
            assert_eq!(b.buttons.len(), 2);
            assert_eq!(b.buttons[0].url.as_deref(), Some("https://t.me"));
            assert!(b.buttons[1].disabled.is_some());
            assert_eq!(b.align.as_deref(), Some("center"));
        }
        other => panic!("expected buttons, got {:?}", other),
    }

    // Button inside rich text is now typed too.
    let text: RichText = roundtrip_check(json!({
        "type": "button",
        "button": {"text": "Go", "callback_data": "go"}
    }));
    match text {
        RichText::Formatted(inner) => match *inner {
            RichTextFormatted::Button(b) => {
                assert_eq!(b.button.callback_data.as_deref(), Some("go"));
            }
            other => panic!("expected button text, got {:?}", other),
        },
        other => panic!("expected formatted, got {:?}", other),
    }

    // Direct button round-trip keeps every field.
    let direct: RichMessageButton = roundtrip_check(json!({
        "text": "Hi",
        "login_url": {"url": "https://example.com"}
    }));
    assert!(direct.login_url.is_some());
    let _: RichBlockButtons = roundtrip_check(json!({"buttons": []}));
}

#[test]
fn input_buttons_block_and_ephemeral_params() {
    use rust_tg_bot_raw::types::ephemeral::EphemeralMessageParameters;
    use rust_tg_bot_raw::types::input_rich_block::InputRichBlock;

    let block: InputRichBlock = roundtrip_check(json!({
        "type": "buttons",
        "buttons": [{"text": "nick", "copy_text": {"text": "copy me"}}]
    }));
    match &block {
        InputRichBlock::Buttons(b) => {
            assert!(b.buttons[0].copy_text.is_some());
        }
        other => panic!("expected buttons, got {:?}", other),
    }

    let params = EphemeralMessageParameters::new(42)
        .callback_query_id("cq-1")
        .replace_original();
    let value = serde_json::to_value(&params).unwrap();
    assert_eq!(
        value,
        json!({
            "receiver_user_id": 42,
            "callback_query_id": "cq-1",
            "replace_callback_query_message": true
        })
    );
}

#[test]
fn generation_stopped_update_and_community_joined() {
    with_large_stack(|| {
        let json = json!({
            "update_id": 31337,
            "stopped_message_generation": {
                "chat": {"id": 1, "type": "private"},
                "draft_id": 9
            }
        });
        let update: Update = roundtrip_check(json);
        let stopped = update.stopped_message_generation().unwrap();
        assert_eq!(stopped.draft_id, 9);
        assert_eq!(update.effective_chat().unwrap().id, 1);
        assert!(update.effective_user().is_none());
        assert!(update.effective_message().is_none());

        let joined: Message = roundtrip_check(json!({
            "message_id": 21,
            "date": 1780000003,
            "chat": {"id": -1002, "type": "supergroup"},
            "community_chat_joined": {"community": {"id": 7, "name": "Fans"}}
        }));
        assert_eq!(
            joined
                .community_chat_joined
                .as_ref()
                .unwrap()
                .community
                .name,
            "Fans"
        );
    });
}

#[test]
fn markup_and_rights_new_flags() {
    use rust_tg_bot_raw::types::chat_administrator_rights::ChatAdministratorRights;
    use rust_tg_bot_raw::types::inline::inline_keyboard_markup::InlineKeyboardMarkup;
    use rust_tg_bot_raw::types::reply_keyboard_markup::ReplyKeyboardMarkup;
    use rust_tg_bot_raw::types::unique_gift::UniqueGiftInfo;

    // force_reply on both keyboard markups.
    let inline: InlineKeyboardMarkup =
        roundtrip_check(json!({"inline_keyboard": [], "force_reply": true}));
    assert_eq!(inline.force_reply, Some(true));
    assert!(inline.inline_keyboard.is_empty());

    let reply: ReplyKeyboardMarkup = roundtrip_check(json!({"keyboard": [], "force_reply": true}));
    assert_eq!(reply.force_reply, Some(true));

    // Welcome-message rights.
    let rights: ChatAdministratorRights = roundtrip_check(json!({
        "is_anonymous": false,
        "can_manage_chat": true,
        "can_delete_messages": true,
        "can_manage_video_chats": false,
        "can_restrict_members": false,
        "can_promote_members": false,
        "can_change_info": false,
        "can_invite_users": true,
        "can_post_stories": false,
        "can_edit_stories": false,
        "can_delete_stories": false,
        "can_send_welcome_messages": true
    }));
    assert_eq!(rights.can_send_welcome_messages, Some(true));

    // Gift info text fields.
    let gift: UniqueGiftInfo = roundtrip_check(json!({
        "gift": {
            "gift_id": "g", "base_name": "b", "name": "n", "number": 1,
            "model": {"name": "m", "sticker": {"file_id": "f", "file_unique_id": "u", "type": "regular", "width": 1, "height": 1, "is_animated": false, "is_video": false}, "rarity_per_mille": 1},
            "symbol": {"name": "s", "sticker": {"file_id": "f", "file_unique_id": "u", "type": "regular", "width": 1, "height": 1, "is_animated": false, "is_video": false}, "rarity_per_mille": 1},
            "backdrop": {"name": "b", "colors": {"center_color": 0, "edge_color": 0, "symbol_color": 0, "text_color": 0}, "rarity_per_mille": 1}
        },
        "origin": "upgrade",
        "text": "enjoy",
        "is_private": true
    }));
    assert_eq!(gift.text.as_deref(), Some("enjoy"));
    assert_eq!(gift.is_private, Some(true));
    assert!(gift.entities.is_none());
}

#[test]
fn send_builders_include_ephemeral_params_on_the_wire() {
    use rust_tg_bot_raw::types::ephemeral::EphemeralMessageParameters;

    // The object form nests under "ephemeral_message_parameters" (not flat ids).
    let params = EphemeralMessageParameters::new(7).replace_original();
    let value = serde_json::to_value(&params).unwrap();
    assert_eq!(
        value,
        json!({
            "receiver_user_id": 7,
            "replace_callback_query_message": true
        })
    );
    // Absent optionals are skipped so untouched builders send no new keys.
    let bare = EphemeralMessageParameters::new(7);
    let bare_value = serde_json::to_value(&bare).unwrap();
    assert_eq!(bare_value, json!({"receiver_user_id": 7}));
}

/// Hot-path types stay small: rarely-present large payloads must remain
/// behind `Box` so every update doesn't pay for them. Bounds have headroom;
/// they catch accidental unboxing, not byte-exact sizes.
#[test]
fn hot_types_size_bounds() {
    use rust_tg_bot_raw::types::message::Message;
    use rust_tg_bot_raw::types::poll::Poll;
    use rust_tg_bot_raw::types::reply::ExternalReplyInfo;
    use rust_tg_bot_raw::types::update::{Update, UpdateKind};
    use std::mem::size_of;

    assert!(
        size_of::<Message>() <= 6400,
        "Message grew: {}",
        size_of::<Message>()
    );
    assert!(
        size_of::<Update>() <= 6464,
        "Update grew: {}",
        size_of::<Update>()
    );
    assert!(
        size_of::<UpdateKind>() <= 6400,
        "UpdateKind grew: {}",
        size_of::<UpdateKind>()
    );
    assert!(size_of::<Poll>() <= 512, "Poll grew: {}", size_of::<Poll>());
    assert!(
        size_of::<ExternalReplyInfo>() <= 3584,
        "ExternalReplyInfo grew: {}",
        size_of::<ExternalReplyInfo>()
    );
}

// ===========================================================================
// Bot API 10.4 -- HTML widgets
// ===========================================================================

#[test]
fn roundtrip_message_widget() {
    with_large_stack(|| {
        let json = json!({
            "message_id": 501,
            "date": 1_700_000_000,
            "chat": {"id": 42, "type": "private"},
            "widget": {
                "id": "shared-widget-1",
                "width": 320,
                "height": 120,
                "html": "<div>hi</div>",
                "public_data": "pub",
                "chat_data": "chat"
            },
            "caption": "a widget",
            "show_caption_above_media": true
        });
        let msg: Message = roundtrip_check(json);
        let widget = msg.widget.as_ref().expect("widget should parse");
        assert_eq!(widget.id.as_deref(), Some("shared-widget-1"));
        assert_eq!(widget.width, Some(320));
        assert_eq!(widget.height, Some(120));
        assert_eq!(widget.html, "<div>hi</div>");
        assert_eq!(widget.public_data.as_deref(), Some("pub"));
        assert_eq!(widget.chat_data.as_deref(), Some("chat"));
        assert_eq!(msg.caption.as_deref(), Some("a widget"));
        assert!(msg.show_caption_above_media);
    });
}

#[test]
fn roundtrip_message_ton_wallet_transfer_and_waived() {
    with_large_stack(|| {
        let json = json!({
            "message_id": 502,
            "date": 1_700_000_100,
            "chat": {"id": 42, "type": "private"},
            "ton_wallet_transfer": {
                "transaction_id": "tx-1",
                "peer_address": "EQpeer",
                "amount": "-1000000",
                "comment": "thanks"
            },
            "paid_messages_waived": {}
        });
        let msg: Message = roundtrip_check(json);
        let transfer = msg
            .ton_wallet_transfer
            .as_ref()
            .expect("transfer should parse");
        assert_eq!(transfer.transaction_id, "tx-1");
        assert_eq!(transfer.peer_address, "EQpeer");
        assert_eq!(transfer.amount, "-1000000");
        assert_eq!(transfer.comment.as_deref(), Some("thanks"));
        assert!(msg.paid_messages_waived.is_some());
    });
}

#[test]
fn roundtrip_external_reply_widget() {
    let external: rust_tg_bot_raw::types::reply::ExternalReplyInfo = roundtrip_check(json!({
        "origin": {"type": "user", "date": 1_700_000_000, "sender_user": {"id": 5, "is_bot": false, "first_name": "Ann"}},
        "widget": {"html": "<div/>"}
    }));
    let widget = external.widget.as_ref().expect("widget should parse");
    assert_eq!(widget.html, "<div/>");
    assert!(widget.id.is_none());
}

// ===========================================================================
// Bot API 10.4 -- widget queries
// ===========================================================================

#[test]
fn roundtrip_update_widget_query() {
    with_large_stack(|| {
        let json = json!({
            "update_id": 4242,
            "widget_query": {
                "id": "wq-1",
                "from": {"id": 77, "is_bot": false, "first_name": "Pat"},
                "chat": {"id": 77, "type": "private"},
                "message_id": 10,
                "chat_instance": "-42",
                "type": {
                    "type": "data",
                    "wallet_card": {
                        "address": "EQwallet",
                        "amount": "1000",
                        "stickers": []
                    }
                },
                "data": "payload",
                "widget_id": "w1"
            }
        });
        let update: Update = roundtrip_check(json);
        let query = update.widget_query().expect("widget_query should parse");
        assert_eq!(query.id, "wq-1");
        assert_eq!(query.data, "payload");
        assert_eq!(query.message_id, Some(10));
        assert_eq!(query.chat_instance, "-42");
        assert_eq!(query.widget_id.as_deref(), Some("w1"));
        assert_eq!(update.effective_user().unwrap().id, 77);
        assert_eq!(update.effective_chat().unwrap().id, 77);
        assert!(update.effective_message().is_none());
        assert!(matches!(
            update.kind,
            rust_tg_bot_raw::types::update::UpdateKind::WidgetQuery(_)
        ));
        match &query.query_type {
            rust_tg_bot_raw::types::widget_query::WidgetQueryType::Data(data) => {
                let card = data.wallet_card.as_ref().expect("wallet card");
                assert_eq!(card.address, "EQwallet");
                assert!(card.stickers.is_empty());
            }
            other => panic!("expected data query type, got {other:?}"),
        }
    });
}

#[test]
fn roundtrip_update_shared_widget_query() {
    with_large_stack(|| {
        let json = json!({
            "update_id": 4243,
            "shared_widget_query": {
                "id": "swq-1",
                "from": {"id": 78, "is_bot": false, "first_name": "Sam"},
                "type": {"type": "sent_transaction", "wallet": {"address": "EQw", "amount": "5"}, "msg_hash": "hash"},
                "data": "d",
                "widget_id": "shared-1"
            }
        });
        let update: Update = roundtrip_check(json);
        let query = update.shared_widget_query().expect("shared query");
        assert_eq!(query.widget_id, "shared-1");
        assert_eq!(update.effective_user().unwrap().id, 78);
        assert!(update.effective_chat().is_none());
        match &query.query_type {
            rust_tg_bot_raw::types::widget_query::WidgetQueryType::SentTransaction(tx) => {
                assert_eq!(tx.msg_hash, "hash");
                assert_eq!(tx.wallet.amount, "5");
            }
            other => panic!("expected sent_transaction, got {other:?}"),
        }
    });
}

#[test]
fn widget_query_type_sign_data_roundtrip() {
    use rust_tg_bot_raw::types::widget_query::{WidgetQueryType, WidgetQueryWallet};

    let signed =
        WidgetQueryType::sign_data(WidgetQueryWallet::new("EQw", "12"), "{\"signature\":\"s\"}");
    let value = serde_json::to_value(&signed).unwrap();
    assert_eq!(value["type"], "sign_data");
    let back: WidgetQueryType = serde_json::from_value(value).unwrap();
    assert!(matches!(back, WidgetQueryType::SignData(_)));

    let plain: WidgetQueryType =
        serde_json::from_value(json!({"type": "data"})).expect("bare data query type");
    assert!(matches!(plain, WidgetQueryType::Data(_)));
}

// ===========================================================================
// Bot API 10.4 -- TON wallet cards and chat share themes
// ===========================================================================

#[test]
fn roundtrip_chat_full_info_ton_wallet_and_share_theme() {
    let json = json!({
        "id": 7,
        "type": "private",
        "max_reaction_count": 1,
        "accent_color_id": 0,
        "accepted_gift_types": {
            "unlimited_gifts": true,
            "limited_gifts": true,
            "unique_gifts": true,
            "premium_subscription": true,
            "gifts_from_channels": true
        },
        "ton_wallet_card": {
            "address": "EQcard",
            "amount": "1000000",
            "stickers": [
                {
                    "position": {
                        "x_percentage": 50.0,
                        "y_percentage": 50.0,
                        "width_percentage": 20.0,
                        "height_percentage": 20.0,
                        "rotation_angle": 0.0,
                        "corner_radius_percentage": 10.0
                    },
                    "nft": {
                        "address": "EQnft",
                        "index": "1",
                        "name": "Star",
                        "description": "A star",
                        "attributes": [{"trait_type": "rarity", "value": "rare"}]
                    }
                }
            ],
            "background_nft_address": "EQbg"
        },
        "share_theme": {
            "type": "emoji",
            "emojis": [
                {"type": "emoji", "emoji": "A"},
                {"type": "custom_emoji", "custom_emoji_id": "custom_emoji_42"}
            ],
            "background_backdrop_id": 3
        }
    });
    let info: ChatFullInfo = roundtrip_check(json);
    let card = info.ton_wallet_card.as_ref().expect("wallet card");
    assert_eq!(card.address, "EQcard");
    assert_eq!(card.amount, "1000000");
    assert_eq!(card.background_nft_address.as_deref(), Some("EQbg"));
    assert_eq!(card.stickers.len(), 1);
    let nft = &card.stickers[0].nft;
    assert_eq!(nft.address, "EQnft");
    assert_eq!(nft.name, "Star");
    assert_eq!(nft.attributes.as_ref().unwrap()[0].trait_type, "rarity");
    match info.share_theme.as_ref().expect("share theme") {
        rust_tg_bot_raw::types::chat_share_theme::ChatShareTheme::Emoji(theme) => {
            assert_eq!(theme.emojis.len(), 2);
            assert_eq!(theme.background_backdrop_id, Some(3));
            assert!(matches!(
                theme.emojis[1],
                rust_tg_bot_raw::types::chat_share_theme::EmojiType::CustomEmoji(_)
            ));
        }
        other => panic!("expected emoji share theme, got {other:?}"),
    }
}

#[test]
fn chat_share_theme_unique_gift_and_backdrops() {
    use rust_tg_bot_raw::types::chat_share_theme::{
        ChatShareTheme, ChatShareThemeBackdrops, EmojiType,
    };

    // Backdrop list returned by getChatShareThemeBackdrops.
    let backdrops: ChatShareThemeBackdrops = roundtrip_check(json!({
        "backdrops": [{
            "id": 12,
            "name": "Aurora",
            "colors": {"center_color": 1, "edge_color": 2, "symbol_color": 3, "text_color": 4}
        }]
    }));
    assert_eq!(backdrops.backdrops[0].id, 12);
    assert_eq!(backdrops.backdrops[0].colors.text_color, 4);

    // Emoji helper constructors round-trip through the tagged wire format.
    let theme = ChatShareTheme::emoji(vec![
        EmojiType::emoji("A"),
        EmojiType::custom_emoji("custom_emoji_42"),
    ]);
    let value = serde_json::to_value(&theme).unwrap();
    assert_eq!(value["type"], "emoji");
    assert_eq!(value["emojis"][0], json!({"type": "emoji", "emoji": "A"}));
}

// ===========================================================================
// Bot API 10.4 -- rich text, quotes and ephemeral parameters
// ===========================================================================

#[test]
fn rich_text_ton_address_variant() {
    use rust_tg_bot_raw::types::rich_text::{RichText, RichTextFormatted, RichTextTonAddress};

    let text: RichText =
        roundtrip_check(json!({"type": "ton_address", "text": "EQabc", "address": "EQabc"}));
    match &text {
        RichText::Formatted(inner) => match inner.as_ref() {
            RichTextFormatted::TonAddress(addr) => {
                assert_eq!(addr.address, "EQabc");
                assert!(matches!(addr.text.as_ref(), RichText::Text(t) if t == "EQabc"));
            }
            other => panic!("expected ton_address, got {other:?}"),
        },
        other => panic!("expected formatted rich text, got {other:?}"),
    }

    let built = RichText::Formatted(Box::new(RichTextFormatted::TonAddress(
        RichTextTonAddress::new("EQabc", "EQabc"),
    )));
    let value = serde_json::to_value(&built).unwrap();
    assert_eq!(value["type"], "ton_address");
}

#[test]
fn text_quote_and_reply_parameters_rich_texts() {
    use rust_tg_bot_raw::types::reply::{ReplyParameters, TextQuote};

    let quote: TextQuote = roundtrip_check(json!({
        "text": "quoted",
        "position": 3,
        "rich_texts": ["plain", {"type": "bold", "text": "bold"}]
    }));
    let rich = quote.rich_texts.as_ref().expect("rich_texts");
    assert_eq!(rich.len(), 2);
    assert!(matches!(
        rich[1],
        rust_tg_bot_raw::types::rich_text::RichText::Formatted(_)
    ));

    let params: ReplyParameters = roundtrip_check(json!({
        "message_id": 9,
        "quote_rich_texts": ["first", "second"]
    }));
    assert_eq!(params.quote_rich_texts.as_ref().unwrap().len(), 2);
}

#[test]
fn ephemeral_message_parameters_callback_query_only() {
    use rust_tg_bot_raw::types::ephemeral::EphemeralMessageParameters;

    // 10.4: receiver_user_id is optional, so an ephemeral reply can target just the
    // callback query (and then the method's chat_id is ignored by Telegram).
    let params = EphemeralMessageParameters::reply_to_callback_query("cbq-1");
    assert!(params.receiver_user_id.is_none());
    assert_eq!(
        serde_json::to_value(&params).unwrap(),
        json!({"callback_query_id": "cbq-1"})
    );

    // The legacy constructor still stamps the receiver.
    let legacy = EphemeralMessageParameters::new(7);
    assert_eq!(
        serde_json::to_value(&legacy).unwrap(),
        json!({"receiver_user_id": 7})
    );

    // Both set: the reply is addressed to one user's button press.
    let both = EphemeralMessageParameters::new(7).callback_query_id("cbq-2");
    assert_eq!(
        serde_json::to_value(&both).unwrap(),
        json!({"receiver_user_id": 7, "callback_query_id": "cbq-2"})
    );
}

#[test]
fn story_area_position_alias_and_area_position() {
    use rust_tg_bot_raw::types::area_position::AreaPosition;

    let area: AreaPosition = roundtrip_check(json!({
        "x_percentage": 1.0,
        "y_percentage": 2.0,
        "width_percentage": 3.0,
        "height_percentage": 4.0,
        "rotation_angle": 5.0,
        "corner_radius_percentage": 6.0
    }));
    assert_eq!(area.rotation_angle, 5.0);

    // The 10.3 name still resolves, but must not be used by new code.
    #[allow(deprecated)]
    let legacy: rust_tg_bot_raw::types::story_area::StoryAreaPosition = area.clone();
    assert_eq!(legacy.corner_radius_percentage, 6.0);
}

#[test]
fn roundtrip_chat_owner_service_messages() {
    use rust_tg_bot_raw::types::chat_owner::{ChatOwnerChanged, ChatOwnerLeft};

    let changed: ChatOwnerChanged = roundtrip_check(json!({
        "new_owner": {"id": 9, "is_bot": false, "first_name": "Owner"}
    }));
    assert_eq!(changed.new_owner.id, 9);

    // The new owner is optional here: the previous owner may return.
    let left: ChatOwnerLeft = roundtrip_check(json!({
        "new_owner": {"id": 10, "is_bot": false, "first_name": "Next"}
    }));
    assert_eq!(left.new_owner.as_ref().unwrap().id, 10);

    let bare: ChatOwnerLeft = roundtrip_check(json!({}));
    assert!(bare.new_owner.is_none());
}

#[test]
fn input_widget_message_content_in_inline_results() {
    use rust_tg_bot_raw::types::inline::input_message_content::InputMessageContent;
    use rust_tg_bot_raw::types::inline::input_shared_widget_message_content::InputSharedWidgetMessageContent;
    use rust_tg_bot_raw::types::inline::input_widget_message_content::InputWidgetMessageContent;

    let widget = InputWidgetMessageContent::new("<div id=\"app\"></div>")
        .width(320)
        .height(180)
        .is_shared(true)
        .allow_queries(true)
        .caption("Hi");
    let value = serde_json::to_value(InputMessageContent::Widget(widget)).unwrap();
    assert_eq!(value["html"], "<div id=\"app\"></div>");
    assert_eq!(value["width"], json!(320));
    assert_eq!(value["is_shared"], json!(true));
    assert_eq!(value["allow_queries"], json!(true));
    assert_eq!(value["caption"], json!("Hi"));
    assert!(value.get("chat_data").is_none());

    // Untagged deserialization picks the variant by its distinguishing field.
    let back: InputMessageContent = serde_json::from_value(value).unwrap();
    assert!(matches!(back, InputMessageContent::Widget(_)));

    let shared = InputSharedWidgetMessageContent::new("widget-1").caption("again");
    let value = serde_json::to_value(InputMessageContent::SharedWidget(shared)).unwrap();
    assert_eq!(value["widget_id"], json!("widget-1"));
    assert_eq!(value["caption"], json!("again"));
    let back: InputMessageContent = serde_json::from_value(value).unwrap();
    assert!(matches!(back, InputMessageContent::SharedWidget(_)));
}
