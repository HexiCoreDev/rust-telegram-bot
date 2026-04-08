use super::*;

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Stickers
    // ======================================================================

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

    pub async fn get_sticker_set(&self, name: &str) -> Result<files::sticker::StickerSet> {
        let params = vec![RequestParameter::new(
            "name",
            serde_json::Value::String(name.to_owned()),
        )];
        self.do_post("getStickerSet", params).await
    }

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

    pub async fn set_sticker_position_in_set(&self, sticker: &str, position: i64) -> Result<bool> {
        let params = vec![
            RequestParameter::new("sticker", serde_json::Value::String(sticker.to_owned())),
            RequestParameter::new("position", serde_json::to_value(position)?),
        ];
        self.do_post("setStickerPositionInSet", params).await
    }

    pub async fn delete_sticker_from_set(&self, sticker: &str) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "sticker",
            serde_json::Value::String(sticker.to_owned()),
        )];
        self.do_post("deleteStickerFromSet", params).await
    }

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

    pub async fn set_sticker_set_title(&self, name: &str, title: &str) -> Result<bool> {
        let params = vec![
            RequestParameter::new("name", serde_json::Value::String(name.to_owned())),
            RequestParameter::new("title", serde_json::Value::String(title.to_owned())),
        ];
        self.do_post("setStickerSetTitle", params).await
    }

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

    pub async fn delete_sticker_set(&self, name: &str) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "name",
            serde_json::Value::String(name.to_owned()),
        )];
        self.do_post("deleteStickerSet", params).await
    }

    pub async fn get_forum_topic_icon_stickers(&self) -> Result<Vec<files::sticker::Sticker>> {
        self.do_post("getForumTopicIconStickers", Vec::new()).await
    }
}
