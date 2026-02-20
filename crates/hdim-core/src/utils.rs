use std::path::{Path, PathBuf};

pub fn file_name_from_path(path: &Path) -> Option<String> {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(|s| s.to_string())
}

pub fn file_name_from_path_buf(path: &PathBuf) -> Option<String> {
    file_name_from_path(path.as_path())
}
