# Filters

Filters determine whether a `MessageHandler` (or other filter-accepting handler) fires
for a given update. They compose with `&`, `|`, `^`, and `!` -- the same mental model as
python-telegram-bot, but enforced by the Rust compiler.

---

## Quick Start

The prelude provides `TEXT()` and `COMMAND()` as ready-to-compose filter functions:

```rust
use telegram_bot::ext::prelude::*;

// Text messages that are NOT commands
let text_only = TEXT() & !COMMAND();

// Use it directly in a handler
app.add_typed_handler(
    MessageHandler::new(TEXT() & !COMMAND(), echo),
    0,
).await;
```

`TEXT()` and `COMMAND()` return `F` values (the composable filter wrapper), so you can
use `&`, `|`, `^`, and `!` operators directly without additional wrapping.

---

## The Filter Trait

```rust
pub trait Filter: Send + Sync + 'static {
    fn check_update(&self, update: &Update) -> FilterResult;
    fn name(&self) -> &str { std::any::type_name::<Self>() }
}
```

---

## FilterResult

```rust
pub enum FilterResult {
    NoMatch,
    Match,
    MatchWithData(HashMap<String, Vec<String>>),
}

impl FilterResult {
    pub fn is_match(&self) -> bool { ... }
    pub fn merge(self, other: FilterResult) -> FilterResult { ... }
}
```

`MatchWithData` is returned by data filters (regex filters, caption-regex, etc.). The
map keys are category names (e.g. `"matches"`), the values are lists of captured strings.

`merge` combines data from two filters when both must match (`AndFilter`). If either is
`NoMatch`, the result is `NoMatch`. If both carry data, their maps are merged with values
appended for duplicate keys.

---

## The F Wrapper

`F` is an ergonomic wrapper around `Box<dyn Filter>` that provides operator overloads:

```rust
pub struct F(pub Box<dyn Filter>);

impl F {
    pub fn new(filter: impl Filter) -> Self { ... }
}
```

Wrap any concrete filter to get composition operators:

```rust
use telegram_bot::ext::filters::base::F;
use telegram_bot::ext::filters::text::TextAny;
use telegram_bot::ext::filters::command::CommandFilter;

// Manual wrapping (when not using prelude helpers)
let f = F::new(TextAny) & !F::new(CommandFilter::starts());
```

In practice, use the prelude functions `TEXT()` and `COMMAND()` which return pre-wrapped
`F` values:

```rust
use telegram_bot::ext::prelude::*;

let f = TEXT() & !COMMAND();
```

Operators return a new `F`, so you can chain arbitrarily:

```rust
let f = TEXT() & !COMMAND();
// Additional composition with custom filters
let f = f & F::new(my_custom_filter);
```

---

## Composition Operators

| Operator | Rust syntax | Behaviour |
|----------|-------------|-----------|
| AND | `f1 & f2` | Both must match. Data maps are merged. |
| OR | `f1 \| f2` | First match wins. Returns that result (with any data). |
| XOR | `f1 ^ f2` | Exactly one must match. Returns the matching result. |
| NOT | `!f` | Inverts the result. `MatchWithData` becomes `NoMatch`. |

Short-circuit evaluation applies: for `AND`, if the left side is `NoMatch`, the right
side is never evaluated. For `OR`, if the left side matches, the right side is skipped.

---

## Built-in Constant Filters

These are zero-size structs exposed as public constants. No allocation required.

### Media / Attachment Filters

| Constant | What it matches |
|----------|-----------------|
| `ANIMATION` | Messages with an animation (GIF) |
| `AUDIO` | Messages with audio |
| `DOCUMENT` | Messages with a document |
| `PHOTO` | Messages with one or more photos |
| `STICKER` | Messages with a sticker |
| `VIDEO` | Messages with a video |
| `VIDEO_NOTE` | Messages with a video note (round video) |
| `VOICE` | Messages with a voice note |
| `PAID_MEDIA` | Messages with paid media |
| `ATTACHMENT` | Any message with an effective attachment (computed) |

Import from `telegram_bot_ext::filters::base` or `telegram_bot_ext::filters::photo`.

### Content Type Filters

| Constant | What it matches |
|----------|-----------------|
| `TEXT` | Messages with a `text` field |
| `CAPTION` | Messages with a `caption` field |
| `COMMAND` | Messages where first entity is a `bot_command` at offset 0 |
| `CONTACT` | Messages containing a contact |
| `LOCATION` | Messages containing a location |
| `VENUE` | Messages containing a venue |
| `GAME` | Messages containing a game |
| `INVOICE` | Messages containing an invoice |
| `POLL` | Messages containing a poll |
| `STORY` | Messages containing a story |
| `ANIMATION` | Messages containing a GIF/animation |

### Status and Meta Filters

| Constant | What it matches |
|----------|-----------------|
| `FORWARDED` | Messages with a `forward_origin` |
| `REPLY` | Messages that are replies (`reply_to_message` present) |
| `REPLY_TO_STORY` | Messages that reply to a story |
| `IS_AUTOMATIC_FORWARD` | Automatically forwarded channel posts |
| `IS_TOPIC_MESSAGE` | Forum topic messages |
| `IS_FROM_OFFLINE` | Messages sent while offline |
| `HAS_MEDIA_SPOILER` | Media with a spoiler overlay |
| `HAS_PROTECTED_CONTENT` | Messages with protected content |
| `BOOST_ADDED` | Boost added notifications |
| `GIVEAWAY` | Giveaway messages |
| `GIVEAWAY_WINNERS` | Giveaway winner announcements |
| `CHECKLIST` | Checklist messages |
| `EFFECT_ID` | Messages with an effect |
| `PASSPORT_DATA` | Messages with Telegram Passport data |

### User and Chat Filters

| Constant | What it matches |
|----------|-----------------|
| `USER` | Messages with a `from` field (sent by a user) |
| `PREMIUM_USER` | Messages from Telegram Premium users |
| `USER_ATTACHMENT` | Users who added the bot to their attachment menu |
| `FORUM` | Messages in forum (topics-enabled) chats |
| `DIRECT_MESSAGES` | Channel direct messages |
| `VIA_BOT` | Messages sent via a bot |
| `ALL` | Any update that has an effective message |
| `SENDER_BOOST_COUNT` | Messages with a sender boost count |

---

## Parameterized Filters

### Command filter (configurable)

```rust
use telegram_bot::ext::filters::command::CommandFilter;

// Default: command must be at offset 0 (same as COMMAND() prelude helper)
let f = F::new(CommandFilter::starts());

// Also match commands that appear mid-text
let f = F::new(CommandFilter::anywhere());
```

### Text filters

```rust
use telegram_bot::ext::filters::text::{TextFilter, CaptionFilter, CaptionRegexFilter,
                                       LanguageFilter, SuccessfulPaymentFilter, DiceFilter};

// Match messages whose text is exactly "Yes" or "No"
let f = F::new(TextFilter::new(["Yes", "No"]));

// Match messages with captions
let f = F::new(CaptionFilter::new(["Help me!"]));

// Regex search on caption (returns MatchWithData)
let f = F::new(CaptionRegexFilter::new(r"order #(\d+)"));

// Filter by sender language code (prefix match: "en" matches "en_US")
let f = F::new(LanguageFilter::new(["en", "de"]));

// Successful payment filter
let f = F::new(SuccessfulPaymentFilter::any());
let f = F::new(SuccessfulPaymentFilter::with_payloads(["premium_monthly"]));
```

### Dice filters

```rust
use telegram_bot::ext::filters::text::{DiceFilter, dice_emoji};

let f = F::new(DiceFilter::all());                                     // any dice
let f = F::new(DiceFilter::with_values([6]));                          // any emoji, value 6
let f = F::new(DiceFilter::with_emoji(dice_emoji::DARTS));             // darts, any value
let f = F::new(DiceFilter::with_emoji_values(dice_emoji::DICE, [3])); // dice cube, value 3

// Shorthand constructors
let f = F::new(DiceFilter::basketball(Some(vec![5, 6])));  // top scores only
let f = F::new(DiceFilter::slot_machine(None));             // any slot machine result
```

Available emoji constants: `dice_emoji::BASKETBALL`, `BOWLING`, `DARTS`, `DICE`,
`FOOTBALL`, `SLOT_MACHINE`.

### Mention filter

```rust
use telegram_bot::ext::filters::text::MentionFilter;

let f = F::new(MentionFilter::from_ids([42, 99]));              // by user ID
let f = F::new(MentionFilter::from_usernames(["alice", "bob"])); // by @username (@ stripped)
let f = F::new(MentionFilter::new([42], ["alice"]));             // combined
```

### Regex filter (message text)

```rust
use telegram_bot::ext::filters::regex::RegexFilter;
use regex::Regex;

// Returns MatchWithData with captures under "matches"
let f = F::new(RegexFilter::new(Regex::new(r"order #(\d+)").unwrap()));
```

### Entity filter

```rust
use telegram_bot::ext::filters::entity::EntityFilter;

// Messages with any URL entity
let f = F::new(EntityFilter::new("url"));

// Messages with a bold entity
let f = F::new(EntityFilter::new("bold"));
```

### User filter

```rust
use telegram_bot::ext::filters::user::UserFilter;

// Only handle messages from specific users
let f = F::new(UserFilter::from_ids([42, 99]));
let f = F::new(UserFilter::from_usernames(["alice", "bob"]));
```

### Chat filter

```rust
use telegram_bot::ext::filters::chat::ChatFilter;

// Only handle messages from specific chats
let f = F::new(ChatFilter::from_ids([-100123456789]));
let f = F::new(ChatFilter::from_usernames(["mychannel"]));

// By chat type
let f = F::new(ChatFilter::private());
let f = F::new(ChatFilter::group());
let f = F::new(ChatFilter::supergroup());
let f = F::new(ChatFilter::channel());
```

### Document filter (by MIME type)

```rust
use telegram_bot::ext::filters::document::DocumentFilter;

// Match documents with a specific MIME type
let f = F::new(DocumentFilter::mime_type("application/pdf"));

// Match by file extension
let f = F::new(DocumentFilter::file_extension("csv"));
```

### StatusUpdate sub-filters

```rust
use telegram_bot::ext::filters::status_update::*;

let f = F::new(NEW_CHAT_MEMBERS);       // New chat members joined
let f = F::new(LEFT_CHAT_MEMBER);       // Members left
let f = F::new(NEW_CHAT_TITLE);         // Title changed
let f = F::new(NEW_CHAT_PHOTO);         // Chat photo changed
let f = F::new(DELETE_CHAT_PHOTO);       // Chat photo deleted
let f = F::new(GROUP_CHAT_CREATED);      // Group created
let f = F::new(SUPERGROUP_CHAT_CREATED); // Supergroup created
let f = F::new(CHANNEL_CHAT_CREATED);    // Channel created
let f = F::new(PINNED_MESSAGE);          // Message pinned
let f = F::new(STATUS_UPDATE);           // Any of the above
```

### Via-bot filter

```rust
use telegram_bot::ext::filters::via_bot::ViaBotFilter;

// Only messages sent via a specific bot username
let f = F::new(ViaBotFilter::from_username("inlinebot"));
```

### Forwarded filter

```rust
use telegram_bot::ext::filters::forwarded::ForwardedFilter;

// Forwarded from a user
let f = F::new(ForwardedFilter::from_user());

// Forwarded from a channel
let f = F::new(ForwardedFilter::from_channel());
```

---

## Creating Custom Filters

### Option 1: FnFilter (closure)

The simplest approach for one-off filters:

```rust
use telegram_bot::ext::filters::base::FnFilter;

let premium_private = FnFilter::new("premium_private", |update| {
    let msg = match &update.message {
        Some(m) => m,
        None => return false,
    };
    let is_premium = msg.from_user.as_ref().map_or(false, |u| u.is_premium.unwrap_or(false));
    let is_private = msg.chat.chat_type == ChatType::Private;
    is_premium && is_private
});

// Wrap and compose
let f = F::new(premium_private) & TEXT();
```

### Option 2: Implement the Filter trait

For reusable filters or ones that carry configuration:

```rust
use telegram_bot::ext::filters::base::{Filter, FilterResult};
use telegram_bot::raw::types::update::Update;

pub struct AdminFilter {
    admin_ids: Vec<i64>,
}

impl AdminFilter {
    pub fn new(admin_ids: Vec<i64>) -> Self {
        Self { admin_ids }
    }
}

impl Filter for AdminFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let user_id = update.effective_user().map(|u| u.id);
        match user_id {
            Some(id) if self.admin_ids.contains(&id) => FilterResult::Match,
            _ => FilterResult::NoMatch,
        }
    }

    fn name(&self) -> &str {
        "AdminFilter"
    }
}

// Use it
let f = F::new(AdminFilter::new(vec![12345678]));
```

### Option 3: Data-returning filter

For filters that extract data (like regex filters):

```rust
use std::collections::HashMap;
use telegram_bot::ext::filters::base::{Filter, FilterResult};
use telegram_bot::raw::types::update::Update;

pub struct OrderIdFilter;

impl Filter for OrderIdFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let text = match update.effective_message().and_then(|m| m.text.as_deref()) {
            Some(t) => t,
            None => return FilterResult::NoMatch,
        };
        if let Some(rest) = text.strip_prefix("order #") {
            if let Some(order_id) = rest.split_whitespace().next() {
                let mut data = HashMap::new();
                data.insert("order_id".to_owned(), vec![order_id.to_owned()]);
                return FilterResult::MatchWithData(data);
            }
        }
        FilterResult::NoMatch
    }

    fn name(&self) -> &str {
        "OrderIdFilter"
    }
}
```

---

## Migration from Python Filters

| Python | Rust |
|--------|------|
| `filters.TEXT` | `TEXT()` (from prelude) |
| `filters.COMMAND` | `COMMAND()` (from prelude) |
| `filters.PHOTO` | `F::new(PHOTO)` (from `filters::base`) |
| `filters.VIDEO` | `F::new(VIDEO)` (from `filters::base`) |
| `filters.AUDIO` | `F::new(AUDIO)` (from `filters::base`) |
| `filters.Document.FileExtension("csv")` | `F::new(DocumentFilter::file_extension("csv"))` |
| `filters.Regex(r"...")` | `F::new(RegexFilter::new(Regex::new(r"...").unwrap()))` |
| `filters.User(user_id=42)` | `F::new(UserFilter::from_ids([42]))` |
| `filters.Chat(chat_id=-100...)` | `F::new(ChatFilter::from_ids([-100...]))` |
| `filters.Text(["Yes", "No"])` | `F::new(TextFilter::new(["Yes", "No"]))` |
| `filters.FORWARDED` | `F::new(FORWARDED)` |
| `filters.PREMIUM_USER` | `F::new(PREMIUM_USER)` |
| `filters.ALL` | `F::new(ALL)` |
| `f1 & f2` | `f1 & f2` (both must be `F`) |
| `f1 \| f2` | `f1 \| f2` |
| `~f` | `!f` |

The key difference: in the prelude, `TEXT()` and `COMMAND()` return `F` directly so they
compose without wrapping. For other constant filters, wrap them in `F::new(...)` first.
