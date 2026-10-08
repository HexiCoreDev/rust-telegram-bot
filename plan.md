# Plan — Bot API 9.6 → 10.3 upgrade

## 0. Goal

Make `rust-telegram-bot` the reference Rust port of Bot API `10.3` (Aug 24 2026,
verified live via Playwright on `core.telegram.org/bots/api-changelog` + `/bots/api`):
complete methods/types, every method with `*_raw` + `*Builder` + `IntoFuture`,
plus measurable perf wins, best-in-class DX, and safe retries/idempotency.
No shortcuts — full args, not just names.

Non-goals: no new transport (still reqwest/rustls), no MSRV bump without approval,
no breaking change without shim + CHANGELOG entry.

## 1. Current state (evidence)

- `Cargo.toml:14`, `README.md:12`: `1.0.0-rc.1`, badge `Bot API 9.6`.
- `crates/telegram-bot-raw/src/bot/mod.rs:598-618`: 22 method-group modules;
  `bot_builders*.rs`: ~170 builders; `grep _raw`: 128 raw fns.
- `crates/telegram-bot-raw/src/types/mod.rs:1-180`: ~73 files, no `rich/`,
  `ephemeral`, `guest`, `live_photo`, `community` modules.
- `grep RichMessage|Ephemeral|Guest|LivePhoto|Community|DisabledButton|sendRich|answerGuest src/`: 0 hits.
- Live API: ~185 methods, ~300 types. Gap ≈ 15 new methods, ~20 modified
  signatures, ~90 new types.

## 2. Spec harvest (source of truth)

Scrape `core.telegram.org/bots/api` h4 sections via Playwright into
`/tmp/opencode/api-10.3-spec.json`:
`{methods: [{name, required: [{name, type}], optional: [...], returns}],
types: [{name, fields: [{name, type, required}]}]}`.
Diff vs `current_methods.txt` + builder list. Output: gap table with per-method
arg deltas. Checked in as `docs/api-10.3-gap.md` for review.
This answers "full commands and args".

## 3. Order of implementation

### Phase A — 10.0 foundation (types + methods)

Types: `LivePhoto`, `InputMediaLivePhoto`, `PaidMediaLivePhoto`,
`InputPaidMediaLivePhoto`, `PollMedia`, `InputPollMedia`, `InputPollOptionMedia`,
`InputMediaSticker/Location/Venue`, `SentGuestMessage`, `BotAccessSettings`,
`Message{guest_bot_caller_user, guest_bot_caller_chat, guest_query_id, live_photo}`,
`Update{guest_message}`, `User{supports_guest_queries}`,
`ChatMemberRestricted{can_react_to_messages}`,
`ChatPermissions{can_react_to_messages}`,
`Poll{members_only, country_codes, media}`, `PollOption{media}`,
`InputPollOption{media}`.
Methods: `sendLivePhoto`, `answerGuestQuery`, `getUserPersonalChatMessages`,
`getManagedBotAccessSettings`, `setManagedBotAccessSettings`,
`deleteMessageReaction`, `deleteAllMessageReactions`, `sendPoll` extensions.
Each: `*_raw` in `src/bot/` group module + builder in `bot_builders*.rs` +
re-export + roundtrip test.

### Phase B — 10.1 rich core (largest)

Types: `RichText*` (~25: Bold/Italic/Underline/Strike/Spoiler/DateTime/
TextMention/Sub/Super/Marked/Code/CustomEmoji/Math/Url/Email/Phone/BankCard/
Mention/Hashtag/Cashtag/BotCommand/Button/Anchor/AnchorLink/Reference/
ReferenceLink), `RichText`, `RichBlockCaption/TableCell/ListItem`,
`RichBlock*` (~20: Paragraph/SectionHeading/Pre/Footer/Divider/Math/Anchor/List/
BlockQuote/PullQuote/Collage/Slideshow/Table/Details/Map/Animation/Audio/Photo/
Video/VoiceNote/Thinking), `RichBlock`, `RichMessage`, `Message::rich_message`,
`InputRichMessage`, `InputRichMessageContent` (usable as `InputMessageContent`),
`Link`, `PollMedia::link`, `InputMediaLink`,
`User{supports_join_request_queries}`, `ChatFullInfo::guard_bot`,
`ChatJoinRequest::query_id`.
Methods: `sendRichMessage`, `sendRichMessageDraft`, `answerChatJoinRequestQuery`,
`sendChatJoinRequestWebApp`, `editMessageText{rich_message}`.
Plumbing: `InputMessageContent` enum new variant, inline/guest/webapp query
result acceptance.

### Phase C — 10.2 ephemeral + communities + input blocks

Types: `InputRichMessageMedia` + `InputRichMessage::media`,
`InputMediaVoiceNote`, `InputRichBlockListItem`, `InputRichBlock*` (~20 mirrors
of B), `InputRichMessage::blocks`, `BotCommand::is_ephemeral`,
`Message{receiver_user, ephemeral_message_id}`,
`ReplyParameters{ephemeral_message_id, message_id now optional-if-ephemeral}`,
`Community`, `CommunityChatAdded/Removed` + `Message` fields,
`ChatFullInfo::community`, `BotSubscriptionUpdated` + `Update::subscription`.
Methods: `editEphemeralMessageText/Media/Caption/ReplyMarkup`,
`deleteEphemeralMessage`, 12 sends gain `receiver_user_id` + `callback_query_id`.
UpdateKind: new `Subscription` variant; `effective_*` helpers cover new message
carriers.

### Phase D — 10.3 migration + refinements (breaking-shape, needs shims)

New: `RichMessageButton`, `RichTextButton`, `RichBlockButtons`/
`InputRichBlockButtons`, `RichBlockTable/InputRichBlockTable::is_compact`,
`RichBlockExpandableBlockQuotation`/`Input...`, `RichBlockDocument`/
`InputRichBlockDocument`, `tg://document?id=` link support,
`EphemeralMessageParameters{receiver_user_id, callback_query_id?,
replace_callback_query_message?}` replacing per-method `receiver_user_id`+
`callback_query_id` on 13 methods (12 sends + `sendRichMessage`),
`editEphemeralMessageMedia` file-upload,
`editEphemeralMessageCaption{show_caption_above_media}`,
`editEphemeralMessageText{rich_message}`,
`ChatAdministratorRights/ChatMemberAdministrator::can_send_welcome_messages` +
`promoteChatMember` param, `DisabledButton` + `InlineKeyboardButton::disabled`,
`force_reply` on `InlineKeyboardMarkup`/`ReplyKeyboardMarkup`,
drafts `{can_stop, keep_on_stop}`, `MessageGenerationStopped` +
`Update::stopped_message_generation`, `CommunityChatJoined` + `Message` field,
`UniqueGiftInfo{text, entities, is_private}`.
Back-compat: keep deprecated `receiver_user_id()`/`callback_query_id()` builder
setters for 1 minor version, forwarding into `ephemeral_message_parameters`;
`#[deprecated]` + CHANGELOG + example update.

### Phase E — Plumbing, builders, tests

Every method: `*_raw` + builder + `IntoFuture` + `Defaults` merge +
`api_kwargs` passthrough. `Update::kind` + `Message` size audit (`Box` large
variants). 25+ roundtrip serialization tests from real payloads, proptest for
`EphemeralMessageParameters` ↔ legacy setters equivalence,
`cargo doc` clean.

### Phase F — Performance (everywhere, measured)

Hot paths: `do_post_inner` (`bot/mod.rs:407-431`), `do_post_json` fast path
(`:438-449`), `apply_defaults` (`:338-384` HashSet alloc per call),
`resolve_file_paths` (`:309-336` sequential fs reads),
`RequestData::from_parameters`, update dispatch (`ext/update_processor.rs`,
`updater.rs:1-714`, `application.rs:1-1585` bounded ch 64).
Changes: single-pass `to_vec` for all text-only builders; `apply_defaults`
without HashSet (flag checks); batched/concurrent file reads;
`EphemeralMessageParameters` single struct shared by 13 builders (no duplicated
serialization); `Box` large `Message`/`UpdateKind` payloads; pool tunable
(now 8) + `TCP_NODELAY`/keepalive audit; filter fast-paths (command/text first);
webhook backpressure keep 503; persistence batching. Bench:
`criterion benches/throughput` + `benchmarks/` RTB vs teloxide vs PTB
(RSS idle/load, msg/s, p99). Target: no regression, binary ≤6.5MB stripped,
RSS ≤17MB load.

### Phase G — DX / ease of use

Keep: `Bot::send_message(...).parse_mode(...).await`, typed
`ParseMode/ChatType/...`, `Context::reply_*`, `ApplicationBuilder` typestate,
`prelude`. Add: `EphemeralMessageParameters::new(receiver).callback_query(...)
.replace_original(...)`, `RichMessage` builder helpers (`paragraph()`, `list()`,
`table()`), `InputRichMessage::from_markdown/html` with explicit `media`,
per-method doc examples, `sendRichMessageDraft` streaming example, error type
with `method` + `retry_after` + `migrated_to_chat_id` hints.

### Phase H — Idempotency / correctness

Telegram has no client idempotency keys, so: update dedup by `update_id`
(in-memory LRU + persistence-backed seen-set, survives restart); at-least-once
webhook/polling offset commit only after successful dispatch; retry only safe
methods (no retry on `sendMessage` success-ambiguous timeouts without dedup key
— document + provide `IdempotentSender` with caller-provided key stored in
`bot_data`); `OnceCell` bot-data init stays; graceful shutdown drains in-flight
updates before offset commit; secret-token constant-time check stays. Tests:
duplicate-update delivery, restart-recovery, retry-storm.

### Phase I — Release

`cargo check/test/clippy/fmt --jobs 4`, docs build, examples build, README badge
`10.3`, comparison table PTB 22.8, CHANGELOG entries per phase, version
`1.0.0-rc.2`.

## 4. Risks

Rich blocks (~70 types) dominate volume — codegen from spec JSON, hand-review.
10.3 ephemeral-object migration is the only intentional breaking shape — shims
required. Poll `correct_option_id` → `correct_option_ids` legacy already handled
in PTB 22.8 — mirror it.

## 5. Execution chunks (stop for review after each)

1. Write this file + spec JSON + gap table. 2. Phase A. 3. Phase B. 4. Phase C.
5. Phase D. 6. Phase E. 7. Phase F–I.
