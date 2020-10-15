use super::super::{
    args::BuildArgs,
    manifest::Member,
    srcinfo::database::DatabaseValue,
    utils::{create_makepkg_command, CommandUtils, DbInit, DbInitValue},
};
use command_extra::CommandExtra;
use std::{
    fs::copy,
    process::{Command, Stdio},
};

pub fn build(args: BuildArgs) -> i32 {
    let BuildArgs {
        syncdeps,
        force,
        pacman,
        log_dest,
        packager,
    } = args;

    let makepkg = || {
        create_makepkg_command()
            .with_arg("--install")
            .with_arg("--noconfirm")
            .with_arg("--asdeps")
            .arg_if("--syncdeps", syncdeps)
            .arg_if("--force", force)
            .may_env("PACMAN", pacman.as_ref())
            .may_env("LOGDEST", log_dest.as_ref())
            .may_env("PACKAGER", packager.as_ref())
    };

    let mut db_init = DbInit::default();
    let DbInitValue {
        database,
        error_count,
        manifest,
    } = match db_init.init() {
        Err(error) => return error.code(),
        Ok(value) => value,
    };

    if error_count != 0 {
        eprintln!("{} error occurred", error_count);
        return 1;
    }

    let build_order = match database.build_order() {
        Ok(build_order) => build_order,
        Err(error) => {
            eprintln!("{}", error);
            return 1;
        }
    };

    let repository = manifest.global_settings.repository.as_path();
    let members: Vec<_> = manifest.resolve_members().collect();

    for pkgbase in build_order {
        let DatabaseValue {
            directory, srcinfo, ..
        } = database.pkgbase().get(pkgbase).unwrap_or_else(|| {
            dbg!(pkgbase);
            panic!("cannot lookup value")
        });

        let Member { directory, .. } = members
            .iter()
            .find(|member| member.directory.as_path() == *directory)
            .unwrap_or_else(|| {
                dbg!(pkgbase, directory);
                panic!("cannot lookup member");
            });

        eprintln!();
        eprintln!();
        eprintln!("==== PACKAGE ====");
        eprintln!();
        eprintln!("🛈 pkgbase:           {}", pkgbase);
        for pkgname in srcinfo.pkgname() {
            eprintln!("🛈 pkgname:           {}", pkgname);
        }
        eprintln!("🛈 source directory:  {}", directory.to_string_lossy());
        eprintln!("🛈 target repository: {}", repository.to_string_lossy());
        eprintln!();

        let status = match makepkg()
            .with_current_dir(directory)
            .with_stdin(Stdio::null())
            .with_stdout(Stdio::inherit())
            .with_stderr(Stdio::inherit())
            .spawn()
            .and_then(|mut child| child.wait())
        {
            Ok(status) => status.code().unwrap_or(1),
            Err(error) => {
                eprintln!("{}", error);
                return 1;
            }
        };

        if status != 0 {
            eprintln!("⮾ makepkg exits with non-zero status code: {}", status);
            return status;
        }

        for package_name in srcinfo
            .package_file_base_names()
            .expect("get package file base names")
        {
            let package_name = &package_name.to_string();
            let package_path = &directory.join(package_name);
            eprintln!("📦 made file {}", package_name);
            {
                let repository_directory = repository.parent().expect("get repository directory");
                eprintln!("  → copy to {}/", repository_directory.to_string_lossy());
                if let Err(error) = copy(package_path, repository_directory.join(package_name)) {
                    eprintln!("⮾ {}", error);
                    return error.raw_os_error().unwrap_or(1);
                }
            }

            {
                eprintln!("  → add to {}", repository.to_string_lossy());
                let status = match Command::new("repo-add")
                    .with_arg("--quiet")
                    .with_arg("--nocolor")
                    .with_arg(repository)
                    .with_arg(package_path)
                    .stdin(Stdio::null())
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .and_then(|mut child| child.wait())
                {
                    Ok(status) => status.code().unwrap_or(1),
                    Err(error) => {
                        eprintln!("{}", error);
                        return error.raw_os_error().unwrap_or(1);
                    }
                };
                if status != 0 {
                    eprintln!("⮾ repo-add exits with non-zero status code: {}", status);
                    return status;
                }
            }
        }
    }

    0
}
