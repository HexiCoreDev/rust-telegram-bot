use super::{push_opt, push_opt_str, Bot, Result};
use crate::request::request_parameter::RequestParameter;
use crate::types::{business, gifts, owned_gift, payment};

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Business account management
    // ======================================================================

    pub async fn get_business_connection(
        &self,
        business_connection_id: &str,
    ) -> Result<business::BusinessConnection> {
        let params = vec![RequestParameter::new(
            "business_connection_id",
            serde_json::Value::String(business_connection_id.to_owned()),
        )];
        self.do_post("getBusinessConnection", params).await
    }

    pub async fn get_business_account_gifts(
        &self,
        business_connection_id: &str,
        exclude_unsaved: Option<bool>,
        exclude_saved: Option<bool>,
        exclude_unlimited: Option<bool>,
        exclude_unique: Option<bool>,
        sort_by_price: Option<bool>,
        offset: Option<&str>,
        limit: Option<i64>,
        exclude_limited_upgradable: Option<bool>,
        exclude_limited_non_upgradable: Option<bool>,
        exclude_from_blockchain: Option<bool>,
    ) -> Result<owned_gift::OwnedGifts> {
        let mut params = vec![RequestParameter::new(
            "business_connection_id",
            serde_json::Value::String(business_connection_id.to_owned()),
        )];
        push_opt(&mut params, "exclude_unsaved", &exclude_unsaved)?;
        push_opt(&mut params, "exclude_saved", &exclude_saved)?;
        push_opt(&mut params, "exclude_unlimited", &exclude_unlimited)?;
        push_opt(&mut params, "exclude_unique", &exclude_unique)?;
        push_opt(&mut params, "sort_by_price", &sort_by_price)?;
        push_opt_str(&mut params, "offset", offset);
        push_opt(&mut params, "limit", &limit)?;
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
        self.do_post("getBusinessAccountGifts", params).await
    }

    pub async fn get_business_account_star_balance(
        &self,
        business_connection_id: &str,
    ) -> Result<payment::stars::star_amount::StarAmount> {
        let params = vec![RequestParameter::new(
            "business_connection_id",
            serde_json::Value::String(business_connection_id.to_owned()),
        )];
        self.do_post("getBusinessAccountStarBalance", params).await
    }

    pub async fn read_business_message(
        &self,
        business_connection_id: &str,
        chat_id: i64,
        message_id: i64,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("chat_id", serde_json::to_value(chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
        ];
        self.do_post("readBusinessMessage", params).await
    }

    pub async fn delete_business_messages(
        &self,
        business_connection_id: &str,
        message_ids: Vec<i64>,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("message_ids", serde_json::to_value(&message_ids)?),
        ];
        self.do_post("deleteBusinessMessages", params).await
    }

    pub async fn set_business_account_name(
        &self,
        business_connection_id: &str,
        first_name: &str,
        last_name: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new(
                "first_name",
                serde_json::Value::String(first_name.to_owned()),
            ),
        ];
        push_opt_str(&mut params, "last_name", last_name);
        self.do_post("setBusinessAccountName", params).await
    }

    pub async fn set_business_account_username(
        &self,
        business_connection_id: &str,
        username: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "business_connection_id",
            serde_json::Value::String(business_connection_id.to_owned()),
        )];
        push_opt_str(&mut params, "username", username);
        self.do_post("setBusinessAccountUsername", params).await
    }

    pub async fn set_business_account_bio(
        &self,
        business_connection_id: &str,
        bio: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "business_connection_id",
            serde_json::Value::String(business_connection_id.to_owned()),
        )];
        push_opt_str(&mut params, "bio", bio);
        self.do_post("setBusinessAccountBio", params).await
    }

    pub async fn set_business_account_gift_settings(
        &self,
        business_connection_id: &str,
        show_gift_button: bool,
        accepted_gift_types: gifts::AcceptedGiftTypes,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("show_gift_button", serde_json::to_value(show_gift_button)?),
            RequestParameter::new(
                "accepted_gift_types",
                serde_json::to_value(&accepted_gift_types)?,
            ),
        ];
        self.do_post("setBusinessAccountGiftSettings", params).await
    }

    pub async fn set_business_account_profile_photo(
        &self,
        business_connection_id: &str,
        photo: serde_json::Value,
        is_public: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("photo", photo),
        ];
        push_opt(&mut params, "is_public", &is_public)?;
        self.do_post("setBusinessAccountProfilePhoto", params).await
    }

    pub async fn remove_business_account_profile_photo(
        &self,
        business_connection_id: &str,
        is_public: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "business_connection_id",
            serde_json::Value::String(business_connection_id.to_owned()),
        )];
        push_opt(&mut params, "is_public", &is_public)?;
        self.do_post("removeBusinessAccountProfilePhoto", params)
            .await
    }

    pub async fn convert_gift_to_stars(
        &self,
        business_connection_id: &str,
        owned_gift_id: &str,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new(
                "owned_gift_id",
                serde_json::Value::String(owned_gift_id.to_owned()),
            ),
        ];
        self.do_post("convertGiftToStars", params).await
    }

    pub async fn upgrade_gift(
        &self,
        business_connection_id: &str,
        owned_gift_id: &str,
        keep_original_details: Option<bool>,
        star_count: Option<i64>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new(
                "owned_gift_id",
                serde_json::Value::String(owned_gift_id.to_owned()),
            ),
        ];
        push_opt(&mut params, "keep_original_details", &keep_original_details)?;
        push_opt(&mut params, "star_count", &star_count)?;
        self.do_post("upgradeGift", params).await
    }

    pub async fn transfer_gift(
        &self,
        business_connection_id: &str,
        owned_gift_id: &str,
        new_owner_chat_id: i64,
        star_count: Option<i64>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new(
                "owned_gift_id",
                serde_json::Value::String(owned_gift_id.to_owned()),
            ),
            RequestParameter::new(
                "new_owner_chat_id",
                serde_json::to_value(new_owner_chat_id)?,
            ),
        ];
        push_opt(&mut params, "star_count", &star_count)?;
        self.do_post("transferGift", params).await
    }

    pub async fn transfer_business_account_stars(
        &self,
        business_connection_id: &str,
        star_count: i64,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("star_count", serde_json::to_value(star_count)?),
        ];
        self.do_post("transferBusinessAccountStars", params).await
    }
}
