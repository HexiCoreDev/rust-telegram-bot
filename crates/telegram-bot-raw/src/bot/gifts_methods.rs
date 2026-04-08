use super::*;

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Gifts
    // ======================================================================

    pub async fn get_available_gifts(&self) -> Result<gifts::Gifts> {
        self.do_post("getAvailableGifts", Vec::new()).await
    }

    pub async fn send_gift(
        &self,
        gift_id: &str,
        user_id: Option<i64>,
        chat_id: Option<ChatId>,
        text: Option<&str>,
        text_parse_mode: Option<&str>,
        text_entities: Option<Vec<message_entity::MessageEntity>>,
        pay_for_upgrade: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "gift_id",
            serde_json::Value::String(gift_id.to_owned()),
        )];
        push_opt(&mut params, "user_id", &user_id)?;
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt_str(&mut params, "text", text);
        push_opt_str(&mut params, "text_parse_mode", text_parse_mode);
        push_opt(&mut params, "text_entities", &text_entities)?;
        push_opt(&mut params, "pay_for_upgrade", &pay_for_upgrade)?;
        self.do_post("sendGift", params).await
    }

    pub async fn gift_premium_subscription(
        &self,
        user_id: i64,
        month_count: i64,
        star_count: i64,
        text: Option<&str>,
        text_parse_mode: Option<&str>,
        text_entities: Option<Vec<message_entity::MessageEntity>>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("month_count", serde_json::to_value(month_count)?),
            RequestParameter::new("star_count", serde_json::to_value(star_count)?),
        ];
        push_opt_str(&mut params, "text", text);
        push_opt_str(&mut params, "text_parse_mode", text_parse_mode);
        push_opt(&mut params, "text_entities", &text_entities)?;
        self.do_post("giftPremiumSubscription", params).await
    }

    pub async fn get_user_gifts(
        &self,
        user_id: i64,
        exclude_unlimited: Option<bool>,
        exclude_limited_upgradable: Option<bool>,
        exclude_limited_non_upgradable: Option<bool>,
        exclude_from_blockchain: Option<bool>,
        exclude_unique: Option<bool>,
        sort_by_price: Option<bool>,
        offset: Option<&str>,
        limit: Option<i64>,
    ) -> Result<owned_gift::OwnedGifts> {
        let mut params = vec![RequestParameter::new(
            "user_id",
            serde_json::to_value(user_id)?,
        )];
        push_opt(&mut params, "exclude_unlimited", &exclude_unlimited)?;
        push_opt(
            &mut params,
            "exclude_limited_upgradable",
            &exclude_limited_upgradable,
        )?;
        push_opt(
            &mut params,
            "exclude_limited_non_upgradable",
            &exclude_limited_non_upgradable,
        )?;
        push_opt(
            &mut params,
            "exclude_from_blockchain",
            &exclude_from_blockchain,
        )?;
        push_opt(&mut params, "exclude_unique", &exclude_unique)?;
        push_opt(&mut params, "sort_by_price", &sort_by_price)?;
        push_opt_str(&mut params, "offset", offset);
        push_opt(&mut params, "limit", &limit)?;
        self.do_post("getUserGifts", params).await
    }

    pub async fn get_chat_gifts(
        &self,
        chat_id: ChatId,
        exclude_unsaved: Option<bool>,
        exclude_saved: Option<bool>,
        exclude_unlimited: Option<bool>,
        exclude_limited_upgradable: Option<bool>,
        exclude_limited_non_upgradable: Option<bool>,
        exclude_from_blockchain: Option<bool>,
        exclude_unique: Option<bool>,
        sort_by_price: Option<bool>,
        offset: Option<&str>,
        limit: Option<i64>,
    ) -> Result<owned_gift::OwnedGifts> {
        let mut params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        push_opt(&mut params, "exclude_unsaved", &exclude_unsaved)?;
        push_opt(&mut params, "exclude_saved", &exclude_saved)?;
        push_opt(&mut params, "exclude_unlimited", &exclude_unlimited)?;
        push_opt(
            &mut params,
            "exclude_limited_upgradable",
            &exclude_limited_upgradable,
        )?;
        push_opt(
            &mut params,
            "exclude_limited_non_upgradable",
            &exclude_limited_non_upgradable,
        )?;
        push_opt(
            &mut params,
            "exclude_from_blockchain",
            &exclude_from_blockchain,
        )?;
        push_opt(&mut params, "exclude_unique", &exclude_unique)?;
        push_opt(&mut params, "sort_by_price", &sort_by_price)?;
        push_opt_str(&mut params, "offset", offset);
        push_opt(&mut params, "limit", &limit)?;
        self.do_post("getChatGifts", params).await
    }
}
