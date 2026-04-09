use super::{Bot, Result};
use crate::request::request_parameter::RequestParameter;

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Passport
    // ======================================================================

    /// Use this method to inform a user that some of the Telegram Passport elements contain errors.
    ///
    /// Calls the Telegram `setPassportDataErrors` API method.
    pub async fn set_passport_data_errors(
        &self,
        user_id: i64,
        errors: Vec<serde_json::Value>,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("errors", serde_json::to_value(&errors)?),
        ];
        self.do_post("setPassportDataErrors", params).await
    }
}
