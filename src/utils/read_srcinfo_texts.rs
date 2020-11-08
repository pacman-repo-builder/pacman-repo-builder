use super::super::manifest::{BuildMetadata, Member, OwnedBuildPacmanRepo, OwnedMember};
use super::{read_srcinfo_file, read_srcinfo_from_directory, read_srcinfo_from_pkgbuild, Pair};
use pipe_trait::*;
use rayon::prelude::*;
use std::path::Path;

pub fn read_srcinfo_texts(
    manifest: &OwnedBuildPacmanRepo,
    mut handle_error: impl FnMut(String),
) -> Vec<Pair<String, OwnedMember>> {
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

            let directory: &Path = directory.as_ref();

            (
                match read_build_metadata.unwrap_or_default() {
                    BuildMetadata::Either => read_srcinfo_from_directory(directory),
                    BuildMetadata::PkgBuild => read_srcinfo_from_pkgbuild(directory),
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
