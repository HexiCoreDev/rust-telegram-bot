use super::{push_opt, push_opt_str, Bot, Result};
use crate::request::request_parameter::RequestParameter;
use crate::types::{files, user_profile_audios, user_profile_photos};

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // User and profile
    // ======================================================================

    /// Use this method to get a list of profile pictures for a user.
    ///
    /// Calls the Telegram `getUserProfilePhotos` API method.
    pub async fn get_user_profile_photos_raw(
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

    /// Use this method to get a list of profile audios for a user.
    ///
    /// Calls the Telegram `getUserProfileAudios` API method.
    pub async fn get_user_profile_audios_raw(
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

    /// Use this method to change the emoji status for a given user.
    ///
    /// Calls the Telegram `setUserEmojiStatus` API method.
    pub async fn set_user_emoji_status_raw(
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

    /// Use this method to set the bot's profile photo.
    ///
    /// Calls the Telegram `setMyProfilePhoto` API method.
    pub async fn set_my_profile_photo_raw(&self, photo: serde_json::Value) -> Result<bool> {
        let params = vec![RequestParameter::new("photo", photo)];
        self.do_post("setMyProfilePhoto", params).await
    }

    /// Use this method to remove the bot's profile photo.
    ///
    /// Calls the Telegram `removeMyProfilePhoto` API method.
    pub async fn remove_my_profile_photo_raw(&self) -> Result<bool> {
        self.do_post("removeMyProfilePhoto", Vec::new()).await
    }

    // ======================================================================
    // Files
    // ======================================================================

    /// Gets basic info about a file and prepares it for downloading.
    /// Internal raw method used by builder APIs.
    ///
    /// Calls the Telegram `getFile` API method.
    pub async fn get_file_raw(&self, file_id: &str) -> Result<files::file::File> {
        let params = vec![RequestParameter::new(
            "file_id",
            serde_json::Value::String(file_id.to_owned()),
        )];
        self.do_post("getFile", params).await
    }
}
