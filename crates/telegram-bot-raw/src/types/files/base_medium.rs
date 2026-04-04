/// Marker trait for Telegram media objects that expose a file identifier.
///
/// Implemented by all concrete media structs (Animation, Audio, Document, etc.)
/// to allow generic code to access the common file identity fields without
/// knowing the concrete type.
pub trait BaseMedium {
    /// The Telegram file identifier that can be used to download or reuse the file.
    fn file_id(&self) -> &str;

    /// The stable unique identifier — the same across time and different bots.
    /// Cannot be used to download or reuse the file.
    fn file_unique_id(&self) -> &str;
}
