use pipe_trait::*;
use std::{fs::read, path::PathBuf};

pub fn read_srcinfo_file(file: PathBuf) -> Result<String, String> {
    file.pipe_ref(read)
        .map_err(|error| format!("⮾ Cannot read file {:?}: {}", file, error))?
        .pipe(String::from_utf8)
        .map_err(|error| {
            format!(
                "⮾ Cannot convert content of file {:?} to UTF-8: {}",
                file, error
            )
        })
}
