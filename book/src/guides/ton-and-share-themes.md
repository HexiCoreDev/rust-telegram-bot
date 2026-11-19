# TON Wallet and Share Themes

Bot API 10.4 exposes two pieces of profile decoration that were previously invisible
to bots: the **TON wallet card** a user shows on their profile, and the **theme** of
the screen shown when someone shares a link to a user or a chat.

> There is no runnable example for these yet: both are read-mostly data that depends
> on a user's own Telegram profile.

## Wallet cards

`getChat` returns `ChatFullInfo::ton_wallet_card` for private chats:

```rust
let info = context.bot().get_chat(chat_id).await?;

if let Some(card) = info.ton_wallet_card.as_ref() {
    // `amount` is a decimal string in the smallest units of the cryptocurrency.
    println!("wallet {} holds {}", card.address, card.amount);

    // Stickers are NFTs placed on the card.
    for sticker in &card.stickers {
        let nft = &sticker.nft;
        println!("  {} #{} ({})", nft.name, nft.index, nft.address);
        for attribute in nft.attributes.iter().flatten() {
            println!("    {} = {}", attribute.trait_type, attribute.value);
        }
    }

    // Colours of the card background come from one of the sticker NFTs.
    if let Some(background) = card.background_nft_address.as_deref() {
        println!("background from {background}");
    }
}
```

`TonNft` also carries `collection_address` and, for gifts that live on the
blockchain, `gift: Option<UniqueGift>`.

## Transfers

Service messages about a transfer arrive on `Message::ton_wallet_transfer`:

```rust
if let Some(transfer) = update
    .effective_message()
    .and_then(|message| message.ton_wallet_transfer.as_deref())
{
    // Negative amounts are outgoing transfers; `comment` is absent when encrypted.
    println!(
        "{} -> {}: {} ({})",
        transfer.peer_address,
        transfer.transaction_id,
        transfer.amount,
        transfer.comment.as_deref().unwrap_or("no comment"),
    );
}
```

## TON addresses in text

- Rich messages format an address with `RichTextTonAddress`
  (`"ton_address"` in the wire format):

  ```rust
  use rust_tg_bot::raw::types::rich_text::{
      RichText, RichTextFormatted, RichTextTonAddress,
  };

  let address = RichText::Formatted(Box::new(RichTextFormatted::TonAddress(
      RichTextTonAddress::new("EQabc…", "EQabc…"),
  )));
  ```

- Plain-text messages report one through `MessageEntityType::TonAddress`
  (`"ton_address"`), and bank card numbers through
  `MessageEntityType::BankCardNumber` (`"bank_card_number"`).

## Areas and the `AreaPosition` rename

The rectangle descriptor formerly called `StoryAreaPosition` is now
`AreaPosition`, because it also positions stickers on wallet cards:

```rust
use rust_tg_bot::raw::types::area_position::AreaPosition;

let area = AreaPosition::new(50.0, 50.0, 20.0, 20.0, 0.0, 10.0);
```

The 10.3 name still resolves as a `#[deprecated]` alias
(`types::story_area::StoryAreaPosition`), and the limits module was renamed to
`constants::area_position_limit` (the old `story_area_position_limit` still
resolves). New code should use `AreaPosition`.

## Share themes

`ChatFullInfo::share_theme` is omitted when the default theme is in use; otherwise
it is one of two shapes:

```rust
use rust_tg_bot::raw::types::chat_share_theme::ChatShareTheme;

match info.share_theme.as_ref() {
    Some(ChatShareTheme::Emoji(theme)) => {
        // 0-6 regular or custom emoji on top of a backdrop.
        println!("{} emoji", theme.emojis.len());
        if let Some(backdrop_id) = theme.background_backdrop_id {
            println!("on backdrop #{backdrop_id}");
        }
    }
    Some(ChatShareTheme::UniqueGift(theme)) => {
        println!("theme built from gift {}", theme.gift.name);
    }
    _ => {}
}
```

`EmojiType` is either a regular emoji or a custom emoji id. Constructors cover both
sides, so a theme is easy to build for code that compares or stores them:

```rust
use rust_tg_bot::raw::types::chat_share_theme::{ChatShareTheme, EmojiType};

let theme = ChatShareTheme::emoji(vec![
    EmojiType::emoji("🔥"),
    EmojiType::custom_emoji("5368322736591234"),
]);
```

## Backdrop catalogue

`getChatShareThemeBackdrops` lists the backgrounds a theme can use; `colors` is the
same `UniqueGiftBackdropColors` shape unique gifts use:

```rust
let backdrops = context.bot().get_chat_share_theme_backdrops().await?;

for backdrop in &backdrops.backdrops {
    println!(
        "#{} {} (center #{:06x})",
        backdrop.id, backdrop.name, backdrop.colors.center_color
    );
}
```

## Gifts behind the data

Two 10.4 methods complete the picture: `getUniqueGift(name)` looks up a unique gift
by its `t.me/nft` name, and `deleteBusinessAccountGift(business_connection_id,
owned_gift_id)` deletes a regular gift owned by a managed business account (requires
the `can_convert_gifts_to_stars` right).

## Next Steps

Wallet cards also travel with widget queries; see [HTML Widgets](widgets.md). For
rich-message formatting see [Rich Messages](rich-messages.md), and for the exact
field semantics the [Bot API reference](https://core.telegram.org/bots/api) is
authoritative.
