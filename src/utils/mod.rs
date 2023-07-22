use std::path::Path;

/// Validates the ~/.yarr/ directory
pub fn dotfile_is_valid() -> bool {
    exists("~/.yarr/config") &&
        exists("~/.yarr/")
}

fn exists(dir: &str) -> bool {
    Path::new(dir).exists()
}
