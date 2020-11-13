use super::super::{
    args::{OutdatedArgs, OutdatedDetails},
    status::{Code, Failure, Status},
    utils::{load_failed_build_record, outdated_packages, DbInit, DbInitValue, PackageFileName},
};
use pipe_trait::*;
use std::{fs::read_dir, path::PathBuf};

pub fn outdated(args: OutdatedArgs) -> Status {
    let OutdatedArgs { details } = args;
    let details = details.unwrap_or_default();

    let mut db_init = DbInit::default();
    let DbInitValue {
        manifest,
        database,
        mut error_count,
    } = db_init.init()?;

    let arch_filter = manifest.global_settings.arch_filter.unwrap_or_default();

    let latest_packages: Vec<_> = database
        .package_file_base_names(|arch| arch_filter.test(arch))
        .filter_map(|item| match item {
            Err(error) => {
                eprintln!("⮾ Error in pkgbase of {}: {}", error.pkgbase, error.message);
                error_count += 1;
                None
            }
            Ok(value) => Some(value),
        })
        .collect();

    let repository = manifest.global_settings.repository.as_ref();
    let directory = if let Some(parent) = repository.parent() {
        parent
    } else {
        eprintln!("⮾ Repository cannot be a directory: {:?}", repository);
        return Code::GenericFailure.into();
    };

    // PROBLEM: read_dir cannot read "" as a directory
    // WORKAROUND: replace it with "."
    let valid_current_directory = PathBuf::from(".");
    let directory = if directory.as_os_str().is_empty() {
        &valid_current_directory
    } else {
        directory
    };

    let entries = match read_dir(directory) {
        Err(error) => {
            eprintln!("⮾ Cannot read {:?} as a directory: {}", directory, error,);
            return error.pipe(Failure::from).into();
        }
        Ok(entries) => entries,
    };

    let mut current_packages = Vec::new();
    for entry in entries {
        let file_name = match entry {
            Err(error) => {
                eprintln!(
                    "⮾ Cannot read an entry of directory {:?}: {}",
                    directory, error,
                );
                error_count += 1;
                continue;
            }
            Ok(entry) => entry.file_name(),
        };

        if let Some(name) = file_name.to_str() {
            current_packages.push(name.to_string())
        } else {
            eprintln!("cannot convert {:?} to UTF-8", file_name);
            error_count += 1;
        }
    }

    let failed_builds = manifest
        .global_settings
        .record_failed_builds
        .pipe_ref(load_failed_build_record)
        .map_err(|error| {
            eprintln!("⮾ {}", error);
            Failure::from(Code::FailedBuildRecordLoadingFailure)
        })?;

    for (
        ref file_name,
        PackageFileName {
            pkgname,
            version,
            arch,
        },
    ) in outdated_packages(&latest_packages, &current_packages, &failed_builds)
    {
        match details {
            OutdatedDetails::PkgName => {
                println!("{}", pkgname);
            }
            OutdatedDetails::PkgFilePath => {
                println!("{}", file_name);
            }
            OutdatedDetails::LossyYaml => {
                println!("---");
                println!("file-name: {}", file_name);
                println!("pkgname: {}", pkgname);
                println!("version: {}", version);
                println!("arch: {}", arch);
            }
            OutdatedDetails::StrictYaml => {
                println!("---");
                println!("file-name: {:?}", file_name);
                println!("pkgname: {:?}", pkgname);
                println!("version: {:?}", version);
                println!("arch: {:?}", arch);
            }
        }
    }

    if error_count == 0 {
        Ok(())
    } else {
        eprintln!("{} errors occurred", error_count);
        Code::GenericFailure.into()
    }
}
