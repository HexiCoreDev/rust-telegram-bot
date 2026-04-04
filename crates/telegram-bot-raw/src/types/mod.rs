// Base trait for all Telegram objects
pub mod telegram_object;

// Core types
pub mod callback_query;
pub mod chat;
pub mod chat_full_info;
pub mod chosen_inline_result;
pub mod message;
pub mod update;
pub mod user;

// Chat-related types
pub mod chat_administrator_rights;
pub mod chat_background;
pub mod chat_boost;
pub mod chat_invite_link;
pub mod chat_join_request;
pub mod chat_location;
pub mod chat_member;
pub mod chat_member_updated;
pub mod chat_owner;
pub mod chat_permissions;

// Message-related types
pub mod copy_text_button;
pub mod force_reply;
pub mod link_preview_options;
pub mod message_auto_delete_timer_changed;
pub mod message_entity;
pub mod message_id;
pub mod message_origin;
pub mod message_reaction_updated;
pub mod reply;
pub mod reply_keyboard_markup;
pub mod reply_keyboard_remove;

// Response types
pub mod response_parameters;

// Bot types
pub mod bot_command;
pub mod bot_command_scope;
pub mod bot_description;
pub mod bot_name;
pub mod managed_bot;

// Keyboard types
pub mod keyboard_button;
pub mod keyboard_button_poll_type;
pub mod keyboard_button_request;
pub mod prepared_keyboard_button;
pub mod switch_inline_query_chosen_chat;

// Media & content types
pub mod dice;
pub mod poll;
pub mod reaction;
pub mod story;
pub mod story_area;
pub mod video_chat;
pub mod web_app_data;
pub mod web_app_info;

// Business types
pub mod business;
pub mod direct_message_price_changed;
pub mod direct_messages_topic;
pub mod suggested_post;

// Gift & giveaway types
pub mod gifts;
pub mod giveaway;
pub mod owned_gift;
pub mod unique_gift;

// Paid content types
pub mod paid_media;
pub mod paid_message_price_changed;

// User types
pub mod birthdate;
pub mod user_profile_audios;
pub mod user_profile_photos;
pub mod user_rating;

// Misc types
pub mod checklists;
pub mod forum_topic;
pub mod input_checklist;
pub mod login_url;
pub mod menu_button;
pub mod proximity_alert_triggered;
pub mod sent_web_app_message;
pub mod shared;
pub mod webhook_info;
pub mod write_access_allowed;

// Submodules
pub mod files;
pub mod games;
pub mod inline;
pub mod passport;
pub mod payment;
