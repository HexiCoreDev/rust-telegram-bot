use std::sync::OnceLock;

use regex::Regex;

static DEEP_LINK_RE: OnceLock<Regex> = OnceLock::new();

fn deep_link_re() -> &'static Regex {
    DEEP_LINK_RE.get_or_init(|| Regex::new(r"^[A-Za-z0-9_\-]+$").unwrap())
}

/// Escapes Telegram Markdown special characters for the given `version` (1 or 2).
pub fn escape_markdown(text: &str, version: u8, entity_type: Option<&str>) -> String {
    let escape_chars: &str = if version == 1 {
        r"_*`["
    } else {
        match entity_type {
            Some("pre") | Some("code") => r"\`",
            Some("text_link") | Some("custom_emoji") => r"\)",
            _ => r"\_*[]()~`>#+-=|{}.!",
        }
    };

    let mut out = String::with_capacity(text.len());
    for ch in text.chars() {
        if escape_chars.contains(ch) {
            out.push('\\');
        }
        out.push(ch);
    }
    out
}

fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for ch in s.chars() {
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#x27;"),
            c => out.push(c),
        }
    }
    out
}

/// Creates an HTML mention `<a href="tg://user?id=...">name</a>` for a user.
pub fn mention_html(user_id: i64, name: &str) -> String {
    format!(
        "<a href=\"tg://user?id={user_id}\">{}</a>",
        html_escape(name)
    )
}

/// Creates a Markdown mention link for a user.
pub fn mention_markdown(user_id: i64, name: &str, version: u8) -> String {
    let tg_link = format!("tg://user?id={user_id}");
    if version == 1 {
        format!("[{name}]({tg_link})")
    } else {
        format!("[{}]({tg_link})", escape_markdown(name, version, None))
    }
}

/// Creates a deep-linked `t.me` URL for the given bot.
///
/// Returns `Err` if `bot_username` is too short, the payload exceeds 64 chars,
/// or the payload contains characters outside `[A-Za-z0-9_-]`.
pub fn create_deep_linked_url(
    bot_username: &str,
    payload: Option<&str>,
    group: bool,
) -> Result<String, String> {
    if bot_username.len() <= 3 {
        return Err("You must provide a valid bot_username.".to_owned());
    }

    let base_url = format!("https://t.me/{bot_username}");

    let payload = match payload {
        None | Some("") => return Ok(base_url),
        Some(p) => p,
    };

    if payload.len() > 64 {
        return Err("The deep-linking payload must not exceed 64 characters.".to_owned());
    }

    if !deep_link_re().is_match(payload) {
        return Err(
            "Only the following characters are allowed for deep-linked URLs: A-Z, a-z, 0-9, _ and -"
                .to_owned(),
        );
    }

    let key = if group { "startgroup" } else { "start" };
    Ok(format!("{base_url}?{key}={payload}"))
}
