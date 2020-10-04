use super::super::manifest::{BuildMetadata, Manifest, Member};
use super::Pair;
use pipe_trait::*;
use std::{fs::read, path::PathBuf};

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
            BuildMetadata::Either => unimplemented!(),
            BuildMetadata::PkgBuild => unimplemented!(),
            BuildMetadata::SrcInfo => directory
                .join(".SRCINFO")
                .pipe_ref(read)
                .map_err(|error| {
                    format!(
                        "cannot read file {:?}: {}",
                        directory.join(".SRCINFO"),
                        error,
                    )
                })
                .and_then(|content| {
                    content.pipe(String::from_utf8).map_err(|error| {
                        format!(
                            "cannot convert content of file {:?} into a UTF-8 text: {}",
                            directory.join(".SRCINFO"),
                            error
                        )
                    })
                }),
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
