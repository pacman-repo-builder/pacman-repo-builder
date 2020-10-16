use super::super::{
    args::PrintConfigArgs,
    manifest::{BuildMetadata, GlobalSettings, Manifest, Member},
    status::{Code, Status},
};
use pipe_trait::*;
use std::{
    fs::{metadata, read_dir},
    io::stdout,
};

pub fn print_config(args: PrintConfigArgs) -> Status {
    let mut error_count = 0u32;

    let PrintConfigArgs {
        containers,
        repository,
        require_pkgbuild,
        require_srcinfo,
    } = args;

    let read_build_metadata = Some(match (args.require_pkgbuild, args.require_srcinfo) {
        (false, false) | (true, true) => BuildMetadata::Either,
        (false, true) => BuildMetadata::SrcInfo,
        (true, false) => BuildMetadata::PkgBuild,
    });

    let global_settings = GlobalSettings {
        container: None,
        read_build_metadata,
        repository,
    };

    let mut members = Vec::new();
    for container in containers {
        let list = match read_dir(&container) {
            Err(error) => {
                eprintln!("⮾ Cannot read directory {:?}: {}", &container, error);
                error_count += 1;
                continue;
            }
            Ok(list) => list,
        };
        for entry in list {
            let directory = match entry {
                Err(error) => {
                    eprintln!("⮾ Cannot read an entry of {:?}: {}", &container, error);
                    error_count += 1;
                    continue;
                }
                Ok(entry) => entry,
            }
            .path();
            match metadata(&directory) {
                Err(error) => {
                    eprintln!("⮾ Cannot stat {:?}: {}", &directory, error);
                    error_count += 1;
                    continue;
                }
                Ok(metadata) => {
                    if !metadata.is_dir() {
                        continue;
                    }
                }
            }
            let file_exists = |name: &'static str| match directory.join(name).pipe(metadata) {
                Ok(metadata) => metadata.is_file(),
                Err(_) => false,
            };
            if require_pkgbuild && !file_exists("PKGBUILD") {
                continue;
            }
            if require_srcinfo && !file_exists(".SRCINFO") {
                continue;
            }
            members.push(Member {
                read_build_metadata: None,
                directory,
            });
        }
    }
    members.sort_by(|a, b| a.directory.cmp(&b.directory));
    let members: Vec<_> = members.iter().map(Member::as_path).collect();

    let manifest = Manifest {
        global_settings: global_settings.as_path(),
        members,
    };
    if let Err(error) = serde_yaml::to_writer(stdout(), &manifest) {
        eprintln!("⮾ Cannot write yaml to stdout: {}", error);
        error_count += 1;
    };

    if error_count == 0 {
        Ok(0)
    } else {
        eprintln!("{} errors occurred.", error_count);
        Code::GenericFailure.into()
    }
}
