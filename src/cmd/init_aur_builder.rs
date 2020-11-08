use super::super::{
    args::InitAurBuilderArgs,
    manifest::{GlobalSettings, InitAurBuilder, Wrapper},
    status::{Code, Failure, Status},
    utils::{list_all_native_packages, CloneAur},
};
use pipe_trait::*;
use std::path::PathBuf;

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
        pacman,
        read_build_metadata,
        ..
    } = &global_settings;

    let current_directory = PathBuf::from(".");

    let container = container
        .as_ref()
        .map(Wrapper::inner)
        .unwrap_or(&current_directory);

    let native_packages = match pacman
        .as_ref()
        .map(AsRef::as_ref)
        .unwrap_or("pacman")
        .pipe(list_all_native_packages)
    {
        Ok(native_packages) => native_packages,
        Err(status) => return status,
    };

    CloneAur {
        container,
        native_packages: &native_packages,
        package_names: aur_package_names.as_ref(),
        read_build_metadata: read_build_metadata.unwrap_or_default(),
        installed_dependencies: Default::default(),
    }
    .run();

    unimplemented!()
}
