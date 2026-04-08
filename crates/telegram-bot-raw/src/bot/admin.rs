use super::{push_opt, push_opt_str, Bot, Result};
use crate::request::request_parameter::RequestParameter;
use crate::types::{
    bot_command, bot_command_scope, bot_description, bot_name, chat_administrator_rights,
    menu_button,
};

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Chat menu & commands
    // ======================================================================

    pub async fn set_chat_menu_button(
        &self,
        chat_id: Option<i64>,
        menu_button: Option<menu_button::MenuButton>,
    ) -> Result<bool> {
        let mut params = Vec::new();
        push_opt(&mut params, "chat_id", &chat_id)?;
        push_opt(&mut params, "menu_button", &menu_button)?;
        self.do_post("setChatMenuButton", params).await
    }

    pub async fn get_chat_menu_button(
        &self,
        chat_id: Option<i64>,
    ) -> Result<menu_button::MenuButton> {
        let mut params = Vec::new();
        push_opt(&mut params, "chat_id", &chat_id)?;
        self.do_post("getChatMenuButton", params).await
    }

    pub async fn set_my_commands(
        &self,
        commands: Vec<bot_command::BotCommand>,
        scope: Option<bot_command_scope::BotCommandScope>,
        language_code: Option<&str>,
    ) -> Result<bool> {
        let mut params = vec![RequestParameter::new(
            "commands",
            serde_json::to_value(&commands)?,
        )];
        push_opt(&mut params, "scope", &scope)?;
        push_opt_str(&mut params, "language_code", language_code);
        self.do_post("setMyCommands", params).await
    }

    pub async fn get_my_commands(
        &self,
        scope: Option<bot_command_scope::BotCommandScope>,
        language_code: Option<&str>,
    ) -> Result<Vec<bot_command::BotCommand>> {
        let mut params = Vec::new();
        push_opt(&mut params, "scope", &scope)?;
        push_opt_str(&mut params, "language_code", language_code);
        self.do_post("getMyCommands", params).await
    }

    pub async fn delete_my_commands(
        &self,
        scope: Option<bot_command_scope::BotCommandScope>,
        language_code: Option<&str>,
    ) -> Result<bool> {
        let mut params = Vec::new();
        push_opt(&mut params, "scope", &scope)?;
        push_opt_str(&mut params, "language_code", language_code);
        self.do_post("deleteMyCommands", params).await
    }

    pub async fn set_my_default_administrator_rights(
        &self,
        rights: Option<chat_administrator_rights::ChatAdministratorRights>,
        for_channels: Option<bool>,
    ) -> Result<bool> {
        let mut params = Vec::new();
        push_opt(&mut params, "rights", &rights)?;
        push_opt(&mut params, "for_channels", &for_channels)?;
        self.do_post("setMyDefaultAdministratorRights", params)
            .await
    }

    pub async fn get_my_default_administrator_rights(
        &self,
        for_channels: Option<bool>,
    ) -> Result<chat_administrator_rights::ChatAdministratorRights> {
        let mut params = Vec::new();
        push_opt(&mut params, "for_channels", &for_channels)?;
        self.do_post("getMyDefaultAdministratorRights", params)
            .await
    }

    // ======================================================================
    // Bot description and name
    // ======================================================================

    pub async fn set_my_description(
        &self,
        description: Option<&str>,
        language_code: Option<&str>,
    ) -> Result<bool> {
        let mut params = Vec::new();
        push_opt_str(&mut params, "description", description);
        push_opt_str(&mut params, "language_code", language_code);
        self.do_post("setMyDescription", params).await
    }

    pub async fn get_my_description(
        &self,
        language_code: Option<&str>,
    ) -> Result<bot_description::BotDescription> {
        let mut params = Vec::new();
        push_opt_str(&mut params, "language_code", language_code);
        self.do_post("getMyDescription", params).await
    }

    pub async fn set_my_short_description(
        &self,
        short_description: Option<&str>,
        language_code: Option<&str>,
    ) -> Result<bool> {
        let mut params = Vec::new();
        push_opt_str(&mut params, "short_description", short_description);
        push_opt_str(&mut params, "language_code", language_code);
        self.do_post("setMyShortDescription", params).await
    }

    pub async fn get_my_short_description(
        &self,
        language_code: Option<&str>,
    ) -> Result<bot_description::BotShortDescription> {
        let mut params = Vec::new();
        push_opt_str(&mut params, "language_code", language_code);
        self.do_post("getMyShortDescription", params).await
    }

    pub async fn set_my_name(
        &self,
        name: Option<&str>,
        language_code: Option<&str>,
    ) -> Result<bool> {
        let mut params = Vec::new();
        push_opt_str(&mut params, "name", name);
        push_opt_str(&mut params, "language_code", language_code);
        self.do_post("setMyName", params).await
    }

    pub async fn get_my_name(&self, language_code: Option<&str>) -> Result<bot_name::BotName> {
        let mut params = Vec::new();
        push_opt_str(&mut params, "language_code", language_code);
        self.do_post("getMyName", params).await
    }
}
