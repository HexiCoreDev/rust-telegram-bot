use super::{push_opt, push_opt_str, Bot, ChatId, Result};
use crate::request::request_parameter::RequestParameter;
use crate::types::{message, payment, reply, suggested_post};

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Payments
    // ======================================================================

    /// Sends an invoice. Internal raw method used by builder APIs.
    ///
    /// Calls the Telegram `sendInvoice` API method.
    pub async fn send_invoice_raw(
        &self,
        chat_id: ChatId,
        title: &str,
        description: &str,
        payload: &str,
        currency: &str,
        prices: Vec<serde_json::Value>,
        provider_token: Option<&str>,
        max_tip_amount: Option<i64>,
        suggested_tip_amounts: Option<Vec<i64>>,
        start_parameter: Option<&str>,
        provider_data: Option<&str>,
        photo_url: Option<&str>,
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
        disable_notification: Option<bool>,
        protect_content: Option<bool>,
        reply_parameters: Option<reply::ReplyParameters>,
        reply_markup: Option<serde_json::Value>,
        message_thread_id: Option<i64>,
        message_effect_id: Option<&str>,
        allow_paid_broadcast: Option<bool>,
        direct_messages_topic_id: Option<i64>,
        suggested_post_parameters: Option<suggested_post::SuggestedPostParameters>,
    ) -> Result<message::Message> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("title", serde_json::Value::String(title.to_owned())),
            RequestParameter::new(
                "description",
                serde_json::Value::String(description.to_owned()),
            ),
            RequestParameter::new("payload", serde_json::Value::String(payload.to_owned())),
            RequestParameter::new("currency", serde_json::Value::String(currency.to_owned())),
            RequestParameter::new("prices", serde_json::to_value(&prices)?),
        ];
        push_opt_str(&mut params, "provider_token", provider_token);
        push_opt(&mut params, "max_tip_amount", &max_tip_amount)?;
        push_opt(&mut params, "suggested_tip_amounts", &suggested_tip_amounts)?;
        push_opt_str(&mut params, "start_parameter", start_parameter);
        push_opt_str(&mut params, "provider_data", provider_data);
        push_opt_str(&mut params, "photo_url", photo_url);
        push_opt(&mut params, "photo_size", &photo_size)?;
        push_opt(&mut params, "photo_width", &photo_width)?;
        push_opt(&mut params, "photo_height", &photo_height)?;
        push_opt(&mut params, "need_name", &need_name)?;
        push_opt(&mut params, "need_phone_number", &need_phone_number)?;
        push_opt(&mut params, "need_email", &need_email)?;
        push_opt(&mut params, "need_shipping_address", &need_shipping_address)?;
        push_opt(
            &mut params,
            "send_phone_number_to_provider",
            &send_phone_number_to_provider,
        )?;
        push_opt(
            &mut params,
            "send_email_to_provider",
            &send_email_to_provider,
        )?;
        push_opt(&mut params, "is_flexible", &is_flexible)?;
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        push_opt(&mut params, "reply_parameters", &reply_parameters)?;
        push_opt(&mut params, "reply_markup", &reply_markup)?;
        push_opt(&mut params, "message_thread_id", &message_thread_id)?;
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
        self.do_post("sendInvoice", params).await
    }

    /// Use this method to create a link for an invoice.
    ///
    /// Calls the Telegram `createInvoiceLink` API method.
    pub async fn create_invoice_link_raw(
        &self,
        title: &str,
        description: &str,
        payload: &str,
        currency: &str,
        prices: Vec<serde_json::Value>,
        provider_token: Option<&str>,
        max_tip_amount: Option<i64>,
        suggested_tip_amounts: Option<Vec<i64>>,
        provider_data: Option<&str>,
        photo_url: Option<&str>,
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
        business_connection_id: Option<&str>,
    ) -> Result<String> {
        let mut params = vec![
            RequestParameter::new("title", serde_json::Value::String(title.to_owned())),
            RequestParameter::new(
                "description",
                serde_json::Value::String(description.to_owned()),
            ),
            RequestParameter::new("payload", serde_json::Value::String(payload.to_owned())),
            RequestParameter::new("currency", serde_json::Value::String(currency.to_owned())),
            RequestParameter::new("prices", serde_json::to_value(&prices)?),
        ];
        push_opt_str(&mut params, "provider_token", provider_token);
        push_opt(&mut params, "max_tip_amount", &max_tip_amount)?;
        push_opt(&mut params, "suggested_tip_amounts", &suggested_tip_amounts)?;
        push_opt_str(&mut params, "provider_data", provider_data);
        push_opt_str(&mut params, "photo_url", photo_url);
        push_opt(&mut params, "photo_size", &photo_size)?;
        push_opt(&mut params, "photo_width", &photo_width)?;
        push_opt(&mut params, "photo_height", &photo_height)?;
        push_opt(&mut params, "need_name", &need_name)?;
        push_opt(&mut params, "need_phone_number", &need_phone_number)?;
        push_opt(&mut params, "need_email", &need_email)?;
        push_opt(&mut params, "need_shipping_address", &need_shipping_address)?;
        push_opt(
            &mut params,
            "send_phone_number_to_provider",
            &send_phone_number_to_provider,
        )?;
        push_opt(
            &mut params,
            "send_email_to_provider",
            &send_email_to_provider,
        )?;
        push_opt(&mut params, "is_flexible", &is_flexible)?;
        push_opt(&mut params, "subscription_period", &subscription_period)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            business_connection_id,
        );
        self.do_post("createInvoiceLink", params).await
    }

    /// Answers a shipping query. Internal raw method.
    ///
    /// Calls the Telegram `answerShippingQuery` API method.
    pub async fn answer_shipping_query_raw(
        &self,
        shipping_query_id: &str,
        ok: bool,
        shipping_options: Option<Vec<serde_json::Value>>,
        error_message: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new(
                "shipping_query_id",
                serde_json::Value::String(shipping_query_id.to_owned()),
            ),
            RequestParameter::new("ok", serde_json::to_value(ok)?),
        ];
        push_opt(&mut params, "shipping_options", &shipping_options)?;
        push_opt_str(&mut params, "error_message", error_message);
        self.do_post("answerShippingQuery", params).await
    }

    /// Answers a pre-checkout query. Internal raw method.
    ///
    /// Calls the Telegram `answerPreCheckoutQuery` API method.
    pub async fn answer_pre_checkout_query_raw(
        &self,
        pre_checkout_query_id: &str,
        ok: bool,
        error_message: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new(
                "pre_checkout_query_id",
                serde_json::Value::String(pre_checkout_query_id.to_owned()),
            ),
            RequestParameter::new("ok", serde_json::to_value(ok)?),
        ];
        push_opt_str(&mut params, "error_message", error_message);
        self.do_post("answerPreCheckoutQuery", params).await
    }

    /// Use this method to refund a successful payment in Telegram Stars.
    ///
    /// Calls the Telegram `refundStarPayment` API method.
    pub async fn refund_star_payment_raw(
        &self,
        user_id: i64,
        telegram_payment_charge_id: &str,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new(
                "telegram_payment_charge_id",
                serde_json::Value::String(telegram_payment_charge_id.to_owned()),
            ),
        ];
        self.do_post("refundStarPayment", params).await
    }

    /// Use this method to get the bot's Telegram Star transactions.
    ///
    /// Calls the Telegram `getStarTransactions` API method.
    pub async fn get_star_transactions_raw(
        &self,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<payment::stars::star_transactions::StarTransactions> {
        let mut params = Vec::new();
        push_opt(&mut params, "offset", &offset)?;
        push_opt(&mut params, "limit", &limit)?;
        self.do_post("getStarTransactions", params).await
    }

    /// Use this method to allow or disallow the user to create Telegram Star subscriptions.
    ///
    /// Calls the Telegram `editUserStarSubscription` API method.
    pub async fn edit_user_star_subscription_raw(
        &self,
        user_id: i64,
        telegram_payment_charge_id: &str,
        is_canceled: bool,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new(
                "telegram_payment_charge_id",
                serde_json::Value::String(telegram_payment_charge_id.to_owned()),
            ),
            RequestParameter::new("is_canceled", serde_json::to_value(is_canceled)?),
        ];
        self.do_post("editUserStarSubscription", params).await
    }

    /// Use this method to get the bot's current Telegram Star balance.
    ///
    /// Calls the Telegram `getMyStarBalance` API method.
    pub async fn get_my_star_balance_raw(&self) -> Result<payment::stars::star_amount::StarAmount> {
        self.do_post("getMyStarBalance", Vec::new()).await
    }
}
