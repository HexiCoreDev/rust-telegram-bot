/// Text encoding name constants.
pub mod text_encoding {
    /// UTF-8 encoding label.
    pub const UTF_8: &str = "utf-8";
    /// UTF-16 little-endian encoding label.
    pub const UTF_16_LE: &str = "utf-16-le";
}

/// Converts a `snake_case` string to `camelCase`.
pub fn to_camel_case(snake_str: &str) -> String {
    let mut components = snake_str.split('_');
    let first = components.next().unwrap_or("").to_owned();
    let rest: String = components
        .map(|s| {
            let mut chars = s.chars();
            match chars.next() {
                None => String::new(),
                Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect();
    first + &rest
}
