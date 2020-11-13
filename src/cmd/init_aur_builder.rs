use super::super::{
    args::InitAurBuilderArgs,
    manifest::{
        BuildPacmanRepo, GlobalSettings, InitAurBuilder, OwnedMember, Wrapper, BUILD_PACMAN_REPO,
    },
    status::{Code, Failure, Status},
    utils::{AlpmWrapper, CloneAur},
};
use std::{fs::OpenOptions, path::PathBuf};

pub fn init_aur_builder(args: InitAurBuilderArgs) -> Status {
    let InitAurBuilderArgs {} = args;

    let InitAurBuilder {
        global_settings,
        aur_package_names,
    } = InitAurBuilder::from_env().map_err(|error| {
        eprintln!("⮾ {}", error);
        Failure::from(Code::ManifestLoadingFailure)
    })?;

    let GlobalSettings {
        container,
        read_build_metadata,
        ..
    } = &global_settings;

    let manifest_file = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(BUILD_PACMAN_REPO)
        .map_err(|error| {
            eprintln!("⮾ {}", error);
            Failure::from(error)
        })?;

    let current_directory = PathBuf::from(".");

    let container = container
        .as_ref()
        .map(Wrapper::inner)
        .unwrap_or(&current_directory);

    let effect = CloneAur {
        container,
        package_names: aur_package_names.as_ref(),
        read_build_metadata: read_build_metadata.unwrap_or_default(),
        installed_dependencies: Default::default(),
        alpm: AlpmWrapper::from_env(),
    }
    .run();

    let mut error_count = effect.error_count;
    let members = effect
        .added_package_names
        .into_iter()
        .map(PathBuf::from)
        .map(Wrapper::from_inner)
        .map(|directory| OwnedMember {
            directory,
            ..Default::default()
        })
        .collect();

    let manifest_content = BuildPacmanRepo {
        global_settings,
        members,
    };

    if let Err(error) = serde_yaml::to_writer(manifest_file, &manifest_content) {
        eprintln!("⮾ {}", error);
        error_count += 1;
    }

    if error_count != 0 {
        eprintln!("{} error occurred", error_count);
        return Code::GenericFailure.into();
    }

    Ok(())
}
