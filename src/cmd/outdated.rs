use super::super::{
    args::OutdatedArgs,
    manifest::Repository,
    utils::{outdated_packages, DbInit, DbInitValue, PackageFileName},
};
use std::fs::read_dir;

pub fn outdated(args: OutdatedArgs) -> i32 {
    let OutdatedArgs {} = args;

    let mut srcinfo_texts = Default::default();
    let mut srcinfo_collection = Default::default();
    let DbInitValue {
        manifest,
        database,
        mut error_count,
    } = match (DbInit {
        srcinfo_texts: &mut srcinfo_texts,
        srcinfo_collection: &mut srcinfo_collection,
    })
    .init()
    {
        Err(error) => return error.code(),
        Ok(value) => value,
    };

    let repositories = manifest
        .resolve_members()
        .filter_map(|member| {
            if let Some(repository) = member.repository {
                Some(repository)
            } else {
                eprintln!(
                    "(warning) a member with directory {:?} has no repositories",
                    member.directory
                );
                None
            }
        })
        .flat_map(|repository| match repository {
            Repository::Single(path) => vec![path],
            Repository::Multiple(paths) => paths,
        });

    let latest_packages: Vec<_> = database
        .package_file_base_names()
        .filter_map(|item| match item {
            Err(error) => {
                eprintln!("error in pkgbase of {}: {}", error.pkgbase, error.message);
                error_count += 1;
                None
            }
            Ok(value) => Some(value),
        })
        .collect();

    let mut current_packages = Vec::new();
    for repository in repositories {
        let directory = if let Some(parent) = repository.parent() {
            parent
        } else {
            eprintln!("repository cannot be a directory: {:?}", repository);
            error_count += 1;
            continue;
        };

        let entries = match read_dir(directory) {
            Err(error) => {
                eprintln!("cannot read {:?} as a directory: {}", directory, error);
                error_count += 1;
                continue;
            }
            Ok(entries) => entries,
        };

        for entry in entries {
            let file_name = match entry {
                Err(error) => {
                    eprintln!(
                        "cannot read an entry of directory {:?}: {}",
                        directory, error
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
    }

    for (
        file_name,
        PackageFileName {
            pkgname,
            version,
            arch,
        },
    ) in outdated_packages(latest_packages, &current_packages)
    {
        println!("---");
        println!("file-name: {}", file_name);
        println!("pkgname: {}", pkgname);
        println!("version: {}", version);
        println!("arch: {}", arch);
    }

    if error_count == 0 {
        0
    } else {
        eprintln!("{} errors occurred", error_count);
        1
    }
}
