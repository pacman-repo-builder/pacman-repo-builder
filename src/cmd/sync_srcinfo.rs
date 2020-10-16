use super::super::{
    args::SyncSrcInfoArgs,
    manifest::{BuildMetadata, Manifest, Member},
    status::{Code, Status},
    utils::read_srcinfo_from_pkgbuild,
};
use pipe_trait::*;
use rayon::prelude::*;
use std::{
    fs::{read_to_string, write},
    io::ErrorKind,
    path::Path,
};

pub fn sync_srcinfo(args: SyncSrcInfoArgs) -> Status {
    let SyncSrcInfoArgs { update } = args;

    let mut outdated = 0u32;
    let mut error_count = 0u32;

    let manifest = Manifest::from_env().map_err(|error| {
        eprintln!("⮾ {}", error);
        Code::ManifestLoadingFailure
    })?;

    struct SyncStatus<'a> {
        up_to_date: bool,
        directory: &'a Path,
    }

    let members: Vec<_> = manifest.resolve_members().collect();
    let results: Vec<_> = members
        .par_iter()
        .filter_map(|member| {
            let Member {
                ref directory,
                read_build_metadata,
                ..
            } = member;

            if read_build_metadata.unwrap_or_default() != BuildMetadata::PkgBuild
                && !directory.join("PKGBUILD").exists()
            {
                return None;
            }

            let new_srcinfo_content = match read_srcinfo_from_pkgbuild(directory) {
                Ok(content) => content,
                Err(error) => return Some(Err(error)),
            };

            let srcinfo_file = directory.join(".SRCINFO");
            let old_srcinfo_content = match read_to_string(&srcinfo_file) {
                Ok(content) => content,
                Err(error) => {
                    if error.kind() == ErrorKind::NotFound {
                        "".to_string()
                    } else {
                        return format!("⮾ Cannot read {:?} as a file: {}", srcinfo_file, error)
                            .pipe(Err)
                            .pipe(Some);
                    }
                }
            };

            fn comparable(srcinfo: &str) -> impl Iterator<Item = &str> {
                srcinfo
                    .lines()
                    .map(|x| x.trim_end())
                    .filter(|x| !x.is_empty())
            }

            if comparable(&new_srcinfo_content).eq(comparable(&old_srcinfo_content)) {
                return Some(Ok(SyncStatus {
                    up_to_date: true,
                    directory,
                }));
            }

            if update {
                if let Err(error) = write(&srcinfo_file, &new_srcinfo_content) {
                    return format!(
                        "⮾ Cannot write content to {:?} as a file: {}",
                        srcinfo_file, error,
                    )
                    .pipe(Err)
                    .pipe(Some);
                }
            }

            Some(Ok(SyncStatus {
                up_to_date: false,
                directory,
            }))
        })
        .collect();

    for result in results {
        match result {
            Err(error) => {
                eprintln!("{}", error);
                error_count += 1;
            }
            Ok(SyncStatus {
                up_to_date: false,
                directory,
            }) => {
                println!("{}", directory.to_string_lossy());
                outdated += 1;
            }
            Ok(SyncStatus {
                up_to_date: true, ..
            }) => {}
        }
    }

    if error_count != 0 {
        eprintln!("{} errors occurred", error_count);
        return Err(Code::GenericFailure);
    }

    match (update, outdated) {
        (_, 0) | (true, _) => Ok(0),
        _ => Err(Code::SrcInfoOutOfSync),
    }
}
