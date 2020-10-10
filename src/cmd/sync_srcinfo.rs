use super::super::{
    args::SyncSrcInfoArgs,
    manifest::{BuildMetadata, Manifest},
    utils::{read_srcinfo_from_pkgbuild, DbInitError},
};
use pipe_trait::*;
use rayon::prelude::*;
use std::{
    fs::{read_to_string, write},
    io::ErrorKind,
    path::PathBuf,
};

pub fn sync_srcinfo(args: SyncSrcInfoArgs) -> i32 {
    let SyncSrcInfoArgs { update } = args;

    let mut outdated = 0u32;
    let mut error_count = 0u32;

    let manifest = match Manifest::from_env() {
        Ok(manifest) => manifest,
        Err(error) => {
            eprintln!("{}", error);
            eprintln!("1 errors occurred");
            return DbInitError::ManifestLoadingFailure.code();
        }
    };

    enum SyncStatus {
        UpToDate,
        OutOfDate(PathBuf),
    }

    let results: Vec<_> = manifest
        .resolve_members()
        .collect::<Vec<_>>()
        .into_par_iter()
        .filter_map(|member| {
            if member.read_build_metadata.unwrap_or_default() != BuildMetadata::PkgBuild
                && !member.directory.join("PKGBUILD").exists()
            {
                return None;
            }

            let new_srcinfo_content = match read_srcinfo_from_pkgbuild(&member.directory) {
                Ok(content) => content,
                Err(error) => return Some(Err(error)),
            };

            let srcinfo_file = member.directory.join(".SRCINFO");
            let old_srcinfo_content = match read_to_string(&srcinfo_file) {
                Ok(content) => content,
                Err(error) => {
                    if error.kind() == ErrorKind::NotFound {
                        "".to_string()
                    } else {
                        return format!("cannot read {:?} as a file: {}", srcinfo_file, error)
                            .pipe(Err)
                            .pipe(Some);
                    }
                }
            };

            if new_srcinfo_content == old_srcinfo_content {
                return Some(Ok(SyncStatus::UpToDate));
            }

            if update {
                if let Err(error) = write(&srcinfo_file, &new_srcinfo_content) {
                    return format!(
                        "cannot write content to {:?} as a file: {}",
                        srcinfo_file, error,
                    )
                    .pipe(Err)
                    .pipe(Some);
                }
            }

            srcinfo_file.pipe(SyncStatus::OutOfDate).pipe(Ok).pipe(Some)
        })
        .collect();

    for result in results {
        match result {
            Err(error) => {
                eprintln!("{}", error);
                error_count += 1;
            }
            Ok(SyncStatus::OutOfDate(srcinfo_file)) => {
                println!("{}", srcinfo_file.to_string_lossy());
                outdated += 1;
            }
            Ok(SyncStatus::UpToDate) => {}
        }
    }

    if error_count != 0 {
        eprintln!("{} errors occurred", error_count);
        return 1;
    }

    match (update, outdated) {
        (_, 0) | (true, _) => 0,
        _ => 3,
    }
}
