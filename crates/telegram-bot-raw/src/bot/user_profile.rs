use super::*;

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // User and profile
    // ======================================================================

    pub async fn get_user_profile_photos(
        &self,
        user_id: i64,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<user_profile_photos::UserProfilePhotos> {
        let mut params = vec![RequestParameter::new(
            "user_id",
            serde_json::to_value(user_id)?,
        )];
        push_opt(&mut params, "offset", &offset)?;
        push_opt(&mut params, "limit", &limit)?;
        self.do_post("getUserProfilePhotos", params).await
    }

    pub async fn get_user_profile_audios(
        &self,
        user_id: i64,
        offset: Option<i64>,
        limit: Option<i64>,
    ) -> Result<user_profile_audios::UserProfileAudios> {
        let mut params = vec![RequestParameter::new(
            "user_id",
            serde_json::to_value(user_id)?,
        )];
        push_opt(&mut params, "offset", &offset)?;
        push_opt(&mut params, "limit", &limit)?;
        self.do_post("getUserProfileAudios", params).await
    }

    pub async fn set_user_emoji_status(
        &self,
        user_id: i64,
        emoji_status_custom_emoji_id: Option<&str>,
        emoji_status_expiration_date: Option<i64>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "user_id",
            serde_json::to_value(user_id)?,
        )];
        push_opt_str(
            &mut params,
            "emoji_status_custom_emoji_id",
            emoji_status_custom_emoji_id,
        );
        push_opt(
            &mut params,
            "emoji_status_expiration_date",
            &emoji_status_expiration_date,
        )?;
        self.do_post("setUserEmojiStatus", params).await
    }

    pub async fn set_my_profile_photo(&self, photo: serde_json::Value) -> Result<bool> {
        let params = vec![RequestParameter::new("photo", photo)];
        self.do_post("setMyProfilePhoto", params).await
    }

    pub async fn remove_my_profile_photo(&self) -> Result<bool> {
        self.do_post("removeMyProfilePhoto", Vec::new()).await
    }

    // ======================================================================
    // Files
    // ======================================================================

    pub(crate) async fn get_file_raw(&self, file_id: &str) -> Result<files::file::File> {
        let params = vec![RequestParameter::new(
            "file_id",
            serde_json::Value::String(file_id.to_owned()),
        )];
        self.do_post("getFile", params).await
    }
}
