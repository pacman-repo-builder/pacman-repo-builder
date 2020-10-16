use super::super::{
    args::{OutdatedArgs, OutdatedDetails},
    status::{Code, Failure, Status},
    utils::{outdated_packages, DbInit, DbInitValue, PackageFileName},
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

    let latest_packages: Vec<_> = database
        .package_file_base_names()
        .filter_map(|item| match item {
            Err(error) => {
                eprintln!("⮾ Error in pkgbase of {}: {}", error.pkgbase, error.message);
                error_count += 1;
                None
            }
            Ok(value) => Some(value),
        })
        .collect();

    let repository = manifest.global_settings.repository.as_path();
    let directory = if let Some(parent) = repository.parent() {
        parent
    } else {
        eprintln!("⮾ Repository cannot be a directory: {:?}", repository);
        return Code::GenericFailure.pipe(Failure::Expected).pipe(Err);
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
            return Code::GenericFailure.pipe(Failure::Expected).pipe(Err);
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

    for (
        ref file_name,
        PackageFileName {
            pkgname,
            version,
            arch,
        },
    ) in outdated_packages(&latest_packages, &current_packages)
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
        Ok(0)
    } else {
        eprintln!("{} errors occurred", error_count);
        Code::GenericFailure.pipe(Failure::Expected).pipe(Err)
    }
}
