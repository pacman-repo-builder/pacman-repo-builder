use super::super::manifest::{BuildMetadata, Manifest, Member};
use super::{read_srcinfo_from_pkgbuild, Pair};
use pipe_trait::*;
use rayon::prelude::*;
use std::{
    fs::{metadata, read},
    path::{Path, PathBuf},
};

pub fn read_srcinfo_texts(
    manifest: &Manifest<PathBuf>,
    mut handle_error: impl FnMut(String),
) -> Vec<Pair<String, Member<PathBuf>>> {
    manifest
        .resolve_members()
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|member| {
            let Member {
                directory,
                read_build_metadata,
                ..
            } = &member;

            (
                match read_build_metadata.unwrap_or_default() {
                    BuildMetadata::Either => read_either(&directory),
                    BuildMetadata::PkgBuild => read_srcinfo_from_pkgbuild(&directory),
                    BuildMetadata::SrcInfo => directory.join(".SRCINFO").pipe(read_srcinfo_file),
                },
                member,
            )
        })
        .collect::<Vec<_>>()
        .into_iter()
        .filter_map(|(srcinfo_result, member)| match srcinfo_result {
            Ok(content) => Some(Pair::new(content, member)),
            Err(error) => {
                handle_error(error);
                None
            }
        })
        .collect()
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
        read_srcinfo_from_pkgbuild(directory)
    } else {
        Err(format!(
            "directory {:?} contains neither .SRCINFO nor PKGBUILD",
            directory,
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
