pub mod base;
pub mod dict;
pub mod json_file;

#[cfg(feature = "persistence-sqlite")]
pub mod sqlite;
