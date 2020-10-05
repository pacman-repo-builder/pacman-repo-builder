use super::super::{
    args::OutdatedArgs,
    manifest::{Manifest, Repository},
    srcinfo::{database::SimpleDatabase, SrcInfo},
    utils::{outdated_packages, read_srcinfo_texts, PackageFileName},
};
use std::fs::read_dir;

pub fn outdated(args: OutdatedArgs) -> i32 {
    let mut error_count = 0u32;

    let OutdatedArgs {} = args;
    let manifest = match Manifest::from_env() {
        Err(error) => {
            eprintln!("{}", error);
            return 2;
        }
        Ok(manifest) => manifest,
    };

    let srcinfo_texts = read_srcinfo_texts(&manifest, |error| {
        eprintln!("{}", error);
        error_count += 1;
    });

    let srcinfo_collection: Vec<_> = srcinfo_texts
        .iter()
        .map(|x| x.to_ref().map(String::as_str).map(SrcInfo))
        .collect();
    let mut database = SimpleDatabase::default();
    for pair in &srcinfo_collection {
        let (srcinfo, directory) = pair.to_ref().into_tuple();
        if let Err(error) = database.insert_srcinfo(srcinfo) {
            eprintln!("error in directory {:?}: {}", directory, error);
            error_count += 1;
        }
    }

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
