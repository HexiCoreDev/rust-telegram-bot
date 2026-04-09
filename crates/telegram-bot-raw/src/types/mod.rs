/// Base trait for all Telegram objects.
pub mod telegram_object;

// Core types
/// Telegram callback query type.
pub mod callback_query;
/// Telegram chat type.
pub mod chat;
/// Full chat information type.
pub mod chat_full_info;
/// Chosen inline result type.
pub mod chosen_inline_result;
/// Telegram message type.
pub mod message;
/// Telegram update type.
pub mod update;
/// Telegram user type.
pub mod user;

// Chat-related types
/// Chat administrator rights type.
pub mod chat_administrator_rights;
/// Chat background types.
pub mod chat_background;
/// Chat boost types.
pub mod chat_boost;
/// Chat invite link type.
pub mod chat_invite_link;
/// Chat join request type.
pub mod chat_join_request;
/// Chat location type.
pub mod chat_location;
/// Chat member status types.
pub mod chat_member;
/// Chat member updated event type.
pub mod chat_member_updated;
/// Chat owner type.
pub mod chat_owner;
/// Chat permissions type.
pub mod chat_permissions;

// Message-related types
/// Copy text button type.
pub mod copy_text_button;
/// Force reply markup type.
pub mod force_reply;
/// Link preview options type.
pub mod link_preview_options;
/// Message auto-delete timer changed service type.
pub mod message_auto_delete_timer_changed;
/// Message entity type.
pub mod message_entity;
/// Message identifier type.
pub mod message_id;
/// Message origin types.
pub mod message_origin;
/// Message reaction updated event type.
pub mod message_reaction_updated;
/// Reply parameters type.
pub mod reply;
/// Reply keyboard markup type.
pub mod reply_keyboard_markup;
/// Reply keyboard remove type.
pub mod reply_keyboard_remove;

// Response types
/// Response parameters type.
pub mod response_parameters;

// Bot types
/// Bot command type.
pub mod bot_command;
/// Bot command scope type.
pub mod bot_command_scope;
/// Bot description types.
pub mod bot_description;
/// Bot name type.
pub mod bot_name;
/// Managed bot type.
pub mod managed_bot;

// Keyboard types
/// Keyboard button type.
pub mod keyboard_button;
/// Keyboard button poll type.
pub mod keyboard_button_poll_type;
/// Keyboard button request types.
pub mod keyboard_button_request;
/// Prepared keyboard button type.
pub mod prepared_keyboard_button;
/// Switch inline query chosen chat type.
pub mod switch_inline_query_chosen_chat;

// Media & content types
/// Dice type.
pub mod dice;
/// Poll and poll answer types.
pub mod poll;
/// Reaction types.
pub mod reaction;
/// Story type.
pub mod story;
/// Story area types.
pub mod story_area;
/// Video chat types.
pub mod video_chat;
/// Web app data type.
pub mod web_app_data;
/// Web app info type.
pub mod web_app_info;

// Business types
/// Business connection and related types.
pub mod business;
/// Direct message price changed service type.
pub mod direct_message_price_changed;
/// Direct messages topic type.
pub mod direct_messages_topic;
/// Suggested post types.
pub mod suggested_post;

// Gift & giveaway types
/// Gift and accepted gift types.
pub mod gifts;
/// Giveaway types.
pub mod giveaway;
/// Owned gift types.
pub mod owned_gift;
/// Unique gift types.
pub mod unique_gift;

// Paid content types
/// Paid media types.
pub mod paid_media;
/// Paid message price changed service type.
pub mod paid_message_price_changed;

// User types
/// Birthdate type.
pub mod birthdate;
/// User profile audios type.
pub mod user_profile_audios;
/// User profile photos type.
pub mod user_profile_photos;
/// User rating type.
pub mod user_rating;

// Misc types
/// Checklist types.
pub mod checklists;
/// Forum topic types.
pub mod forum_topic;
/// Input checklist type.
pub mod input_checklist;
/// Login URL type.
pub mod login_url;
/// Menu button types.
pub mod menu_button;
/// Proximity alert triggered service type.
pub mod proximity_alert_triggered;
/// Sent web app message type.
pub mod sent_web_app_message;
/// Shared user and chat types.
pub mod shared;
/// Webhook info type.
pub mod webhook_info;
/// Write access allowed service type.
pub mod write_access_allowed;

// Submodules
/// File and media input types.
pub mod files;
/// Game types.
pub mod games;
/// Inline query types.
pub mod inline;
/// Telegram Passport types.
pub mod passport;
/// Payment and invoice types.
pub mod payment;
