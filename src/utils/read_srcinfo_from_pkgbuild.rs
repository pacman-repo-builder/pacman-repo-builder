use super::create_makepkg_command;
use pipe_trait::*;
use std::path::Path;

pub fn read_srcinfo_from_pkgbuild(directory: &Path) -> Result<String, String> {
    let output = create_makepkg_command()
        .current_dir(directory)
        .arg("--printsrcinfo")
        .output()
        .map_err(|error| {
            format!(
                "⮾ Fail to execute 'makepkg --printsrcinfo' in directory {:?}: {}",
                directory, error,
            )
        })?;

    if output.status.success() {
        output
            .stdout
            .pipe(String::from_utf8)
            .map_err(|error| {
                format!(
                "⮾ Fail to convert output of 'makepkg --printsrcinfo' in directory {:?} to UTF-8: {}",
                directory, error,
            )
            })?
            .pipe(Ok)
    } else {
        Err(format!(
            "⮾ Execution of 'makepkg --printsrcinfo' in directory {:?} exits with code {:?}\n{}",
            directory,
            output.status.code(),
            output.stderr.as_slice().pipe(String::from_utf8_lossy),
        ))
    }
}
