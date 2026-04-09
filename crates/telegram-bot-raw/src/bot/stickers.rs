use super::{input_file_param, push_opt, push_opt_file, push_opt_str, Bot, ChatId, Result};
use crate::request::request_parameter::RequestParameter;
use crate::types::{files, message, reply, suggested_post};

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Stickers
    // ======================================================================

    /// Sends a sticker. Internal raw method used by builder APIs.
    ///
    /// Calls the Telegram `sendSticker` API method.
    pub(crate) async fn send_sticker_raw(
        &self,
        chat_id: ChatId,
        sticker: files::input_file::InputFile,
        emoji: Option<&str>,
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        message_thread_id: Option<i64>,
        business_connection_id: Option<&str>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            input_file_param("sticker", sticker),
        ];
        push_opt_str(&mut params, "emoji", emoji);
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            business_connection_id,
        );
        push_opt_str(&mut params, "message_effect_id", message_effect_id);
        push_opt(&mut params, "allow_paid_broadcast", &allow_paid_broadcast)?;
        push_opt(
            &mut params,
            "direct_messages_topic_id",
            &direct_messages_topic_id,
        )?;
        push_opt(
            &mut params,
            "suggested_post_parameters",
            &suggested_post_parameters,
        )?;
        self.do_post("sendSticker", params).await
    }

    /// Use this method to get a sticker set by name.
    ///
    /// Calls the Telegram `getStickerSet` API method.
    pub async fn get_sticker_set(&self, name: &str) -> Result<files::sticker::StickerSet> {
        let params = vec![RequestParameter::new(
            "name",
            serde_json::Value::String(name.to_owned()),
        )];
        self.do_post("getStickerSet", params).await
    }

    /// Use this method to get information about custom emoji stickers by their identifiers.
    ///
    /// Calls the Telegram `getCustomEmojiStickers` API method.
    pub async fn get_custom_emoji_stickers(
        &self,
        custom_emoji_ids: Vec<String>,
    ) -> Result<Vec<files::sticker::Sticker>> {
        let params = vec![RequestParameter::new(
            "custom_emoji_ids",
            serde_json::to_value(&custom_emoji_ids)?,
        )];
        self.do_post("getCustomEmojiStickers", params).await
    }

    /// Use this method to upload a sticker file for later use in sticker sets.
    ///
    /// Calls the Telegram `uploadStickerFile` API method.
    pub async fn upload_sticker_file(
        &self,
        user_id: i64,
        sticker: files::input_file::InputFile,
        sticker_format: &str,
    ) -> Result<files::file::File> {
        let params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            input_file_param("sticker", sticker),
            RequestParameter::new(
                "sticker_format",
                serde_json::Value::String(sticker_format.to_owned()),
            ),
        ];
        self.do_post("uploadStickerFile", params).await
    }

    /// Use this method to create a new sticker set owned by a user.
    ///
    /// Calls the Telegram `createNewStickerSet` API method.
    pub async fn create_new_sticker_set(
        &self,
        user_id: i64,
        name: &str,
        title: &str,
        stickers: Vec<serde_json::Value>,
        sticker_type: Option<&str>,
        needs_repainting: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("name", serde_json::Value::String(name.to_owned())),
            RequestParameter::new("title", serde_json::Value::String(title.to_owned())),
            RequestParameter::new("stickers", serde_json::to_value(&stickers)?),
        ];
        push_opt_str(&mut params, "sticker_type", sticker_type);
        push_opt(&mut params, "needs_repainting", &needs_repainting)?;
        self.do_post("createNewStickerSet", params).await
    }

    /// Use this method to add a new sticker to an existing sticker set.
    ///
    /// Calls the Telegram `addStickerToSet` API method.
    pub async fn add_sticker_to_set(
        &self,
        user_id: i64,
        name: &str,
        sticker: serde_json::Value,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("name", serde_json::Value::String(name.to_owned())),
            RequestParameter::new("sticker", sticker),
        ];
        self.do_post("addStickerToSet", params).await
    }

    /// Use this method to move a sticker in a set to a specific position.
    ///
    /// Calls the Telegram `setStickerPositionInSet` API method.
    pub async fn set_sticker_position_in_set(&self, sticker: &str, position: i64) -> Result<bool> {
        let params = vec![
            RequestParameter::new("sticker", serde_json::Value::String(sticker.to_owned())),
            RequestParameter::new("position", serde_json::to_value(position)?),
        ];
        self.do_post("setStickerPositionInSet", params).await
    }

    /// Use this method to delete a sticker from a set.
    ///
    /// Calls the Telegram `deleteStickerFromSet` API method.
    pub async fn delete_sticker_from_set(&self, sticker: &str) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "sticker",
            serde_json::Value::String(sticker.to_owned()),
        )];
        self.do_post("deleteStickerFromSet", params).await
    }

    /// Use this method to replace an existing sticker in a sticker set with a new one.
    ///
    /// Calls the Telegram `replaceStickerInSet` API method.
    pub async fn replace_sticker_in_set(
        &self,
        user_id: i64,
        name: &str,
        old_sticker: &str,
        sticker: serde_json::Value,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("name", serde_json::Value::String(name.to_owned())),
            RequestParameter::new(
                "old_sticker",
                serde_json::Value::String(old_sticker.to_owned()),
            ),
            RequestParameter::new("sticker", sticker),
        ];
        self.do_post("replaceStickerInSet", params).await
    }

    /// Use this method to change the list of emoji assigned to a regular or custom emoji sticker.
    ///
    /// Calls the Telegram `setStickerEmojiList` API method.
    pub async fn set_sticker_emoji_list(
        &self,
        sticker: &str,
        emoji_list: Vec<String>,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("sticker", serde_json::Value::String(sticker.to_owned())),
            RequestParameter::new("emoji_list", serde_json::to_value(&emoji_list)?),
        ];
        self.do_post("setStickerEmojiList", params).await
    }

    /// Use this method to change search keywords assigned to a regular or custom emoji sticker.
    ///
    /// Calls the Telegram `setStickerKeywords` API method.
    pub async fn set_sticker_keywords(
        &self,
        sticker: &str,
        keywords: Option<Vec<String>>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "sticker",
            serde_json::Value::String(sticker.to_owned()),
        )];
        push_opt(&mut params, "keywords", &keywords)?;
        self.do_post("setStickerKeywords", params).await
    }

    /// Use this method to change the mask position of a mask sticker.
    ///
    /// Calls the Telegram `setStickerMaskPosition` API method.
    pub async fn set_sticker_mask_position(
        &self,
        sticker: &str,
        mask_position: Option<files::sticker::MaskPosition>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "sticker",
            serde_json::Value::String(sticker.to_owned()),
        )];
        push_opt(&mut params, "mask_position", &mask_position)?;
        self.do_post("setStickerMaskPosition", params).await
    }

    /// Use this method to set the thumbnail of a regular or mask sticker set.
    ///
    /// Calls the Telegram `setStickerSetThumbnail` API method.
    pub async fn set_sticker_set_thumbnail(
        &self,
        name: &str,
        user_id: i64,
        format: &str,
        thumbnail: Option<files::input_file::InputFile>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("name", serde_json::Value::String(name.to_owned())),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("format", serde_json::Value::String(format.to_owned())),
        ];
        push_opt_file(&mut params, "thumbnail", thumbnail);
        self.do_post("setStickerSetThumbnail", params).await
    }

    /// Use this method to set the title of a created sticker set.
    ///
    /// Calls the Telegram `setStickerSetTitle` API method.
    pub async fn set_sticker_set_title(&self, name: &str, title: &str) -> Result<bool> {
        let params = vec![
            RequestParameter::new("name", serde_json::Value::String(name.to_owned())),
            RequestParameter::new("title", serde_json::Value::String(title.to_owned())),
        ];
        self.do_post("setStickerSetTitle", params).await
    }

    /// Use this method to set the thumbnail of a custom emoji sticker set.
    ///
    /// Calls the Telegram `setCustomEmojiStickerSetThumbnail` API method.
    pub async fn set_custom_emoji_sticker_set_thumbnail(
        &self,
        name: &str,
        custom_emoji_id: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "name",
            serde_json::Value::String(name.to_owned()),
        )];
        push_opt_str(&mut params, "custom_emoji_id", custom_emoji_id);
        self.do_post("setCustomEmojiStickerSetThumbnail", params)
            .await
    }

    /// Use this method to delete a sticker set that was created by the bot.
    ///
    /// Calls the Telegram `deleteStickerSet` API method.
    pub async fn delete_sticker_set(&self, name: &str) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "name",
            serde_json::Value::String(name.to_owned()),
        )];
        self.do_post("deleteStickerSet", params).await
    }

    /// Use this method to get custom emoji stickers which can be used as a forum topic icon.
    ///
    /// Calls the Telegram `getForumTopicIconStickers` API method.
    pub async fn get_forum_topic_icon_stickers(&self) -> Result<Vec<files::sticker::Sticker>> {
        self.do_post("getForumTopicIconStickers", Vec::new()).await
    }
}
