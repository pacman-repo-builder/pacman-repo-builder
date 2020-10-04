use super::super::manifest::{BuildMetadata, Manifest, Member};
use super::Pair;
use pipe_trait::*;
use std::{
    fs::{metadata, read},
    path::{Path, PathBuf},
    process::Command,
};

pub fn read_srcinfo_texts(
    manifest: &Manifest<PathBuf>,
    mut handle_error: impl FnMut(String),
) -> Vec<Pair<String, PathBuf>> {
    let mut result = Vec::new();
    for member in manifest.resolve_members() {
        let Member {
            directory,
            read_build_metadata,
            ..
        } = member;

        let srcinfo_result = match read_build_metadata.unwrap_or_default() {
            BuildMetadata::Either => read_either(&directory),
            BuildMetadata::PkgBuild => read_build_dir(&directory),
            BuildMetadata::SrcInfo => directory.join(".SRCINFO").pipe(read_srcinfo_file),
        };

        match srcinfo_result {
            Ok(content) => result.push(Pair::new(content, directory)),
            Err(error) => {
                handle_error(error);
                continue;
            }
        };
    }

    result
}

fn read_either(directory: &Path) -> Result<String, String> {
    let srcinfo_file = directory.join(".SRCINFO");
    let pkgbuild_file = directory.join("PKGBUILD");

    let file_exists = |file: &Path| match metadata(file) {
        Ok(metadata) => metadata.is_file(),
        Err(_) => false,
    };

    if file_exists(&srcinfo_file) {
        read_srcinfo_file(srcinfo_file)
    } else if file_exists(&pkgbuild_file) {
        read_build_dir(directory)
    } else {
        Err(format!(
            "directory {:?} contains neither .SRCINFO nor PKGBUILD",
            directory,
        ))
    }
}

fn read_build_dir(directory: &Path) -> Result<String, String> {
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
            "execution of 'makepkg --printsrcinfo' in directory {:?} exits with code {:?}",
            directory,
            output.status.code(),
        ))
    }
}

fn read_srcinfo_file(file: PathBuf) -> Result<String, String> {
    file.pipe_ref(read)
        .map_err(|error| format!("cannot read file {:?}: {}", file, error))?
        .pipe(String::from_utf8)
        .map_err(|error| {
            format!(
                "cannot convert content of file {:?} to UTF-8: {}",
                file, error
            )
        })
}
