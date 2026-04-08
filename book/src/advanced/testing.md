# Testing

Testing your bot ensures that handlers behave correctly, filters match the right updates, and persistence stores data as expected. This chapter covers strategies from unit testing individual components to integration testing with real persistence backends.

## Unit Testing Filters

Filters are pure functions of `&Update -> FilterResult`. Test them without a running bot by constructing `Update` values from JSON:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use telegram_bot::ext::prelude::{Filter, FilterResult, Update};

    fn make_update(json_val: serde_json::Value) -> Update {
        serde_json::from_value(json_val).unwrap()
    }

    fn text_message_update(text: &str) -> Update {
        make_update(json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 0,
                "chat": { "id": 1, "type": "private" },
                "from": { "id": 1, "is_bot": false, "first_name": "Test" },
                "text": text
            }
        }))
    }

    #[test]
    fn text_length_filter_accepts_valid() {
        let filter = TextLengthFilter::new(1, 100);
        let update = text_message_update("Hello!");
        assert!(filter.check_update(&update).is_match());
    }

    #[test]
    fn text_length_filter_rejects_empty() {
        let filter = TextLengthFilter::new(1, 100);
        let update = text_message_update("");
        assert!(!filter.check_update(&update).is_match());
    }

    #[test]
    fn text_length_filter_rejects_too_long() {
        let filter = TextLengthFilter::new(1, 5);
        let update = text_message_update("This is too long");
        assert!(!filter.check_update(&update).is_match());
    }
}
```

### Testing Built-in Filters

The same approach works for built-in filters:

```rust
#[cfg(test)]
mod tests {
    use telegram_bot::ext::prelude::{TEXT, COMMAND};

    #[test]
    fn text_filter_matches_text_messages() {
        let update = text_message_update("hello");
        assert!(TEXT().0.check_update(&update).is_match());
    }

    #[test]
    fn command_filter_matches_commands() {
        let update = make_update(json!({
            "update_id": 1,
            "message": {
                "message_id": 1,
                "date": 0,
                "chat": { "id": 1, "type": "private" },
                "from": { "id": 1, "is_bot": false, "first_name": "Test" },
                "text": "/start",
                "entities": [{
                    "type": "bot_command",
                    "offset": 0,
                    "length": 6
                }]
            }
        }));
        assert!(COMMAND().0.check_update(&update).is_match());
    }
}
```

### Testing Filter Composition

Verify that combined filters behave correctly:

```rust
#[test]
fn text_and_not_command() {
    let filter = TEXT() & !COMMAND();

    // Plain text should match
    let text_update = text_message_update("hello");
    assert!(filter.0.check_update(&text_update).is_match());

    // Command should not match
    let cmd_update = make_update(json!({
        "update_id": 1,
        "message": {
            "message_id": 1,
            "date": 0,
            "chat": { "id": 1, "type": "private" },
            "from": { "id": 1, "is_bot": false, "first_name": "Test" },
            "text": "/start",
            "entities": [{ "type": "bot_command", "offset": 0, "length": 6 }]
        }
    }));
    assert!(!filter.0.check_update(&cmd_update).is_match());
}
```

## Testing Handler Logic

Handlers contain your business logic. Test the logic by extracting it into pure functions:

```rust
// In your bot code:
fn format_greeting(first_name: &str, visit_count: i64) -> String {
    if visit_count == 0 {
        format!("Welcome, {first_name}!")
    } else {
        format!("Welcome back, {first_name}! Visit #{}", visit_count + 1)
    }
}

// In your tests:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_first_visit() {
        assert_eq!(format_greeting("Alice", 0), "Welcome, Alice!");
    }

    #[test]
    fn greeting_return_visit() {
        assert_eq!(
            format_greeting("Bob", 4),
            "Welcome back, Bob! Visit #5"
        );
    }
}
```

This pattern -- extract logic into testable functions, call them from handlers -- is the most reliable way to test bot behaviour without mocking the Telegram API.

## Testing Persistence

### In-Memory SQLite

Use `SqlitePersistence::in_memory()` to test persistence logic without touching the filesystem:

```rust
#[cfg(test)]
mod tests {
    use telegram_bot::ext::persistence::sqlite::SqlitePersistence;
    use telegram_bot::ext::persistence::base::BasePersistence;
    use std::collections::HashMap;

    #[tokio::test]
    async fn sqlite_round_trip() {
        let persistence = SqlitePersistence::in_memory()
            .expect("failed to create in-memory SQLite");

        // Write user data
        let mut data = HashMap::new();
        data.insert("name".to_string(), serde_json::json!("Alice"));
        persistence.update_user_data(42, &data).await.unwrap();

        // Read it back
        let all_users = persistence.get_user_data().await.unwrap();
        let user_42 = all_users.get(&42).unwrap();
        assert_eq!(
            user_42.get("name").and_then(|v| v.as_str()),
            Some("Alice"),
        );
    }
}
```

### Temporary JSON Files

For `JsonFilePersistence`, use a temporary directory:

```rust
#[cfg(test)]
mod tests {
    use telegram_bot::ext::persistence::json_file::JsonFilePersistence;
    use telegram_bot::ext::persistence::base::BasePersistence;

    #[tokio::test]
    async fn json_file_round_trip() {
        let dir = tempfile::tempdir().unwrap();
        let prefix = dir
            .path()
            .join("test_bot")
            .to_string_lossy()
            .to_string();

        let persistence = JsonFilePersistence::new(&prefix, true, false);

        let mut data = std::collections::HashMap::new();
        data.insert("key".to_string(), serde_json::json!("value"));
        persistence.update_bot_data(&data).await.unwrap();
        persistence.flush().await.unwrap();

        let loaded = persistence.get_bot_data().await.unwrap();
        assert_eq!(
            loaded.get("key").and_then(|v| v.as_str()),
            Some("value"),
        );
    }
}
```

## Testing Conversation State

Test conversation state transitions by directly manipulating the state store:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn state_transitions() {
        let store: StateStore = Arc::new(RwLock::new(HashMap::new()));

        // Simulate /start
        {
            let mut s = store.write().await;
            s.insert(1, UserState {
                conv: ConvState::Top(TopState::SelectingAction),
                ..Default::default()
            });
        }

        // Verify state
        {
            let s = store.read().await;
            assert_eq!(
                s.get(&1).unwrap().conv,
                ConvState::Top(TopState::SelectingAction),
            );
        }

        // Simulate transition to member level
        {
            let mut s = store.write().await;
            let us = s.get_mut(&1).unwrap();
            us.conv = ConvState::Member(MemberState::SelectingLevel);
        }

        // Verify transition
        {
            let s = store.read().await;
            assert_eq!(
                s.get(&1).unwrap().conv,
                ConvState::Member(MemberState::SelectingLevel),
            );
        }
    }
}
```

## Integration Testing Patterns

For full integration tests that exercise the entire handler pipeline, you have two options.

### 1. Build the Application but Do Not Run It

Create the `Application`, register handlers, but do not call `run_polling()`. You can verify handler registration and filter matching:

```rust
#[tokio::test]
async fn integration_test_setup() {
    let token = "fake-token-for-testing";
    let app = ApplicationBuilder::new().token(token).build();

    app.add_typed_handler(
        MessageHandler::new(TEXT() & !COMMAND(), echo), 0,
    ).await;

    // The app is configured but not connected to Telegram.
    // Verify handler registration, filter composition, etc.
}
```

### 2. Test with Fake Updates

Feed hand-crafted updates through the update sender to test the full pipeline:

```rust
#[tokio::test]
async fn test_update_processing() {
    let token = "fake-token-for-testing";
    let app = ApplicationBuilder::new().token(token).build();

    // Register handlers...

    app.initialize().await.unwrap();
    app.start().await.unwrap();

    // Send a fake update through the channel
    let raw_update = serde_json::from_value(json!({
        "update_id": 1,
        "message": {
            "message_id": 1,
            "date": 0,
            "chat": { "id": 1, "type": "private" },
            "from": { "id": 1, "is_bot": false, "first_name": "Test" },
            "text": "hello"
        }
    })).unwrap();

    app.update_sender().send(raw_update).await.unwrap();

    // Allow time for processing
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    app.stop().await.ok();
}
```

Note: API calls will fail without a real token, but this tests the dispatch pipeline.

## Test Organisation

A recommended project layout for tests:

```
crates/
  my-bot/
    src/
      main.rs
      handlers.rs       # Handler functions
      filters.rs        # Custom filters
      logic.rs          # Pure business logic
    tests/
      filters_test.rs   # Unit tests for filters
      logic_test.rs     # Unit tests for business logic
      persistence_test.rs  # Integration tests for persistence
```

Keep handler functions thin -- they should extract data from the update, call business logic functions, and send responses. Test the business logic functions directly.

## Tips

- **Use `serde_json::json!` to construct updates.** It is the fastest way to create test fixtures.
- **Test filters independently.** Filters are the easiest part of the bot to test thoroughly.
- **Test state machines with direct store manipulation.** You do not need a running bot to verify state transitions.
- **Use `#[tokio::test]` for async tests.** The persistence backends require an async runtime.
- **Use `tempfile` for filesystem tests.** It automatically cleans up temporary directories.
- **Keep business logic pure.** Functions that take plain values and return plain values are trivial to test.

## Next Steps

- [Deployment](deployment.md) -- deploy your tested bot to production.
- [Custom Filters](custom-filters.md) -- write filters that are easy to test in isolation.
