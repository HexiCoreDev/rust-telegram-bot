//! Entity and CaptionEntity filters.

use crate::filters::base::{Filter, FilterResult, Update};

// ---------------------------------------------------------------------------
// Entity
// ---------------------------------------------------------------------------

pub struct EntityFilter {
    entity_type: String,
    display: String,
}

impl EntityFilter {
    pub fn new(entity_type: impl Into<String>) -> Self {
        let et: String = entity_type.into();
        let display = format!("filters.Entity({})", et);
        Self {
            entity_type: et,
            display,
        }
    }

    pub fn entity_type(&self) -> &str {
        &self.entity_type
    }
}

impl Filter for EntityFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let msg = match update.effective_message() {
            Some(m) => m,
            None => return FilterResult::NoMatch,
        };
        let entities = match msg.entities.as_ref() {
            Some(arr) => arr,
            None => return FilterResult::NoMatch,
        };
        if entities
            .iter()
            .any(|e| e.entity_type == self.entity_type)
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        &self.display
    }
}

// ---------------------------------------------------------------------------
// CaptionEntity
// ---------------------------------------------------------------------------

pub struct CaptionEntityFilter {
    entity_type: String,
    display: String,
}

impl CaptionEntityFilter {
    pub fn new(entity_type: impl Into<String>) -> Self {
        let et: String = entity_type.into();
        let display = format!("filters.CaptionEntity({})", et);
        Self {
            entity_type: et,
            display,
        }
    }
}

impl Filter for CaptionEntityFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let msg = match update.effective_message() {
            Some(m) => m,
            None => return FilterResult::NoMatch,
        };
        let entities = match msg.caption_entities.as_ref() {
            Some(arr) => arr,
            None => return FilterResult::NoMatch,
        };
        if entities
            .iter()
            .any(|e| e.entity_type == self.entity_type)
        {
            FilterResult::Match
        } else {
            FilterResult::NoMatch
        }
    }

    fn name(&self) -> &str {
        &self.display
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn entity_filter_matches() {
        let f = EntityFilter::new("hashtag");
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "text": "#rust",
                "entities": [{"type": "hashtag", "offset": 0, "length": 5}]
            }
        }))
        .unwrap();
        assert!(f.check_update(&update).is_match());
    }

    #[test]
    fn entity_filter_wrong_type() {
        let f = EntityFilter::new("url");
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "text": "#rust",
                "entities": [{"type": "hashtag", "offset": 0, "length": 5}]
            }
        }))
        .unwrap();
        assert!(!f.check_update(&update).is_match());
    }

    #[test]
    fn entity_filter_no_entities() {
        let f = EntityFilter::new("mention");
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "text": "hello"
            }
        }))
        .unwrap();
        assert!(!f.check_update(&update).is_match());
    }

    #[test]
    fn caption_entity_filter() {
        let f = CaptionEntityFilter::new("bold");
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "caption": "Look at **this**",
                "caption_entities": [{"type": "bold", "offset": 8, "length": 6}]
            }
        }))
        .unwrap();
        assert!(f.check_update(&update).is_match());
    }

    #[test]
    fn caption_entity_no_match() {
        let f = CaptionEntityFilter::new("italic");
        let update: Update = serde_json::from_value(json!({
            "update_id": 1,
            "message": {
                "message_id": 1, "date": 0,
                "chat": {"id": 1, "type": "private"},
                "caption": "plain text",
                "caption_entities": [{"type": "bold", "offset": 0, "length": 5}]
            }
        }))
        .unwrap();
        assert!(!f.check_update(&update).is_match());
    }
}
