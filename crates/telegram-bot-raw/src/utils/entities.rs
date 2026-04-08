/// A lightweight span describing one entity inside a message text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntitySpan {
    /// UTF-16 codepoint offset of the entity start.
    pub offset: usize,
    /// Length of the entity in UTF-16 codepoints.
    pub length: usize,
    /// The entity type string (e.g. `"bold"`, `"url"`).
    pub entity_type: String,
}

/// Extracts the substring covered by a UTF-16 codepoint range from `text`.
pub fn parse_message_entity(text: &str, offset: usize, length: usize) -> String {
    let utf16: Vec<u16> = text.encode_utf16().collect();
    let slice = &utf16[offset..(offset + length).min(utf16.len())];
    String::from_utf16_lossy(slice)
}

/// Returns `(index, extracted_text)` pairs for entities whose type is in `types`.
///
/// When `types` is `None` all entities are returned.
pub fn parse_message_entities<'a>(
    text: &str,
    entities: &'a [EntitySpan],
    types: Option<&[&str]>,
) -> Vec<(&'a EntitySpan, String)> {
    entities
        .iter()
        .filter(|e| types.map_or(true, |ts| ts.contains(&e.entity_type.as_str())))
        .map(|e| {
            let extracted = parse_message_entity(text, e.offset, e.length);
            (e, extracted)
        })
        .collect()
}
