use pipe_trait::*;
use std::{path::Path, process::Command};

pub fn read_srcinfo_from_pkgbuild(directory: &Path) -> Result<String, String> {
    let output = Command::new("makepkg")
        .current_dir(directory)
        .arg("--printsrcinfo")
        .output()
        .map_err(|error| {
            format!(
                "fail to execute 'makepkg --printsrcinfo' in directory {:?}: {}",
                directory, error,
            )
        })?;

    if output.status.success() {
        output
            .stdout
            .pipe(String::from_utf8)
            .map_err(|error| {
                format!(
                "fail to convert output of 'makepkg --printsrcinfo' in directory {:?} to UTF-8: {}",
                directory, error,
            )
            })?
            .pipe(Ok)
    } else {
        Err(format!(
            "execution of 'makepkg --printsrcinfo' in directory {:?} exits with code {:?}\n{}",
            directory,
            output.status.code(),
            output.stderr.as_slice().pipe(String::from_utf8_lossy),
        ))
    }
}
