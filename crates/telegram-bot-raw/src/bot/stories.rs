use super::{push_opt, push_opt_str, Bot, Result};
use crate::request::request_parameter::RequestParameter;
use crate::types::{message_entity, story};

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Stories
    // ======================================================================

    /// Use this method to post a story on behalf of a managed business account.
    ///
    /// Calls the Telegram `postStory` API method.
    pub async fn post_story_raw(
        &self,
        business_connection_id: &str,
        content: serde_json::Value,
        active_period: i64,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        areas: Option<Vec<serde_json::Value>>,
        post_to_chat_page: Option<bool>,
        protect_content: Option<bool>,
    ) -> Result<story::Story> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("content", content),
            RequestParameter::new("active_period", serde_json::to_value(active_period)?),
        ];
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(&mut params, "areas", &areas)?;
        push_opt(&mut params, "post_to_chat_page", &post_to_chat_page)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        self.do_post("postStory", params).await
    }

    /// Use this method to edit a story posted on behalf of a managed business account.
    ///
    /// Calls the Telegram `editStory` API method.
    pub async fn edit_story_raw(
        &self,
        business_connection_id: &str,
        story_id: i64,
        content: serde_json::Value,
        caption: Option<&str>,
        parse_mode: Option<&str>,
        caption_entities: Option<Vec<message_entity::MessageEntity>>,
        areas: Option<Vec<serde_json::Value>>,
    ) -> Result<story::Story> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("story_id", serde_json::to_value(story_id)?),
            RequestParameter::new("content", content),
        ];
        push_opt_str(&mut params, "caption", caption);
        push_opt_str(&mut params, "parse_mode", parse_mode);
        push_opt(&mut params, "caption_entities", &caption_entities)?;
        push_opt(&mut params, "areas", &areas)?;
        self.do_post("editStory", params).await
    }

    /// Use this method to delete a story posted on behalf of a managed business account.
    ///
    /// Calls the Telegram `deleteStory` API method.
    pub async fn delete_story_raw(
        &self,
        business_connection_id: &str,
        story_id: i64,
    ) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("story_id", serde_json::to_value(story_id)?),
        ];
        self.do_post("deleteStory", params).await
    }

    /// Use this method to repost a story on behalf of a managed business account.
    ///
    /// Calls the Telegram `repostStory` API method.
    pub async fn repost_story_raw(
        &self,
        business_connection_id: &str,
        from_chat_id: i64,
        from_story_id: i64,
        active_period: i64,
        post_to_chat_page: Option<bool>,
        protect_content: Option<bool>,
    ) -> Result<story::Story> {
        let mut params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("from_chat_id", serde_json::to_value(from_chat_id)?),
            RequestParameter::new("from_story_id", serde_json::to_value(from_story_id)?),
            RequestParameter::new("active_period", serde_json::to_value(active_period)?),
        ];
        push_opt(&mut params, "post_to_chat_page", &post_to_chat_page)?;
        push_opt(&mut params, "protect_content", &protect_content)?;
        self.do_post("repostStory", params).await
    }
}
