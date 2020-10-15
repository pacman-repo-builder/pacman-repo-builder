use super::super::{
    args::{OutdatedArgs, OutdatedDetails},
    utils::{outdated_packages, DbInit, DbInitValue, PackageFileName},
};
use std::{fs::read_dir, path::PathBuf};

pub fn outdated(args: OutdatedArgs) -> i32 {
    let OutdatedArgs { details } = args;
    let details = details.unwrap_or_default();

    let mut db_init = DbInit::default();
    let DbInitValue {
        manifest,
        database,
        mut error_count,
    } = match db_init.init() {
        Err(error) => return error.code(),
        Ok(value) => value,
    };

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
        return 1;
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
            return 1;
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
        // TODO: Remove 'repository*' information
        match details {
            OutdatedDetails::PkgName => {
                println!("{}", pkgname);
            }
            OutdatedDetails::PkgFilePath => {
                println!("{}", directory.join(file_name).to_string_lossy());
            }
            OutdatedDetails::LossyYaml => {
                println!("---");
                println!("repository-file: {}", repository.to_string_lossy());
                println!("repository-directory: {}", directory.to_string_lossy());
                println!("file-name: {}", file_name);
                println!("pkgname: {}", pkgname);
                println!("version: {}", version);
                println!("arch: {}", arch);
            }
            OutdatedDetails::StrictYaml => {
                println!("---");
                println!("repository-file: {:?}", repository);
                println!("repository-directory: {:?}", directory);
                println!("file-name: {:?}", file_name);
                println!("pkgname: {:?}", pkgname);
                println!("version: {:?}", version);
                println!("arch: {:?}", arch);
            }
        }
    }

    if error_count == 0 {
        0
    } else {
        eprintln!("{} errors occurred", error_count);
        1
    }
}
