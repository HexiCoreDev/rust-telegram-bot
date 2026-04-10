//! Builder pattern for Business, Payments, and Gifts API methods.
//!
//! This module mirrors the pattern established in [`bot_builders`](crate::bot_builders),
//! providing ergonomic builder structs for the business account management,
//! payment, and gift-related Telegram Bot API endpoints.
//!
//! ```ignore
//! bot.get_business_account_gifts("conn-id")
//!     .exclude_unsaved(true)
//!     .limit(10)
//!     .await?;
//! ```

#![allow(clippy::too_many_arguments)]

use crate::bot::{Bot, ChatId};
use crate::error::Result;
use crate::request::request_parameter::RequestParameter;
use crate::types::{business, gifts, message_entity, owned_gift, payment};
use serde::Serialize;

// ---------------------------------------------------------------------------
// Private helpers (duplicated from bot_builders.rs since those are private)
// ---------------------------------------------------------------------------

fn push_opt<T: Serialize>(
    params: &mut Vec<RequestParameter>,
    name: &'static str,
    val: &Option<T>,
) -> std::result::Result<(), serde_json::Error> {
    if let Some(v) = val {
        params.push(RequestParameter::new(name, serde_json::to_value(v)?));
    }
    Ok(())
}

fn push_opt_str(params: &mut Vec<RequestParameter>, name: &'static str, val: &Option<String>) {
    if let Some(v) = val {
        params.push(RequestParameter::new(
            name,
            serde_json::Value::String(v.clone()),
        ));
    }
}

macro_rules! impl_into_future {
    ($builder:ident, $output:ty) => {
        impl<'a> std::future::IntoFuture for $builder<'a> {
            type Output = Result<$output>;
            type IntoFuture =
                std::pin::Pin<Box<dyn std::future::Future<Output = Self::Output> + Send + 'a>>;
            fn into_future(self) -> Self::IntoFuture {
                Box::pin(self.send())
            }
        }
    };
}

// =========================================================================
// BUSINESS METHODS
// =========================================================================

// =========================================================================
// GetBusinessConnectionBuilder
// =========================================================================

/// Builder for the [`getBusinessConnection`] API method.
pub struct GetBusinessConnectionBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
}

impl<'a> GetBusinessConnectionBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<business::BusinessConnection> {
        let params = vec![RequestParameter::new(
            "business_connection_id",
            serde_json::Value::String(self.business_connection_id),
        )];
        self.bot
            .do_api_request("getBusinessConnection", params)
            .await
    }
}

impl_into_future!(GetBusinessConnectionBuilder, business::BusinessConnection);

// =========================================================================
// GetBusinessAccountGiftsBuilder
// =========================================================================

/// Builder for the [`getBusinessAccountGifts`] API method.
pub struct GetBusinessAccountGiftsBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
    exclude_unsaved: Option<bool>,
    exclude_saved: Option<bool>,
    exclude_unlimited: Option<bool>,
    exclude_unique: Option<bool>,
    sort_by_price: Option<bool>,
    offset: Option<String>,
    limit: Option<i64>,
    exclude_limited_upgradable: Option<bool>,
    exclude_limited_non_upgradable: Option<bool>,
    exclude_from_blockchain: Option<bool>,
}

impl<'a> GetBusinessAccountGiftsBuilder<'a> {
    /// Sets the `exclude_unsaved` parameter.
    pub fn exclude_unsaved(mut self, val: bool) -> Self {
        self.exclude_unsaved = Some(val);
        self
    }
    /// Sets the `exclude_saved` parameter.
    pub fn exclude_saved(mut self, val: bool) -> Self {
        self.exclude_saved = Some(val);
        self
    }
    /// Sets the `exclude_unlimited` parameter.
    pub fn exclude_unlimited(mut self, val: bool) -> Self {
        self.exclude_unlimited = Some(val);
        self
    }
    /// Sets the `exclude_unique` parameter.
    pub fn exclude_unique(mut self, val: bool) -> Self {
        self.exclude_unique = Some(val);
        self
    }
    /// Sets the `sort_by_price` parameter.
    pub fn sort_by_price(mut self, val: bool) -> Self {
        self.sort_by_price = Some(val);
        self
    }
    /// Sets the `offset` parameter.
    pub fn offset(mut self, val: impl Into<String>) -> Self {
        self.offset = Some(val.into());
        self
    }
    /// Sets the `limit` parameter.
    pub fn limit(mut self, val: i64) -> Self {
        self.limit = Some(val);
        self
    }
    /// Sets the `exclude_limited_upgradable` parameter.
    pub fn exclude_limited_upgradable(mut self, val: bool) -> Self {
        self.exclude_limited_upgradable = Some(val);
        self
    }
    /// Sets the `exclude_limited_non_upgradable` parameter.
    pub fn exclude_limited_non_upgradable(mut self, val: bool) -> Self {
        self.exclude_limited_non_upgradable = Some(val);
        self
    }
    /// Sets the `exclude_from_blockchain` parameter.
    pub fn exclude_from_blockchain(mut self, val: bool) -> Self {
        self.exclude_from_blockchain = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<owned_gift::OwnedGifts> {
        let mut params = vec![RequestParameter::new(
            "business_connection_id",
            serde_json::Value::String(self.business_connection_id),
        )];
        push_opt(&mut params, "exclude_unsaved", &self.exclude_unsaved)?;
        push_opt(&mut params, "exclude_saved", &self.exclude_saved)?;
        push_opt(&mut params, "exclude_unlimited", &self.exclude_unlimited)?;
        push_opt(&mut params, "exclude_unique", &self.exclude_unique)?;
        push_opt(&mut params, "sort_by_price", &self.sort_by_price)?;
        push_opt_str(&mut params, "offset", &self.offset);
        push_opt(&mut params, "limit", &self.limit)?;
        push_opt(
            &mut params,
            "exclude_limited_upgradable",
            &self.exclude_limited_upgradable,
        )?;
        push_opt(
            &mut params,
            "exclude_limited_non_upgradable",
            &self.exclude_limited_non_upgradable,
        )?;
        push_opt(
            &mut params,
            "exclude_from_blockchain",
            &self.exclude_from_blockchain,
        )?;
        self.bot
            .do_api_request("getBusinessAccountGifts", params)
            .await
    }
}

impl_into_future!(GetBusinessAccountGiftsBuilder, owned_gift::OwnedGifts);

// =========================================================================
// GetBusinessAccountStarBalanceBuilder
// =========================================================================

/// Builder for the [`getBusinessAccountStarBalance`] API method.
pub struct GetBusinessAccountStarBalanceBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
}

impl<'a> GetBusinessAccountStarBalanceBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<payment::stars::star_amount::StarAmount> {
        let params = vec![RequestParameter::new(
            "business_connection_id",
            serde_json::Value::String(self.business_connection_id),
        )];
        self.bot
            .do_api_request("getBusinessAccountStarBalance", params)
            .await
    }
}

impl_into_future!(
    GetBusinessAccountStarBalanceBuilder,
    payment::stars::star_amount::StarAmount
);

// =========================================================================
// ReadBusinessMessageBuilder
// =========================================================================

/// Builder for the [`readBusinessMessage`] API method.
pub struct ReadBusinessMessageBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
    chat_id: i64,
    message_id: i64,
}

impl<'a> ReadBusinessMessageBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(self.business_connection_id),
            ),
            RequestParameter::new("chat_id", serde_json::to_value(self.chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(self.message_id)?),
        ];
        self.bot.do_api_request("readBusinessMessage", params).await
    }
}

impl_into_future!(ReadBusinessMessageBuilder, bool);

// =========================================================================
// DeleteBusinessMessagesBuilder
// =========================================================================

/// Builder for the [`deleteBusinessMessages`] API method.
pub struct DeleteBusinessMessagesBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
    message_ids: Vec<i64>,
}

impl<'a> DeleteBusinessMessagesBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(self.business_connection_id),
            ),
            RequestParameter::new("message_ids", serde_json::to_value(&self.message_ids)?),
        ];
        self.bot
            .do_api_request("deleteBusinessMessages", params)
            .await
    }
}

impl_into_future!(DeleteBusinessMessagesBuilder, bool);

// =========================================================================
// SetBusinessAccountNameBuilder
// =========================================================================

/// Builder for the [`setBusinessAccountName`] API method.
pub struct SetBusinessAccountNameBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
    first_name: String,
    last_name: Option<String>,
}

impl<'a> SetBusinessAccountNameBuilder<'a> {
    /// Sets the `last_name` parameter.
    pub fn last_name(mut self, val: impl Into<String>) -> Self {
        self.last_name = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(self.business_connection_id),
            ),
            RequestParameter::new("first_name", serde_json::Value::String(self.first_name)),
        ];
        push_opt_str(&mut params, "last_name", &self.last_name);
        self.bot
            .do_api_request("setBusinessAccountName", params)
            .await
    }
}

impl_into_future!(SetBusinessAccountNameBuilder, bool);

// =========================================================================
// SetBusinessAccountUsernameBuilder
// =========================================================================

/// Builder for the [`setBusinessAccountUsername`] API method.
pub struct SetBusinessAccountUsernameBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
    username: Option<String>,
}

impl<'a> SetBusinessAccountUsernameBuilder<'a> {
    /// Sets the `username` parameter.
    pub fn username(mut self, val: impl Into<String>) -> Self {
        self.username = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "business_connection_id",
            serde_json::Value::String(self.business_connection_id),
        )];
        push_opt_str(&mut params, "username", &self.username);
        self.bot
            .do_api_request("setBusinessAccountUsername", params)
            .await
    }
}

impl_into_future!(SetBusinessAccountUsernameBuilder, bool);

// =========================================================================
// SetBusinessAccountBioBuilder
// =========================================================================

/// Builder for the [`setBusinessAccountBio`] API method.
pub struct SetBusinessAccountBioBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
    bio: Option<String>,
}

impl<'a> SetBusinessAccountBioBuilder<'a> {
    /// Sets the `bio` parameter.
    pub fn bio(mut self, val: impl Into<String>) -> Self {
        self.bio = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "business_connection_id",
            serde_json::Value::String(self.business_connection_id),
        )];
        push_opt_str(&mut params, "bio", &self.bio);
        self.bot
            .do_api_request("setBusinessAccountBio", params)
            .await
    }
}

impl_into_future!(SetBusinessAccountBioBuilder, bool);

// =========================================================================
// SetBusinessAccountGiftSettingsBuilder
// =========================================================================

/// Builder for the [`setBusinessAccountGiftSettings`] API method.
pub struct SetBusinessAccountGiftSettingsBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
    show_gift_button: bool,
    accepted_gift_types: gifts::AcceptedGiftTypes,
}

impl<'a> SetBusinessAccountGiftSettingsBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(self.business_connection_id),
            ),
            RequestParameter::new(
                "show_gift_button",
                serde_json::to_value(self.show_gift_button)?,
            ),
            RequestParameter::new(
                "accepted_gift_types",
                serde_json::to_value(&self.accepted_gift_types)?,
            ),
        ];
        self.bot
            .do_api_request("setBusinessAccountGiftSettings", params)
            .await
    }
}

impl_into_future!(SetBusinessAccountGiftSettingsBuilder, bool);

// =========================================================================
// SetBusinessAccountProfilePhotoBuilder
// =========================================================================

/// Builder for the [`setBusinessAccountProfilePhoto`] API method.
pub struct SetBusinessAccountProfilePhotoBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
    photo: serde_json::Value,
    is_public: Option<bool>,
}

impl<'a> SetBusinessAccountProfilePhotoBuilder<'a> {
    /// Sets the `is_public` parameter.
    pub fn is_public(mut self, val: bool) -> Self {
        self.is_public = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(self.business_connection_id),
            ),
            RequestParameter::new("photo", self.photo),
        ];
        push_opt(&mut params, "is_public", &self.is_public)?;
        self.bot
            .do_api_request("setBusinessAccountProfilePhoto", params)
            .await
    }
}

impl_into_future!(SetBusinessAccountProfilePhotoBuilder, bool);

// =========================================================================
// RemoveBusinessAccountProfilePhotoBuilder
// =========================================================================

/// Builder for the [`removeBusinessAccountProfilePhoto`] API method.
pub struct RemoveBusinessAccountProfilePhotoBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
    is_public: Option<bool>,
}

impl<'a> RemoveBusinessAccountProfilePhotoBuilder<'a> {
    /// Sets the `is_public` parameter.
    pub fn is_public(mut self, val: bool) -> Self {
        self.is_public = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "business_connection_id",
            serde_json::Value::String(self.business_connection_id),
        )];
        push_opt(&mut params, "is_public", &self.is_public)?;
        self.bot
            .do_api_request("removeBusinessAccountProfilePhoto", params)
            .await
    }
}

impl_into_future!(RemoveBusinessAccountProfilePhotoBuilder, bool);

// =========================================================================
// ConvertGiftToStarsBuilder
// =========================================================================

/// Builder for the [`convertGiftToStars`] API method.
pub struct ConvertGiftToStarsBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
    owned_gift_id: String,
}

impl<'a> ConvertGiftToStarsBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(self.business_connection_id),
            ),
            RequestParameter::new(
                "owned_gift_id",
                serde_json::Value::String(self.owned_gift_id),
            ),
        ];
        self.bot.do_api_request("convertGiftToStars", params).await
    }
}

impl_into_future!(ConvertGiftToStarsBuilder, bool);

// =========================================================================
// UpgradeGiftBuilder
// =========================================================================

/// Builder for the [`upgradeGift`] API method.
pub struct UpgradeGiftBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
    owned_gift_id: String,
    keep_original_details: Option<bool>,
    star_count: Option<i64>,
}

impl<'a> UpgradeGiftBuilder<'a> {
    /// Sets the `keep_original_details` parameter.
    pub fn keep_original_details(mut self, val: bool) -> Self {
        self.keep_original_details = Some(val);
        self
    }
    /// Sets the `star_count` parameter.
    pub fn star_count(mut self, val: i64) -> Self {
        self.star_count = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(self.business_connection_id),
            ),
            RequestParameter::new(
                "owned_gift_id",
                serde_json::Value::String(self.owned_gift_id),
            ),
        ];
        push_opt(
            &mut params,
            "keep_original_details",
            &self.keep_original_details,
        )?;
        push_opt(&mut params, "star_count", &self.star_count)?;
        self.bot.do_api_request("upgradeGift", params).await
    }
}

impl_into_future!(UpgradeGiftBuilder, bool);

// =========================================================================
// TransferGiftBuilder
// =========================================================================

/// Builder for the [`transferGift`] API method.
pub struct TransferGiftBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
    owned_gift_id: String,
    new_owner_chat_id: i64,
    star_count: Option<i64>,
}

impl<'a> TransferGiftBuilder<'a> {
    /// Sets the `star_count` parameter.
    pub fn star_count(mut self, val: i64) -> Self {
        self.star_count = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(self.business_connection_id),
            ),
            RequestParameter::new(
                "owned_gift_id",
                serde_json::Value::String(self.owned_gift_id),
            ),
            RequestParameter::new(
                "new_owner_chat_id",
                serde_json::to_value(self.new_owner_chat_id)?,
            ),
        ];
        push_opt(&mut params, "star_count", &self.star_count)?;
        self.bot.do_api_request("transferGift", params).await
    }
}

impl_into_future!(TransferGiftBuilder, bool);

// =========================================================================
// TransferBusinessAccountStarsBuilder
// =========================================================================

/// Builder for the [`transferBusinessAccountStars`] API method.
pub struct TransferBusinessAccountStarsBuilder<'a> {
    bot: &'a Bot,
    business_connection_id: String,
    star_count: i64,
}

impl<'a> TransferBusinessAccountStarsBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(self.business_connection_id),
            ),
            RequestParameter::new("star_count", serde_json::to_value(self.star_count)?),
        ];
        self.bot
            .do_api_request("transferBusinessAccountStars", params)
            .await
    }
}

impl_into_future!(TransferBusinessAccountStarsBuilder, bool);

// =========================================================================
// PAYMENT METHODS
// =========================================================================

// =========================================================================
// CreateInvoiceLinkBuilder
// =========================================================================

/// Builder for the [`createInvoiceLink`] API method.
pub struct CreateInvoiceLinkBuilder<'a> {
    bot: &'a Bot,
    title: String,
    description: String,
    payload: String,
    currency: String,
    prices: Vec<serde_json::Value>,
    provider_token: Option<String>,
    max_tip_amount: Option<i64>,
    suggested_tip_amounts: Option<Vec<i64>>,
    provider_data: Option<String>,
    photo_url: Option<String>,
    photo_size: Option<i64>,
    photo_width: Option<i64>,
    photo_height: Option<i64>,
    need_name: Option<bool>,
    need_phone_number: Option<bool>,
    need_email: Option<bool>,
    need_shipping_address: Option<bool>,
    send_phone_number_to_provider: Option<bool>,
    send_email_to_provider: Option<bool>,
    is_flexible: Option<bool>,
    subscription_period: Option<i64>,
    business_connection_id: Option<String>,
}

impl<'a> CreateInvoiceLinkBuilder<'a> {
    /// Sets the `provider_token` parameter.
    pub fn provider_token(mut self, val: impl Into<String>) -> Self {
        self.provider_token = Some(val.into());
        self
    }
    /// Sets the `max_tip_amount` parameter.
    pub fn max_tip_amount(mut self, val: i64) -> Self {
        self.max_tip_amount = Some(val);
        self
    }
    /// Sets the `suggested_tip_amounts` parameter.
    pub fn suggested_tip_amounts(mut self, val: Vec<i64>) -> Self {
        self.suggested_tip_amounts = Some(val);
        self
    }
    /// Sets the `provider_data` parameter.
    pub fn provider_data(mut self, val: impl Into<String>) -> Self {
        self.provider_data = Some(val.into());
        self
    }
    /// Sets the `photo_url` parameter.
    pub fn photo_url(mut self, val: impl Into<String>) -> Self {
        self.photo_url = Some(val.into());
        self
    }
    /// Sets the `photo_size` parameter.
    pub fn photo_size(mut self, val: i64) -> Self {
        self.photo_size = Some(val);
        self
    }
    /// Sets the `photo_width` parameter.
    pub fn photo_width(mut self, val: i64) -> Self {
        self.photo_width = Some(val);
        self
    }
    /// Sets the `photo_height` parameter.
    pub fn photo_height(mut self, val: i64) -> Self {
        self.photo_height = Some(val);
        self
    }
    /// Sets the `need_name` parameter.
    pub fn need_name(mut self, val: bool) -> Self {
        self.need_name = Some(val);
        self
    }
    /// Sets the `need_phone_number` parameter.
    pub fn need_phone_number(mut self, val: bool) -> Self {
        self.need_phone_number = Some(val);
        self
    }
    /// Sets the `need_email` parameter.
    pub fn need_email(mut self, val: bool) -> Self {
        self.need_email = Some(val);
        self
    }
    /// Sets the `need_shipping_address` parameter.
    pub fn need_shipping_address(mut self, val: bool) -> Self {
        self.need_shipping_address = Some(val);
        self
    }
    /// Sets the `send_phone_number_to_provider` parameter.
    pub fn send_phone_number_to_provider(mut self, val: bool) -> Self {
        self.send_phone_number_to_provider = Some(val);
        self
    }
    /// Sets the `send_email_to_provider` parameter.
    pub fn send_email_to_provider(mut self, val: bool) -> Self {
        self.send_email_to_provider = Some(val);
        self
    }
    /// Sets the `is_flexible` parameter.
    pub fn is_flexible(mut self, val: bool) -> Self {
        self.is_flexible = Some(val);
        self
    }
    /// Sets the `subscription_period` parameter.
    pub fn subscription_period(mut self, val: i64) -> Self {
        self.subscription_period = Some(val);
        self
    }
    /// Sets the `business_connection_id` parameter.
    pub fn business_connection_id(mut self, val: impl Into<String>) -> Self {
        self.business_connection_id = Some(val.into());
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<String> {
        let mut params = vec![
            RequestParameter::new("title", serde_json::Value::String(self.title)),
            RequestParameter::new("description", serde_json::Value::String(self.description)),
            RequestParameter::new("payload", serde_json::Value::String(self.payload)),
            RequestParameter::new("currency", serde_json::Value::String(self.currency)),
            RequestParameter::new("prices", serde_json::to_value(&self.prices)?),
        ];
        push_opt_str(&mut params, "provider_token", &self.provider_token);
        push_opt(&mut params, "max_tip_amount", &self.max_tip_amount)?;
        push_opt(
            &mut params,
            "suggested_tip_amounts",
            &self.suggested_tip_amounts,
        )?;
        push_opt_str(&mut params, "provider_data", &self.provider_data);
        push_opt_str(&mut params, "photo_url", &self.photo_url);
        push_opt(&mut params, "photo_size", &self.photo_size)?;
        push_opt(&mut params, "photo_width", &self.photo_width)?;
        push_opt(&mut params, "photo_height", &self.photo_height)?;
        push_opt(&mut params, "need_name", &self.need_name)?;
        push_opt(&mut params, "need_phone_number", &self.need_phone_number)?;
        push_opt(&mut params, "need_email", &self.need_email)?;
        push_opt(
            &mut params,
            "need_shipping_address",
            &self.need_shipping_address,
        )?;
        push_opt(
            &mut params,
            "send_phone_number_to_provider",
            &self.send_phone_number_to_provider,
        )?;
        push_opt(
            &mut params,
            "send_email_to_provider",
            &self.send_email_to_provider,
        )?;
        push_opt(&mut params, "is_flexible", &self.is_flexible)?;
        push_opt(
            &mut params,
            "subscription_period",
            &self.subscription_period,
        )?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            &self.business_connection_id,
        );
        self.bot.do_api_request("createInvoiceLink", params).await
    }
}

impl_into_future!(CreateInvoiceLinkBuilder, String);

// =========================================================================
// EditUserStarSubscriptionBuilder
// =========================================================================

/// Builder for the [`editUserStarSubscription`] API method.
pub struct EditUserStarSubscriptionBuilder<'a> {
    bot: &'a Bot,
    user_id: i64,
    telegram_payment_charge_id: String,
    is_canceled: bool,
}

impl<'a> EditUserStarSubscriptionBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let params = vec![
            RequestParameter::new("user_id", serde_json::to_value(self.user_id)?),
            RequestParameter::new(
                "telegram_payment_charge_id",
                serde_json::Value::String(self.telegram_payment_charge_id),
            ),
            RequestParameter::new("is_canceled", serde_json::to_value(self.is_canceled)?),
        ];
        self.bot
            .do_api_request("editUserStarSubscription", params)
            .await
    }
}

impl_into_future!(EditUserStarSubscriptionBuilder, bool);

// =========================================================================
// GetMyStarBalanceBuilder
// =========================================================================

/// Builder for the [`getMyStarBalance`] API method.
pub struct GetMyStarBalanceBuilder<'a> {
    bot: &'a Bot,
}

impl<'a> GetMyStarBalanceBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<payment::stars::star_amount::StarAmount> {
        self.bot
            .do_api_request("getMyStarBalance", Vec::new())
            .await
    }
}

impl_into_future!(
    GetMyStarBalanceBuilder,
    payment::stars::star_amount::StarAmount
);

// =========================================================================
// GetStarTransactionsBuilder
// =========================================================================

/// Builder for the [`getStarTransactions`] API method.
pub struct GetStarTransactionsBuilder<'a> {
    bot: &'a Bot,
    offset: Option<i64>,
    limit: Option<i64>,
}

impl<'a> GetStarTransactionsBuilder<'a> {
    /// Sets the `offset` parameter.
    pub fn offset(mut self, val: i64) -> Self {
        self.offset = Some(val);
        self
    }
    /// Sets the `limit` parameter.
    pub fn limit(mut self, val: i64) -> Self {
        self.limit = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<payment::stars::star_transactions::StarTransactions> {
        let mut params = Vec::new();
        push_opt(&mut params, "offset", &self.offset)?;
        push_opt(&mut params, "limit", &self.limit)?;
        self.bot.do_api_request("getStarTransactions", params).await
    }
}

impl_into_future!(
    GetStarTransactionsBuilder,
    payment::stars::star_transactions::StarTransactions
);

// =========================================================================
// RefundStarPaymentBuilder
// =========================================================================

/// Builder for the [`refundStarPayment`] API method.
pub struct RefundStarPaymentBuilder<'a> {
    bot: &'a Bot,
    user_id: i64,
    telegram_payment_charge_id: String,
}

impl<'a> RefundStarPaymentBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let params = vec![
            RequestParameter::new("user_id", serde_json::to_value(self.user_id)?),
            RequestParameter::new(
                "telegram_payment_charge_id",
                serde_json::Value::String(self.telegram_payment_charge_id),
            ),
        ];
        self.bot.do_api_request("refundStarPayment", params).await
    }
}

impl_into_future!(RefundStarPaymentBuilder, bool);

// =========================================================================
// GIFT METHODS
// =========================================================================

// =========================================================================
// GetAvailableGiftsBuilder
// =========================================================================

/// Builder for the [`getAvailableGifts`] API method.
pub struct GetAvailableGiftsBuilder<'a> {
    bot: &'a Bot,
}

impl<'a> GetAvailableGiftsBuilder<'a> {
    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<gifts::Gifts> {
        self.bot
            .do_api_request("getAvailableGifts", Vec::new())
            .await
    }
}

impl_into_future!(GetAvailableGiftsBuilder, gifts::Gifts);

// =========================================================================
// SendGiftBuilder
// =========================================================================

/// Builder for the [`sendGift`] API method.
pub struct SendGiftBuilder<'a> {
    bot: &'a Bot,
    gift_id: String,
    user_id: Option<i64>,
    chat_id: Option<ChatId>,
    text: Option<String>,
    text_parse_mode: Option<String>,
    text_entities: Option<Vec<message_entity::MessageEntity>>,
    pay_for_upgrade: Option<bool>,
}

impl<'a> SendGiftBuilder<'a> {
    /// Sets the `user_id` parameter.
    pub fn user_id(mut self, val: i64) -> Self {
        self.user_id = Some(val);
        self
    }
    /// Sets the `chat_id` parameter.
    pub fn chat_id(mut self, val: impl Into<ChatId>) -> Self {
        self.chat_id = Some(val.into());
        self
    }
    /// Sets the `text` parameter.
    pub fn text(mut self, val: impl Into<String>) -> Self {
        self.text = Some(val.into());
        self
    }
    /// Sets the `text_parse_mode` parameter.
    pub fn text_parse_mode(mut self, val: impl Into<String>) -> Self {
        self.text_parse_mode = Some(val.into());
        self
    }
    /// Sets the `text_entities` parameter.
    pub fn text_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self {
        self.text_entities = Some(val);
        self
    }
    /// Sets the `pay_for_upgrade` parameter.
    pub fn pay_for_upgrade(mut self, val: bool) -> Self {
        self.pay_for_upgrade = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "gift_id",
            serde_json::Value::String(self.gift_id),
        )];
        push_opt(&mut params, "user_id", &self.user_id)?;
        push_opt(&mut params, "chat_id", &self.chat_id)?;
        push_opt_str(&mut params, "text", &self.text);
        push_opt_str(&mut params, "text_parse_mode", &self.text_parse_mode);
        push_opt(&mut params, "text_entities", &self.text_entities)?;
        push_opt(&mut params, "pay_for_upgrade", &self.pay_for_upgrade)?;
        self.bot.do_api_request("sendGift", params).await
    }
}

impl_into_future!(SendGiftBuilder, bool);

// =========================================================================
// GiftPremiumSubscriptionBuilder
// =========================================================================

/// Builder for the [`giftPremiumSubscription`] API method.
pub struct GiftPremiumSubscriptionBuilder<'a> {
    bot: &'a Bot,
    user_id: i64,
    month_count: i64,
    star_count: i64,
    text: Option<String>,
    text_parse_mode: Option<String>,
    text_entities: Option<Vec<message_entity::MessageEntity>>,
}

impl<'a> GiftPremiumSubscriptionBuilder<'a> {
    /// Sets the `text` parameter.
    pub fn text(mut self, val: impl Into<String>) -> Self {
        self.text = Some(val.into());
        self
    }
    /// Sets the `text_parse_mode` parameter.
    pub fn text_parse_mode(mut self, val: impl Into<String>) -> Self {
        self.text_parse_mode = Some(val.into());
        self
    }
    /// Sets the `text_entities` parameter.
    pub fn text_entities(mut self, val: Vec<message_entity::MessageEntity>) -> Self {
        self.text_entities = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("user_id", serde_json::to_value(self.user_id)?),
            RequestParameter::new("month_count", serde_json::to_value(self.month_count)?),
            RequestParameter::new("star_count", serde_json::to_value(self.star_count)?),
        ];
        push_opt_str(&mut params, "text", &self.text);
        push_opt_str(&mut params, "text_parse_mode", &self.text_parse_mode);
        push_opt(&mut params, "text_entities", &self.text_entities)?;
        self.bot
            .do_api_request("giftPremiumSubscription", params)
            .await
    }
}

impl_into_future!(GiftPremiumSubscriptionBuilder, bool);

// =========================================================================
// GetUserGiftsBuilder
// =========================================================================

/// Builder for the [`getUserGifts`] API method.
pub struct GetUserGiftsBuilder<'a> {
    bot: &'a Bot,
    user_id: i64,
    exclude_unlimited: Option<bool>,
    exclude_limited_upgradable: Option<bool>,
    exclude_limited_non_upgradable: Option<bool>,
    exclude_from_blockchain: Option<bool>,
    exclude_unique: Option<bool>,
    sort_by_price: Option<bool>,
    offset: Option<String>,
    limit: Option<i64>,
}

impl<'a> GetUserGiftsBuilder<'a> {
    /// Sets the `exclude_unlimited` parameter.
    pub fn exclude_unlimited(mut self, val: bool) -> Self {
        self.exclude_unlimited = Some(val);
        self
    }
    /// Sets the `exclude_limited_upgradable` parameter.
    pub fn exclude_limited_upgradable(mut self, val: bool) -> Self {
        self.exclude_limited_upgradable = Some(val);
        self
    }
    /// Sets the `exclude_limited_non_upgradable` parameter.
    pub fn exclude_limited_non_upgradable(mut self, val: bool) -> Self {
        self.exclude_limited_non_upgradable = Some(val);
        self
    }
    /// Sets the `exclude_from_blockchain` parameter.
    pub fn exclude_from_blockchain(mut self, val: bool) -> Self {
        self.exclude_from_blockchain = Some(val);
        self
    }
    /// Sets the `exclude_unique` parameter.
    pub fn exclude_unique(mut self, val: bool) -> Self {
        self.exclude_unique = Some(val);
        self
    }
    /// Sets the `sort_by_price` parameter.
    pub fn sort_by_price(mut self, val: bool) -> Self {
        self.sort_by_price = Some(val);
        self
    }
    /// Sets the `offset` parameter.
    pub fn offset(mut self, val: impl Into<String>) -> Self {
        self.offset = Some(val.into());
        self
    }
    /// Sets the `limit` parameter.
    pub fn limit(mut self, val: i64) -> Self {
        self.limit = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<owned_gift::OwnedGifts> {
        let mut params = vec![RequestParameter::new(
            "user_id",
            serde_json::to_value(self.user_id)?,
        )];
        push_opt(&mut params, "exclude_unlimited", &self.exclude_unlimited)?;
        push_opt(
            &mut params,
            "exclude_limited_upgradable",
            &self.exclude_limited_upgradable,
        )?;
        push_opt(
            &mut params,
            "exclude_limited_non_upgradable",
            &self.exclude_limited_non_upgradable,
        )?;
        push_opt(
            &mut params,
            "exclude_from_blockchain",
            &self.exclude_from_blockchain,
        )?;
        push_opt(&mut params, "exclude_unique", &self.exclude_unique)?;
        push_opt(&mut params, "sort_by_price", &self.sort_by_price)?;
        push_opt_str(&mut params, "offset", &self.offset);
        push_opt(&mut params, "limit", &self.limit)?;
        self.bot.do_api_request("getUserGifts", params).await
    }
}

impl_into_future!(GetUserGiftsBuilder, owned_gift::OwnedGifts);

// =========================================================================
// GetChatGiftsBuilder
// =========================================================================

/// Builder for the [`getChatGifts`] API method.
pub struct GetChatGiftsBuilder<'a> {
    bot: &'a Bot,
    chat_id: ChatId,
    exclude_unsaved: Option<bool>,
    exclude_saved: Option<bool>,
    exclude_unlimited: Option<bool>,
    exclude_limited_upgradable: Option<bool>,
    exclude_limited_non_upgradable: Option<bool>,
    exclude_from_blockchain: Option<bool>,
    exclude_unique: Option<bool>,
    sort_by_price: Option<bool>,
    offset: Option<String>,
    limit: Option<i64>,
}

impl<'a> GetChatGiftsBuilder<'a> {
    /// Sets the `exclude_unsaved` parameter.
    pub fn exclude_unsaved(mut self, val: bool) -> Self {
        self.exclude_unsaved = Some(val);
        self
    }
    /// Sets the `exclude_saved` parameter.
    pub fn exclude_saved(mut self, val: bool) -> Self {
        self.exclude_saved = Some(val);
        self
    }
    /// Sets the `exclude_unlimited` parameter.
    pub fn exclude_unlimited(mut self, val: bool) -> Self {
        self.exclude_unlimited = Some(val);
        self
    }
    /// Sets the `exclude_limited_upgradable` parameter.
    pub fn exclude_limited_upgradable(mut self, val: bool) -> Self {
        self.exclude_limited_upgradable = Some(val);
        self
    }
    /// Sets the `exclude_limited_non_upgradable` parameter.
    pub fn exclude_limited_non_upgradable(mut self, val: bool) -> Self {
        self.exclude_limited_non_upgradable = Some(val);
        self
    }
    /// Sets the `exclude_from_blockchain` parameter.
    pub fn exclude_from_blockchain(mut self, val: bool) -> Self {
        self.exclude_from_blockchain = Some(val);
        self
    }
    /// Sets the `exclude_unique` parameter.
    pub fn exclude_unique(mut self, val: bool) -> Self {
        self.exclude_unique = Some(val);
        self
    }
    /// Sets the `sort_by_price` parameter.
    pub fn sort_by_price(mut self, val: bool) -> Self {
        self.sort_by_price = Some(val);
        self
    }
    /// Sets the `offset` parameter.
    pub fn offset(mut self, val: impl Into<String>) -> Self {
        self.offset = Some(val.into());
        self
    }
    /// Sets the `limit` parameter.
    pub fn limit(mut self, val: i64) -> Self {
        self.limit = Some(val);
        self
    }

    /// Sends the request to the Telegram Bot API.
    pub async fn send(self) -> Result<owned_gift::OwnedGifts> {
        let mut params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&self.chat_id)?,
        )];
        push_opt(&mut params, "exclude_unsaved", &self.exclude_unsaved)?;
        push_opt(&mut params, "exclude_saved", &self.exclude_saved)?;
        push_opt(&mut params, "exclude_unlimited", &self.exclude_unlimited)?;
        push_opt(
            &mut params,
            "exclude_limited_upgradable",
            &self.exclude_limited_upgradable,
        )?;
        push_opt(
            &mut params,
            "exclude_limited_non_upgradable",
            &self.exclude_limited_non_upgradable,
        )?;
        push_opt(
            &mut params,
            "exclude_from_blockchain",
            &self.exclude_from_blockchain,
        )?;
        push_opt(&mut params, "exclude_unique", &self.exclude_unique)?;
        push_opt(&mut params, "sort_by_price", &self.sort_by_price)?;
        push_opt_str(&mut params, "offset", &self.offset);
        push_opt(&mut params, "limit", &self.limit)?;
        self.bot.do_api_request("getChatGifts", params).await
    }
}

impl_into_future!(GetChatGiftsBuilder, owned_gift::OwnedGifts);

// =========================================================================
// Factory methods on Bot
// =========================================================================

impl Bot {
    // -- Business account management -----------------------------------------

    /// Build a `getBusinessConnection` request.
    pub fn get_business_connection(
        &self,
        business_connection_id: impl Into<String>,
    ) -> GetBusinessConnectionBuilder<'_> {
        GetBusinessConnectionBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
        }
    }

    /// Build a `getBusinessAccountGifts` request.
    pub fn get_business_account_gifts(
        &self,
        business_connection_id: impl Into<String>,
    ) -> GetBusinessAccountGiftsBuilder<'_> {
        GetBusinessAccountGiftsBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
            exclude_unsaved: None,
            exclude_saved: None,
            exclude_unlimited: None,
            exclude_unique: None,
            sort_by_price: None,
            offset: None,
            limit: None,
            exclude_limited_upgradable: None,
            exclude_limited_non_upgradable: None,
            exclude_from_blockchain: None,
        }
    }

    /// Build a `getBusinessAccountStarBalance` request.
    pub fn get_business_account_star_balance(
        &self,
        business_connection_id: impl Into<String>,
    ) -> GetBusinessAccountStarBalanceBuilder<'_> {
        GetBusinessAccountStarBalanceBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
        }
    }

    /// Build a `readBusinessMessage` request.
    pub fn read_business_message(
        &self,
        business_connection_id: impl Into<String>,
        chat_id: i64,
        message_id: i64,
    ) -> ReadBusinessMessageBuilder<'_> {
        ReadBusinessMessageBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
            chat_id,
            message_id,
        }
    }

    /// Build a `deleteBusinessMessages` request.
    pub fn delete_business_messages(
        &self,
        business_connection_id: impl Into<String>,
        message_ids: Vec<i64>,
    ) -> DeleteBusinessMessagesBuilder<'_> {
        DeleteBusinessMessagesBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
            message_ids,
        }
    }

    /// Build a `setBusinessAccountName` request.
    pub fn set_business_account_name(
        &self,
        business_connection_id: impl Into<String>,
        first_name: impl Into<String>,
    ) -> SetBusinessAccountNameBuilder<'_> {
        SetBusinessAccountNameBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
            first_name: first_name.into(),
            last_name: None,
        }
    }

    /// Build a `setBusinessAccountUsername` request.
    pub fn set_business_account_username(
        &self,
        business_connection_id: impl Into<String>,
    ) -> SetBusinessAccountUsernameBuilder<'_> {
        SetBusinessAccountUsernameBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
            username: None,
        }
    }

    /// Build a `setBusinessAccountBio` request.
    pub fn set_business_account_bio(
        &self,
        business_connection_id: impl Into<String>,
    ) -> SetBusinessAccountBioBuilder<'_> {
        SetBusinessAccountBioBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
            bio: None,
        }
    }

    /// Build a `setBusinessAccountGiftSettings` request.
    pub fn set_business_account_gift_settings(
        &self,
        business_connection_id: impl Into<String>,
        show_gift_button: bool,
        accepted_gift_types: gifts::AcceptedGiftTypes,
    ) -> SetBusinessAccountGiftSettingsBuilder<'_> {
        SetBusinessAccountGiftSettingsBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
            show_gift_button,
            accepted_gift_types,
        }
    }

    /// Build a `setBusinessAccountProfilePhoto` request.
    pub fn set_business_account_profile_photo(
        &self,
        business_connection_id: impl Into<String>,
        photo: serde_json::Value,
    ) -> SetBusinessAccountProfilePhotoBuilder<'_> {
        SetBusinessAccountProfilePhotoBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
            photo,
            is_public: None,
        }
    }

    /// Build a `removeBusinessAccountProfilePhoto` request.
    pub fn remove_business_account_profile_photo(
        &self,
        business_connection_id: impl Into<String>,
    ) -> RemoveBusinessAccountProfilePhotoBuilder<'_> {
        RemoveBusinessAccountProfilePhotoBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
            is_public: None,
        }
    }

    /// Build a `convertGiftToStars` request.
    pub fn convert_gift_to_stars(
        &self,
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
    ) -> ConvertGiftToStarsBuilder<'_> {
        ConvertGiftToStarsBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
            owned_gift_id: owned_gift_id.into(),
        }
    }

    /// Build an `upgradeGift` request.
    pub fn upgrade_gift(
        &self,
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
    ) -> UpgradeGiftBuilder<'_> {
        UpgradeGiftBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
            owned_gift_id: owned_gift_id.into(),
            keep_original_details: None,
            star_count: None,
        }
    }

    /// Build a `transferGift` request.
    pub fn transfer_gift(
        &self,
        business_connection_id: impl Into<String>,
        owned_gift_id: impl Into<String>,
        new_owner_chat_id: i64,
    ) -> TransferGiftBuilder<'_> {
        TransferGiftBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
            owned_gift_id: owned_gift_id.into(),
            new_owner_chat_id,
            star_count: None,
        }
    }

    /// Build a `transferBusinessAccountStars` request.
    pub fn transfer_business_account_stars(
        &self,
        business_connection_id: impl Into<String>,
        star_count: i64,
    ) -> TransferBusinessAccountStarsBuilder<'_> {
        TransferBusinessAccountStarsBuilder {
            bot: self,
            business_connection_id: business_connection_id.into(),
            star_count,
        }
    }

    // -- Payments ------------------------------------------------------------

    /// Build a `createInvoiceLink` request.
    pub fn create_invoice_link(
        &self,
        title: impl Into<String>,
        description: impl Into<String>,
        payload: impl Into<String>,
        currency: impl Into<String>,
        prices: Vec<serde_json::Value>,
    ) -> CreateInvoiceLinkBuilder<'_> {
        CreateInvoiceLinkBuilder {
            bot: self,
            title: title.into(),
            description: description.into(),
            payload: payload.into(),
            currency: currency.into(),
            prices,
            provider_token: None,
            max_tip_amount: None,
            suggested_tip_amounts: None,
            provider_data: None,
            photo_url: None,
            photo_size: None,
            photo_width: None,
            photo_height: None,
            need_name: None,
            need_phone_number: None,
            need_email: None,
            need_shipping_address: None,
            send_phone_number_to_provider: None,
            send_email_to_provider: None,
            is_flexible: None,
            subscription_period: None,
            business_connection_id: None,
        }
    }

    /// Build an `editUserStarSubscription` request.
    pub fn edit_user_star_subscription(
        &self,
        user_id: i64,
        telegram_payment_charge_id: impl Into<String>,
        is_canceled: bool,
    ) -> EditUserStarSubscriptionBuilder<'_> {
        EditUserStarSubscriptionBuilder {
            bot: self,
            user_id,
            telegram_payment_charge_id: telegram_payment_charge_id.into(),
            is_canceled,
        }
    }

    /// Build a `getMyStarBalance` request.
    pub fn get_my_star_balance(&self) -> GetMyStarBalanceBuilder<'_> {
        GetMyStarBalanceBuilder { bot: self }
    }

    /// Build a `getStarTransactions` request.
    pub fn get_star_transactions(&self) -> GetStarTransactionsBuilder<'_> {
        GetStarTransactionsBuilder {
            bot: self,
            offset: None,
            limit: None,
        }
    }

    /// Build a `refundStarPayment` request.
    pub fn refund_star_payment(
        &self,
        user_id: i64,
        telegram_payment_charge_id: impl Into<String>,
    ) -> RefundStarPaymentBuilder<'_> {
        RefundStarPaymentBuilder {
            bot: self,
            user_id,
            telegram_payment_charge_id: telegram_payment_charge_id.into(),
        }
    }

    // -- Gifts ---------------------------------------------------------------

    /// Build a `getAvailableGifts` request.
    pub fn get_available_gifts(&self) -> GetAvailableGiftsBuilder<'_> {
        GetAvailableGiftsBuilder { bot: self }
    }

    /// Build a `sendGift` request.
    pub fn send_gift(&self, gift_id: impl Into<String>) -> SendGiftBuilder<'_> {
        SendGiftBuilder {
            bot: self,
            gift_id: gift_id.into(),
            user_id: None,
            chat_id: None,
            text: None,
            text_parse_mode: None,
            text_entities: None,
            pay_for_upgrade: None,
        }
    }

    /// Build a `giftPremiumSubscription` request.
    pub fn gift_premium_subscription(
        &self,
        user_id: i64,
        month_count: i64,
        star_count: i64,
    ) -> GiftPremiumSubscriptionBuilder<'_> {
        GiftPremiumSubscriptionBuilder {
            bot: self,
            user_id,
            month_count,
            star_count,
            text: None,
            text_parse_mode: None,
            text_entities: None,
        }
    }

    /// Build a `getUserGifts` request.
    pub fn get_user_gifts(&self, user_id: i64) -> GetUserGiftsBuilder<'_> {
        GetUserGiftsBuilder {
            bot: self,
            user_id,
            exclude_unlimited: None,
            exclude_limited_upgradable: None,
            exclude_limited_non_upgradable: None,
            exclude_from_blockchain: None,
            exclude_unique: None,
            sort_by_price: None,
            offset: None,
            limit: None,
        }
    }

    /// Build a `getChatGifts` request.
    pub fn get_chat_gifts(&self, chat_id: impl Into<ChatId>) -> GetChatGiftsBuilder<'_> {
        GetChatGiftsBuilder {
            bot: self,
            chat_id: chat_id.into(),
            exclude_unsaved: None,
            exclude_saved: None,
            exclude_unlimited: None,
            exclude_limited_upgradable: None,
            exclude_limited_non_upgradable: None,
            exclude_from_blockchain: None,
            exclude_unique: None,
            sort_by_price: None,
            offset: None,
            limit: None,
        }
    }
}
