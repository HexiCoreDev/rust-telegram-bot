use super::{input_file_param, push_opt, push_opt_str, Bot, ChatId, Result};
use crate::request::request_parameter::RequestParameter;
use crate::types::{
    chat_full_info, chat_invite_link, chat_member, chat_permissions, files,
};

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Chat management
    // ======================================================================

    pub async fn leave_chat(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("leaveChat", params).await
    }

    pub async fn get_chat(&self, chat_id: ChatId) -> Result<chat_full_info::ChatFullInfo> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("getChat", params).await
    }

    pub async fn get_chat_administrators(
        &self,
        chat_id: ChatId,
    ) -> Result<Vec<chat_member::ChatMember>> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("getChatAdministrators", params).await
    }

    pub async fn get_chat_member_count(&self, chat_id: ChatId) -> Result<i64> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("getChatMemberCount", params).await
    }

    pub async fn get_chat_member(
        &self,
        chat_id: ChatId,
        user_id: i64,
    ) -> Result<chat_member::ChatMember> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
        ];
        self.do_post("getChatMember", params).await
    }

    pub async fn ban_chat_member(
        &self,
        chat_id: ChatId,
        user_id: i64,
        until_date: Option<i64>,
        revoke_messages: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
        ];
        push_opt(&mut params, "until_date", &until_date)?;
        push_opt(&mut params, "revoke_messages", &revoke_messages)?;
        self.do_post("banChatMember", params).await
    }

    pub async fn unban_chat_member(
        &self,
        chat_id: ChatId,
        user_id: i64,
        only_if_banned: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
        ];
        push_opt(&mut params, "only_if_banned", &only_if_banned)?;
        self.do_post("unbanChatMember", params).await
    }

    pub async fn ban_chat_sender_chat(&self, chat_id: ChatId, sender_chat_id: i64) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("sender_chat_id", serde_json::to_value(sender_chat_id)?),
        ];
        self.do_post("banChatSenderChat", params).await
    }

    pub async fn unban_chat_sender_chat(
        &self,
        chat_id: ChatId,
        sender_chat_id: i64,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("sender_chat_id", serde_json::to_value(sender_chat_id)?),
        ];
        self.do_post("unbanChatSenderChat", params).await
    }

    pub async fn restrict_chat_member(
        &self,
        chat_id: ChatId,
        user_id: i64,
        permissions: chat_permissions::ChatPermissions,
        until_date: Option<i64>,
        use_independent_chat_permissions: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new("permissions", serde_json::to_value(&permissions)?),
        ];
        push_opt(&mut params, "until_date", &until_date)?;
        push_opt(
            &mut params,
            "use_independent_chat_permissions",
            &use_independent_chat_permissions,
        )?;
        self.do_post("restrictChatMember", params).await
    }

    pub async fn promote_chat_member(
        &self,
        chat_id: ChatId,
        user_id: i64,
        is_anonymous: Option<bool>,
        can_manage_chat: Option<bool>,
        can_post_messages: Option<bool>,
        can_edit_messages: Option<bool>,
        can_delete_messages: Option<bool>,
        can_manage_video_chats: Option<bool>,
        can_restrict_members: Option<bool>,
        can_promote_members: Option<bool>,
        can_change_info: Option<bool>,
        can_invite_users: Option<bool>,
        can_pin_messages: Option<bool>,
        can_manage_topics: Option<bool>,
        can_post_stories: Option<bool>,
        can_edit_stories: Option<bool>,
        can_delete_stories: Option<bool>,
        can_manage_direct_messages: Option<bool>,
        can_manage_tags: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
        ];
        push_opt(&mut params, "is_anonymous", &is_anonymous)?;
        push_opt(&mut params, "can_manage_chat", &can_manage_chat)?;
        push_opt(&mut params, "can_post_messages", &can_post_messages)?;
        push_opt(&mut params, "can_edit_messages", &can_edit_messages)?;
        push_opt(&mut params, "can_delete_messages", &can_delete_messages)?;
        push_opt(
            &mut params,
            "can_manage_video_chats",
            &can_manage_video_chats,
        )?;
        push_opt(&mut params, "can_restrict_members", &can_restrict_members)?;
        push_opt(&mut params, "can_promote_members", &can_promote_members)?;
        push_opt(&mut params, "can_change_info", &can_change_info)?;
        push_opt(&mut params, "can_invite_users", &can_invite_users)?;
        push_opt(&mut params, "can_pin_messages", &can_pin_messages)?;
        push_opt(&mut params, "can_manage_topics", &can_manage_topics)?;
        push_opt(&mut params, "can_post_stories", &can_post_stories)?;
        push_opt(&mut params, "can_edit_stories", &can_edit_stories)?;
        push_opt(&mut params, "can_delete_stories", &can_delete_stories)?;
        push_opt(
            &mut params,
            "can_manage_direct_messages",
            &can_manage_direct_messages,
        )?;
        push_opt(&mut params, "can_manage_tags", &can_manage_tags)?;
        self.do_post("promoteChatMember", params).await
    }

    pub async fn set_chat_administrator_custom_title(
        &self,
        chat_id: ChatId,
        user_id: i64,
        custom_title: &str,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
            RequestParameter::new(
                "custom_title",
                serde_json::Value::String(custom_title.to_owned()),
            ),
        ];
        self.do_post("setChatAdministratorCustomTitle", params)
            .await
    }

    pub async fn set_chat_permissions(
        &self,
        chat_id: ChatId,
        permissions: chat_permissions::ChatPermissions,
        use_independent_chat_permissions: Option<bool>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("permissions", serde_json::to_value(&permissions)?),
        ];
        push_opt(
            &mut params,
            "use_independent_chat_permissions",
            &use_independent_chat_permissions,
        )?;
        self.do_post("setChatPermissions", params).await
    }

    pub async fn set_chat_photo(
        &self,
        chat_id: ChatId,
        photo: files::input_file::InputFile,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            input_file_param("photo", photo),
        ];
        self.do_post("setChatPhoto", params).await
    }

    pub async fn delete_chat_photo(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("deleteChatPhoto", params).await
    }

    pub async fn set_chat_title(&self, chat_id: ChatId, title: &str) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("title", serde_json::Value::String(title.to_owned())),
        ];
        self.do_post("setChatTitle", params).await
    }

    pub async fn set_chat_description(
        &self,
        chat_id: ChatId,
        description: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        push_opt_str(&mut params, "description", description);
        self.do_post("setChatDescription", params).await
    }

    pub async fn set_chat_sticker_set(
        &self,
        chat_id: ChatId,
        sticker_set_name: &str,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new(
                "sticker_set_name",
                serde_json::Value::String(sticker_set_name.to_owned()),
            ),
        ];
        self.do_post("setChatStickerSet", params).await
    }

    pub async fn delete_chat_sticker_set(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("deleteChatStickerSet", params).await
    }

    pub async fn set_chat_member_tag(
        &self,
        chat_id: ChatId,
        user_id: i64,
        tag: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
        ];
        push_opt_str(&mut params, "tag", tag);
        self.do_post("setChatMemberTag", params).await
    }

    // ======================================================================
    // Chat pinning
    // ======================================================================

    pub async fn pin_chat_message(
        &self,
        chat_id: ChatId,
        message_id: i64,
        disable_notification: Option<bool>,
        business_connection_id: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("message_id", serde_json::to_value(message_id)?),
        ];
        push_opt(&mut params, "disable_notification", &disable_notification)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            business_connection_id,
        );
        self.do_post("pinChatMessage", params).await
    }

    pub async fn unpin_chat_message(
        &self,
        chat_id: ChatId,
        message_id: Option<i64>,
        business_connection_id: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        push_opt(&mut params, "message_id", &message_id)?;
        push_opt_str(
            &mut params,
            "business_connection_id",
            business_connection_id,
        );
        self.do_post("unpinChatMessage", params).await
    }

    pub async fn unpin_all_chat_messages(&self, chat_id: ChatId) -> Result<bool> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("unpinAllChatMessages", params).await
    }

    // ======================================================================
    // Chat invite links
    // ======================================================================

    pub async fn export_chat_invite_link(&self, chat_id: ChatId) -> Result<String> {
        let params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        self.do_post("exportChatInviteLink", params).await
    }

    pub async fn create_chat_invite_link(
        &self,
        chat_id: ChatId,
        expire_date: Option<i64>,
        member_limit: Option<i64>,
        name: Option<&str>,
        creates_join_request: Option<bool>,
    ) -> Result<chat_invite_link::ChatInviteLink> {
        let mut params = vec![RequestParameter::new(
            "chat_id",
            serde_json::to_value(&chat_id)?,
        )];
        push_opt(&mut params, "expire_date", &expire_date)?;
        push_opt(&mut params, "member_limit", &member_limit)?;
        push_opt_str(&mut params, "name", name);
        push_opt(&mut params, "creates_join_request", &creates_join_request)?;
        self.do_post("createChatInviteLink", params).await
    }

    pub async fn edit_chat_invite_link(
        &self,
        chat_id: ChatId,
        invite_link: &str,
        expire_date: Option<i64>,
        member_limit: Option<i64>,
        name: Option<&str>,
        creates_join_request: Option<bool>,
    ) -> Result<chat_invite_link::ChatInviteLink> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new(
                "invite_link",
                serde_json::Value::String(invite_link.to_owned()),
            ),
        ];
        push_opt(&mut params, "expire_date", &expire_date)?;
        push_opt(&mut params, "member_limit", &member_limit)?;
        push_opt_str(&mut params, "name", name);
        push_opt(&mut params, "creates_join_request", &creates_join_request)?;
        self.do_post("editChatInviteLink", params).await
    }

    pub async fn revoke_chat_invite_link(
        &self,
        chat_id: ChatId,
        invite_link: &str,
    ) -> Result<chat_invite_link::ChatInviteLink> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new(
                "invite_link",
                serde_json::Value::String(invite_link.to_owned()),
            ),
        ];
        self.do_post("revokeChatInviteLink", params).await
    }

    pub async fn create_chat_subscription_invite_link(
        &self,
        chat_id: ChatId,
        subscription_period: i64,
        subscription_price: i64,
        name: Option<&str>,
    ) -> Result<chat_invite_link::ChatInviteLink> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new(
                "subscription_period",
                serde_json::to_value(subscription_period)?,
            ),
            RequestParameter::new(
                "subscription_price",
                serde_json::to_value(subscription_price)?,
            ),
        ];
        push_opt_str(&mut params, "name", name);
        self.do_post("createChatSubscriptionInviteLink", params)
            .await
    }

    pub async fn edit_chat_subscription_invite_link(
        &self,
        chat_id: ChatId,
        invite_link: &str,
        name: Option<&str>,
    ) -> Result<chat_invite_link::ChatInviteLink> {
        let mut params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new(
                "invite_link",
                serde_json::Value::String(invite_link.to_owned()),
            ),
        ];
        push_opt_str(&mut params, "name", name);
        self.do_post("editChatSubscriptionInviteLink", params).await
    }

    pub async fn approve_chat_join_request(&self, chat_id: ChatId, user_id: i64) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
        ];
        self.do_post("approveChatJoinRequest", params).await
    }

    pub async fn decline_chat_join_request(&self, chat_id: ChatId, user_id: i64) -> Result<bool> {
        let params = vec![
            RequestParameter::new("chat_id", serde_json::to_value(&chat_id)?),
            RequestParameter::new("user_id", serde_json::to_value(user_id)?),
        ];
        self.do_post("declineChatJoinRequest", params).await
    }
}
