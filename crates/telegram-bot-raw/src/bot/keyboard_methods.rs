use super::*;

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Keyboard button methods (Bot API 9.6)
    // ======================================================================

    /// Use this method to save a keyboard button to be later shown to the
    /// user via a Mini App.
    ///
    /// Returns a [`PreparedKeyboardButton`](crate::types::prepared_keyboard_button::PreparedKeyboardButton)
    /// on success.
    pub(crate) async fn save_prepared_keyboard_button_raw(
        &self,
        user_id: i64,
        button: inline::inline_keyboard_button::InlineKeyboardButton,
    ) -> Result<prepared_keyboard_button::PreparedKeyboardButton> {
        let params = vec![
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("button", serde_json::to_value(&button)?),
        ];
        self.do_post("savePreparedKeyboardButton", params).await
    }
}
