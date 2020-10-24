use super::super::{
    args::PrintConfigArgs,
    manifest::{
        ArchFilter, BuildMetadata, Manifest, Member, OwnedGlobalSettings, OwnedMember, Wrapper,
    },
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
        with_record_failed_builds,
        with_install_missing_dependencies,
        with_clean_before_build,
        with_clean_after_build,
        with_force_rebuild,
        with_arch_filter,
        with_pacman,
        with_packager,
        with_allow_failure,
        with_dereference_database_symlinks,
    } = args;

    let read_build_metadata = Some(match (args.require_pkgbuild, args.require_srcinfo) {
        (false, false) | (true, true) => BuildMetadata::Either,
        (false, true) => BuildMetadata::SrcInfo,
        (true, false) => BuildMetadata::PkgBuild,
    });

    let global_settings = OwnedGlobalSettings {
        container: None,
        repository: Wrapper::from_inner(repository),
        read_build_metadata,
        record_failed_builds: with_record_failed_builds.map(Wrapper::from_inner),
        install_missing_dependencies: with_install_missing_dependencies,
        clean_before_build: with_clean_before_build,
        clean_after_build: with_clean_after_build,
        force_rebuild: with_force_rebuild,
        arch_filter: ArchFilter::from_arch_vec(with_arch_filter),
        pacman: with_pacman.map(Wrapper::from_inner),
        packager: with_packager.map(Wrapper::from_inner),
        allow_failure: with_allow_failure,
        dereference_database_symlinks: with_dereference_database_symlinks,
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
            members.push(OwnedMember {
                read_build_metadata: None,
                directory: Wrapper::from_inner(directory),
                ..Default::default()
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
