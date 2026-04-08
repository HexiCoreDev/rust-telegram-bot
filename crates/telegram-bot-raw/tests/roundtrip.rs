//! Round-trip serialization tests against realistic Telegram Bot API JSON payloads.
//!
//! Each test: deserialize from JSON string -> serialize back -> compare that all
//! important fields survive the round-trip. We do not require byte-exact JSON
//! (field ordering may differ), but verify all critical fields match.

use serde_json::json;
use telegram_bot_raw::types::callback_query::CallbackQuery;
use telegram_bot_raw::types::chat::Chat;
use telegram_bot_raw::types::chat_full_info::ChatFullInfo;
use telegram_bot_raw::types::inline::inline_query::InlineQuery;
use telegram_bot_raw::types::message::Message;
use telegram_bot_raw::types::update::Update;
use telegram_bot_raw::types::user::User;

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
            telegram_bot_raw::types::message_origin::MessageOrigin::User(data) => {
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
        assert_eq!(msg.is_topic_message, Some(true));
        assert_eq!(msg.edit_date, Some(1700005100));
        assert_eq!(msg.has_protected_content, Some(true));
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
        let msg = update.message.as_ref().unwrap();
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
        let cbq = update.callback_query.as_ref().unwrap();
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
        let iq = update.inline_query.as_ref().unwrap();
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
        assert!(update.message.is_none());
        let msg = update.edited_message.as_ref().unwrap();
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
        let post = update.channel_post.as_ref().unwrap();
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
        let cbq = update.callback_query.as_ref().unwrap();
        let msg = cbq.message.as_ref().unwrap();
        assert_eq!(msg.message_id(), 80);
    });
}

// ===========================================================================
// Update -- extra/unknown fields survive via serde(flatten)
// ===========================================================================

#[test]
fn roundtrip_update_unknown_fields_preserved() {
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
        let update: Update = roundtrip_check(json);
        assert_eq!(update.update_id, 100000007);
        assert!(
            update.extra.contains_key("some_future_field"),
            "Unknown field 'some_future_field' should be captured in extra"
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
        let msg = update.message.as_ref().unwrap();
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
        let msg = update.message.as_ref().unwrap();
        assert_eq!(msg.message_id, 999);
        assert_eq!(msg.message_thread_id, Some(42));
        assert_eq!(msg.is_topic_message, Some(true));
        let from = msg.from_user.as_ref().unwrap();
        assert_eq!(from.is_premium, Some(true));
        assert!(msg.reply_markup.is_some());
    });
}
