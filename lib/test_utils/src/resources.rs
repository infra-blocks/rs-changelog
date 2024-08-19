use std::path::PathBuf;

pub fn resources_dir() -> PathBuf {
    let mut cwd = std::env::current_dir().unwrap();

    loop {
        match cwd.file_name() {
            Some(file_name) => {
                if file_name.eq("rs-changelog") {
                    return cwd.join("resources");
                } else {
                    cwd.pop();
                }
            }
            None => {
                panic!(
                    "couldn't find git root from current directory: {}",
                    cwd.to_str().unwrap()
                );
            }
        }
    }
}

pub fn changelog_file(name: &str) -> PathBuf {
    resources_dir().join(name)
}
