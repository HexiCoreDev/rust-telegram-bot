use std::path::Path;

/// Returns `true` when `path` refers to an existing regular file on the local filesystem.
pub fn is_local_file(path: &str) -> bool {
    Path::new(path).is_file()
}

/// Extracts the final filename component from a path string.
pub fn guess_file_name(path: &str) -> Option<String> {
    Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .map(|s| s.to_owned())
}
