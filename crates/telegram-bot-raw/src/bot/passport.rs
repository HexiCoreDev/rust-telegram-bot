use super::*;

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Passport
    // ======================================================================

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
