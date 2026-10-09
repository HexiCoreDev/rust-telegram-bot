# Rich Messages

Rich messages are highly structured content: headings, lists, tables, media,
quotations, collapsible blocks, footnotes, formulas, and inline buttons — with
seamless formatting for streamed AI replies. Three ways to produce them, from
simplest to most explicit: Markdown, HTML, or blocks.

See the runnable `rich_message_bot` example
(`crates/telegram-bot/examples/rich_message_bot.rs`).

## Markdown

The fastest path. GitHub-flavored Markdown plus a few extras (task lists,
tables, footnotes, collapsible `<details>`, media blocks):

```rust
use rust_tg_bot::raw::types::input_rich_message::InputRichMessage;

let message = InputRichMessage::markdown("# Report\n\n- **up**\n- _down_");
context.bot().send_rich_message(chat_id, message).await?;
```

Plain URLs, mentions, hashtags, commands, and phone numbers are detected
automatically unless you call `.skip_entity_detection()`.

## HTML

For features without Markdown syntax (subscript, anchors, maps, buttons),
use HTML:

```rust
let message = InputRichMessage::html(
    "<h1>Report</h1><p>Speed: <b>42</b> <sup>ms</sup></p>",
);
```

## Blocks

For full control, build the block tree explicitly. Blocks nest: a list holds
items, items hold blocks, quotations hold blocks:

```rust
use rust_tg_bot::raw::types::input_rich_block::{
    InputRichBlock, InputRichBlockList, InputRichBlockListItem,
    InputRichBlockParagraph, InputRichBlockSectionHeading,
};
use rust_tg_bot::raw::types::input_rich_message::InputRichMessage;

let message = InputRichMessage::blocks(vec![
    InputRichBlock::SectionHeading(InputRichBlockSectionHeading::new("Report", 1)),
    InputRichBlock::Paragraph(InputRichBlockParagraph::new("Highlights.")),
    InputRichBlock::List(InputRichBlockList::new(vec![
        InputRichBlockListItem::new(vec![InputRichBlock::Paragraph(
            InputRichBlockParagraph::new("Revenue up"),
        )]),
    ])),
]);
context.bot().send_rich_message(chat_id, message).await?;
```

Media blocks take `InputMedia*` values; the caption on a media block is
ignored (captions belong on `RichBlockCaption`). Reuse previously uploaded
files in markup with `tg://photo?id=...` links and supply the bytes through
`InputRichMessageMedia`.

## Receiving

Incoming rich messages arrive as `Message.rich_message` with a `RichMessage`
(blocks plus an `is_rtl` flag). Received blocks (`RichBlock`) mirror the
input shapes, so pattern-match on them directly:

```rust
use rust_tg_bot::raw::types::rich_block::RichBlock;

if let Some(rich) = &msg.rich_message {
    for block in &rich.blocks {
        if let RichBlock::SectionHeading(h) = block {
            println!("heading level {} received", h.size);
        }
    }
}
```

## Streaming drafts

While generating a reply, stream partial output as an ephemeral 30-second
preview, then persist the final message. Pass `can_stop(true)` to offer a
stop button; a press arrives as an `Update` with `stopped_message_generation`:

```rust
context
    .bot()
    .send_rich_message_draft(chat_id, draft_id, InputRichMessage::markdown("Thinking…"))
    .can_stop(true)
    .await?;
// ... generate ...
context
    .bot()
    .send_rich_message(chat_id, InputRichMessage::markdown("# Done"))
    .await?;
```

## Limits

Up to 32768 characters, 500 blocks, 16 nesting levels, 50 media attachments,
and 20 table columns per message.
