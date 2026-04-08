# Custom Filters

Filters decide whether a handler should run for a given update. The crate ships filters for common cases (commands, text, media types), but you can write your own for any matching logic.

## The Filter Trait

Every filter implements the `Filter` trait:

```rust
pub trait Filter: Send + Sync + 'static {
    fn check_update(&self, update: &Update) -> FilterResult;

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}
```

Implement it on any type:

```rust
use telegram_bot::ext::prelude::{Filter, FilterResult, Update};

pub struct TextLengthFilter {
    min: usize,
    max: usize,
}

impl TextLengthFilter {
    pub fn new(min: usize, max: usize) -> Self {
        Self { min, max }
    }
}

impl Filter for TextLengthFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let text = update
            .effective_message()
            .and_then(|m| m.text.as_deref());

        match text {
            Some(t) if t.len() >= self.min && t.len() <= self.max => {
                FilterResult::Match
            }
            _ => FilterResult::NoMatch,
        }
    }

    fn name(&self) -> &str {
        "TextLengthFilter"
    }
}
```

## FilterResult

`FilterResult` has three variants:

| Variant | Meaning |
|---|---|
| `FilterResult::NoMatch` | The filter did not match -- handler is skipped |
| `FilterResult::Match` | The filter matched -- handler runs |
| `FilterResult::MatchWithData(HashMap<String, Vec<String>>)` | Matched and carries extracted data |

Filters must not perform I/O or fail. They receive a `&Update` reference and return a pure result.

### MatchWithData

Data filters pass extracted information directly to the handler, avoiding redundant parsing. For example, a regex filter uses this to pass capture groups:

```rust
use std::collections::HashMap;
use telegram_bot::ext::prelude::{Filter, FilterResult, Update};

pub struct HashtagFilter;

impl Filter for HashtagFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let text = update
            .effective_message()
            .and_then(|m| m.text.as_deref())
            .unwrap_or("");

        let tags: Vec<String> = text
            .split_whitespace()
            .filter(|w| w.starts_with('#'))
            .map(|w| w.to_string())
            .collect();

        if tags.is_empty() {
            FilterResult::NoMatch
        } else {
            let mut data = HashMap::new();
            data.insert("hashtags".to_string(), tags);
            FilterResult::MatchWithData(data)
        }
    }

    fn name(&self) -> &str {
        "HashtagFilter"
    }
}
```

The handler can access this data through `context.matches`.

## Composing Filters with Operators

The `F` wrapper provides bitwise operators for combining filters:

| Operator | Meaning | Example |
|---|---|---|
| `&` | AND -- both must match | `TEXT() & !COMMAND()` |
| `\|` | OR -- either can match | `PHOTO \| VIDEO` |
| `^` | XOR -- exactly one must match | `filter_a ^ filter_b` |
| `!` | NOT -- inverts the match | `!COMMAND()` |

### Using F to Wrap Custom Filters

Wrap your custom filter in `F` to use the operators:

```rust
use telegram_bot::ext::prelude::{F, MessageHandler, TEXT, COMMAND};

let length_filter = F::new(TextLengthFilter::new(1, 100));
let combined = TEXT() & !COMMAND() & length_filter;

app.add_typed_handler(
    MessageHandler::new(combined, my_handler), 0,
).await;
```

### How Composition Works Internally

- `AndFilter`: checks left first. If it returns `NoMatch`, short-circuits. Otherwise checks right and merges data from both results.
- `OrFilter`: checks left first. If it matches, returns immediately. Otherwise checks right.
- `NotFilter`: inverts `Match` to `NoMatch` and vice versa. Data is lost on inversion.
- `XorFilter`: matches only when exactly one side matches.

## FnFilter for Closures

For one-off filters that do not warrant their own struct, use `FnFilter`:

```rust
use telegram_bot::ext::prelude::{F, Filter, FilterResult};
use telegram_bot::ext::prelude::Update;

// Import FnFilter from the filters module
use telegram_bot::ext::filters::base::FnFilter;

let admin_only = FnFilter::new("admin_only", |update: &Update| {
    update
        .effective_user()
        .map(|u| u.id == 123456789)
        .unwrap_or(false)
});

let filter = F::new(admin_only);
```

`FnFilter::new` takes a label (used for debug output) and a closure `Fn(&Update) -> bool`. The closure is wrapped so `true` becomes `FilterResult::Match` and `false` becomes `FilterResult::NoMatch`.

## Stateful Filters

Filters can hold state. Since they must be `Send + Sync`, use thread-safe wrappers:

```rust
use std::collections::HashSet;
use std::sync::Mutex;
use telegram_bot::ext::prelude::{Filter, FilterResult, Update};

pub struct AllowlistFilter {
    allowed: Mutex<HashSet<i64>>,
}

impl AllowlistFilter {
    pub fn new(initial: impl IntoIterator<Item = i64>) -> Self {
        Self {
            allowed: Mutex::new(initial.into_iter().collect()),
        }
    }

    pub fn add(&self, user_id: i64) {
        self.allowed.lock().unwrap().insert(user_id);
    }

    pub fn remove(&self, user_id: i64) {
        self.allowed.lock().unwrap().remove(&user_id);
    }
}

impl Filter for AllowlistFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let user_id = update.effective_user().map(|u| u.id);
        match user_id {
            Some(id) if self.allowed.lock().unwrap().contains(&id) => {
                FilterResult::Match
            }
            _ => FilterResult::NoMatch,
        }
    }

    fn name(&self) -> &str {
        "AllowlistFilter"
    }
}
```

Use `std::sync::Mutex` (not `tokio::sync::Mutex`) because `check_update` is synchronous and filters must not be async.

## Built-in Filters

The crate ships a comprehensive set of filters. Here are the most common ones:

```rust
use telegram_bot::ext::prelude::{TEXT, COMMAND, F};
use telegram_bot::ext::filters::base::{ALL, PHOTO, VIDEO, AUDIO, VOICE, LOCATION, CONTACT};
use telegram_bot::ext::filters::chat::{ChatTypePrivate, ChatTypeGroup};
use telegram_bot::ext::filters::text::{CAPTION, TextFilter, CaptionFilter};
use telegram_bot::ext::filters::regex::RegexFilter;
use telegram_bot::ext::filters::user::UserFilter;

// Text and commands
TEXT()                          // Any text message
COMMAND()                       // Any bot command
F::new(ALL)                     // Any message

// Media types
F::new(PHOTO)                   // Photo messages
F::new(VIDEO)                   // Video messages
F::new(AUDIO)                   // Audio files
F::new(VOICE)                   // Voice messages

// Chat types
F::new(ChatTypePrivate)         // Private chats only

// Regex matching
F::new(RegexFilter::new(r"^\d+$").unwrap())  // Matches digits only

// Specific users
F::new(UserFilter::new(vec![123456789]))      // Specific user IDs
```

## Next Steps

- [Error Handling](error-handling.md) -- what happens when a filtered handler's async body fails.
- [Testing](testing.md) -- unit-test your custom filters without a live bot connection.
