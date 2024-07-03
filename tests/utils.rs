use std::path::PathBuf;

pub fn changelog_dir() -> PathBuf {
    ["tests", "resources"].iter().collect()
}

pub fn changelog_file(name: &str) -> PathBuf {
    changelog_dir().join(name)
}
