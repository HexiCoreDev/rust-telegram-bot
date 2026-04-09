//! Internal macros for reducing boilerplate across type implementations.

/// Adds a `new()` constructor that takes required fields and defaults the rest.
///
/// Requires `Default` to be derived on the target struct. Required fields use
/// `impl Into<T>` for ergonomic construction.
///
/// # Usage
///
/// ```ignore
/// impl_new!(BotCommand { command: String, description: String });
/// ```
///
/// Generates:
///
/// ```ignore
/// impl BotCommand {
///     pub fn new(command: impl Into<String>, description: impl Into<String>) -> Self {
///         Self {
///             command: command.into(),
///             description: description.into(),
///             ..Default::default()
///         }
///     }
/// }
/// ```
macro_rules! impl_new {
    // Variant with no required fields (unit-like constructor).
    ($type:ident {}) => {
        impl $type {
            /// Creates a new instance with all fields set to their defaults.
            pub fn new() -> Self {
                Self::default()
            }
        }
    };

    // Variant with required fields.
    ($type:ident { $($req:ident : $req_ty:ty),+ $(,)? }) => {
        impl $type {
            /// Creates a new instance with the given required fields; optional fields
            /// are initialised to their `Default` values.
            pub fn new($($req: impl Into<$req_ty>),+) -> Self {
                Self {
                    $($req: $req.into(),)+
                    ..Default::default()
                }
            }
        }
    };
}
