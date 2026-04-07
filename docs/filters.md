# Filters

Filters determine whether a `MessageHandler` (or other filter-accepting handler) fires
for a given update. They compose with `&`, `|`, `^`, and `!` -- the same mental model as
python-telegram-bot, but enforced by the Rust compiler.

---

## The Filter Trait

```rust
pub trait Filter: Send + Sync + 'static {
    fn check_update(&self, update: &Update) -> FilterResult;
    fn name(&self) -> &str { std::any::type_name::<Self>() }
}
```

`update` here is `serde_json::Value`, not the strongly-typed `Update` struct. This allows
filters to work without knowing the full type schema and makes them easy to test.

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
use telegram_bot_ext::filters::base::{F, All};
use telegram_bot_ext::filters::text::TEXT;
use telegram_bot_ext::filters::command::COMMAND;

// Text messages that are not commands
let f = F::new(TEXT) & !F::new(COMMAND);

// Photos or documents
let f = F::new(PHOTO) | F::new(DOCUMENT);

// Exactly one of two filters (XOR)
let f = F::new(PHOTO) ^ F::new(VIDEO);
```

Operators return a new `F`, so you can chain arbitrarily:

```rust
let f = F::new(TEXT) & !F::new(COMMAND) & F::new(PREMIUM_USER);
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
use telegram_bot_ext::filters::command::CommandFilter;

// Default: command must be at offset 0 (same as COMMAND constant)
let f = CommandFilter::starts();

// Also match commands that appear mid-text
let f = CommandFilter::anywhere();
```

### Text filters

```rust
use telegram_bot_ext::filters::text::{TextFilter, CaptionFilter, CaptionRegexFilter,
                                       LanguageFilter, SuccessfulPaymentFilter, DiceFilter};

// Match messages whose text is exactly "Yes" or "No"
let f = TextFilter::new(["Yes", "No"]);

// Match messages with captions
let f = CaptionFilter::new(["Help me!"]);

// Regex search on caption (returns MatchWithData)
let f = CaptionRegexFilter::new(r"order #(\d+)");

// Filter by sender language code (prefix match: "en" matches "en_US")
let f = LanguageFilter::new(["en", "de"]);

// Successful payment filter
let f = SuccessfulPaymentFilter::any();
let f = SuccessfulPaymentFilter::with_payloads(["premium_monthly"]);
```

### Dice filters

```rust
use telegram_bot_ext::filters::text::{DiceFilter, dice_emoji};

let f = DiceFilter::all();                                     // any dice
let f = DiceFilter::with_values([6]);                          // any emoji, value 6
let f = DiceFilter::with_emoji(dice_emoji::DARTS);             // darts, any value
let f = DiceFilter::with_emoji_values(dice_emoji::DICE, [3]);  // dice cube, value 3

// Shorthand constructors
let f = DiceFilter::basketball(Some(vec![5, 6]));  // top scores only
let f = DiceFilter::slot_machine(None);             // any slot machine result
```

Available emoji constants: `dice_emoji::BASKETBALL`, `BOWLING`, `DARTS`, `DICE`,
`FOOTBALL`, `SLOT_MACHINE`.

### Mention filter

```rust
use telegram_bot_ext::filters::text::MentionFilter;

let f = MentionFilter::from_ids([42, 99]);              // by user ID
let f = MentionFilter::from_usernames(["alice", "bob"]); // by @username (@ stripped)
let f = MentionFilter::new([42], ["alice"]);             // combined
```

### Regex filter (message text)

```rust
use telegram_bot_ext::filters::regex::RegexFilter;
use regex::Regex;

// Returns MatchWithData with captures under "matches"
let f = RegexFilter::new(Regex::new(r"order #(\d+)").unwrap());
```

### Entity filter

```rust
use telegram_bot_ext::filters::entity::EntityFilter;

// Messages with any URL entity
let f = EntityFilter::new("url");

// Messages with a bold entity
let f = EntityFilter::new("bold");
```

### User filter

```rust
use telegram_bot_ext::filters::user::UserFilter;

// Only handle messages from specific users
let f = UserFilter::from_ids([42, 99]);
let f = UserFilter::from_usernames(["alice", "bob"]);
```

### Chat filter

```rust
use telegram_bot_ext::filters::chat::ChatFilter;

// Only handle messages from specific chats
let f = ChatFilter::from_ids([-100123456789]);
let f = ChatFilter::from_usernames(["mychannel"]);

// By chat type
let f = ChatFilter::private();
let f = ChatFilter::group();
let f = ChatFilter::supergroup();
let f = ChatFilter::channel();
```

### Document filter (by MIME type)

```rust
use telegram_bot_ext::filters::document::DocumentFilter;

// Match documents with a specific MIME type
let f = DocumentFilter::mime_type("application/pdf");

// Match by file extension
let f = DocumentFilter::file_extension("csv");
```

### StatusUpdate sub-filters

```rust
use telegram_bot_ext::filters::status_update::*;

// New chat members joined
let f = NEW_CHAT_MEMBERS;

// Members left
let f = LEFT_CHAT_MEMBER;

// Title changed
let f = NEW_CHAT_TITLE;

// Chat photo changed
let f = NEW_CHAT_PHOTO;

// Chat photo deleted
let f = DELETE_CHAT_PHOTO;

// Group created
let f = GROUP_CHAT_CREATED;

// Supergroup created
let f = SUPERGROUP_CHAT_CREATED;

// Channel created
let f = CHANNEL_CHAT_CREATED;

// Message pinned
let f = PINNED_MESSAGE;

// The StatusUpdate wrapper: any of the above
let f = STATUS_UPDATE;
```

### Via-bot filter

```rust
use telegram_bot_ext::filters::via_bot::ViaBotFilter;

// Only messages sent via a specific bot username
let f = ViaBotFilter::from_username("inlinebot");
```

### Forwarded filter

```rust
use telegram_bot_ext::filters::forwarded::ForwardedFilter;

// Forwarded from a user
let f = ForwardedFilter::from_user();

// Forwarded from a channel
let f = ForwardedFilter::from_channel();
```

---

## Creating Custom Filters

### Option 1: FnFilter (closure)

The simplest approach for one-off filters:

```rust
use telegram_bot_ext::filters::base::FnFilter;

let premium_private = FnFilter::new("premium_private", |update| {
    let is_premium = update["message"]["from"]["is_premium"]
        .as_bool()
        .unwrap_or(false);
    let is_private = update["message"]["chat"]["type"]
        .as_str()
        .map_or(false, |t| t == "private");
    is_premium && is_private
});

// Wrap and compose
let f = F::new(premium_private) & F::new(TEXT);
```

### Option 2: Implement the Filter trait

For reusable filters or ones that carry configuration:

```rust
use telegram_bot_ext::filters::base::{Filter, FilterResult, Update};

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
        let user_id = update["message"]["from"]["id"].as_i64();
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
use telegram_bot_ext::filters::base::{Filter, FilterResult, Update};

pub struct OrderIdFilter;

impl Filter for OrderIdFilter {
    fn check_update(&self, update: &Update) -> FilterResult {
        let text = update["message"]["text"].as_str()?;
        // Simple manual parse
        if let Some(rest) = text.strip_prefix("order #") {
            let order_id = rest.split_whitespace().next()?.to_owned();
            let mut data = HashMap::new();
            data.insert("order_id".to_owned(), vec![order_id]);
            return FilterResult::MatchWithData(data);
        }
        FilterResult::NoMatch
    }

    fn name(&self) -> &str {
        "OrderIdFilter"
    }
}
```

Note: `check_update` takes `&Update` (a `serde_json::Value`), so use `?` with `Option`
via an inline closure or rework the logic to return `FilterResult::NoMatch` explicitly.

---

## Migration from Python Filters

| Python | Rust |
|--------|------|
| `filters.TEXT` | `TEXT` (from `filters::text`) |
| `filters.COMMAND` | `COMMAND` (from `filters::command`) |
| `filters.PHOTO` | `PHOTO` (from `filters::base`) |
| `filters.VIDEO` | `VIDEO` (from `filters::base`) |
| `filters.AUDIO` | `AUDIO` (from `filters::base`) |
| `filters.Document.FileExtension("csv")` | `DocumentFilter::file_extension("csv")` |
| `filters.Regex(r"...")` | `RegexFilter::new(Regex::new(r"...").unwrap())` |
| `filters.User(user_id=42)` | `UserFilter::from_ids([42])` |
| `filters.Chat(chat_id=-100...)` | `ChatFilter::from_ids([-100...])` |
| `filters.Text(["Yes", "No"])` | `TextFilter::new(["Yes", "No"])` |
| `filters.FORWARDED` | `FORWARDED` |
| `filters.PREMIUM_USER` | `PREMIUM_USER` |
| `filters.ALL` | `ALL` |
| `f1 & f2` | `F::new(f1) & F::new(f2)` |
| `f1 \| f2` | `F::new(f1) \| F::new(f2)` |
| `~f` | `!F::new(f)` |

The key difference: Python filters are instances that support operators directly.
In Rust, you must wrap concrete filters in `F` to get the operators. Once wrapped,
the operator syntax is identical.
