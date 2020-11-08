use super::{read_srcinfo_file, read_srcinfo_from_pkgbuild};
use std::{fs::metadata, path::Path};

pub fn read_srcinfo_from_directory(directory: &Path) -> Result<String, String> {
    let srcinfo_file = directory.join(".SRCINFO");
    let pkgbuild_file = directory.join("PKGBUILD");

    let file_exists = |file: &Path| match metadata(file) {
        Ok(metadata) => metadata.is_file(),
        Err(_) => false,
    };

    if file_exists(&srcinfo_file) {
        read_srcinfo_file(srcinfo_file)
    } else if file_exists(&pkgbuild_file) {
        read_srcinfo_from_pkgbuild(directory)
    } else {
        Err(format!(
            "⮾ Directory {:?} contains neither .SRCINFO nor PKGBUILD",
            directory,
        ))
    }
}
