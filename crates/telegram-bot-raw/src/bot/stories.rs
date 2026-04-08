use super::*;

#[allow(dead_code)]
impl Bot {
    // ======================================================================
    // Stories
    // ======================================================================

    pub async fn post_story(
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

    pub async fn edit_story(
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

    pub async fn delete_story(&self, business_connection_id: &str, story_id: i64) -> Result<bool> {
        let params = vec![
            RequestParameter::new(
                "business_connection_id",
                serde_json::Value::String(business_connection_id.to_owned()),
            ),
            RequestParameter::new("story_id", serde_json::to_value(story_id)?),
        ];
        self.do_post("deleteStory", params).await
    }

    pub async fn repost_story(
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
