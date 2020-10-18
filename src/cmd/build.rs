use super::super::{
    args::BuildArgs,
    manifest::Member,
    srcinfo::database::DatabaseValue,
    status::{Code, Failure, Status},
    utils::{create_makepkg_command, CommandUtils, DbInit, DbInitValue},
};
use command_extra::CommandExtra;
use pipe_trait::*;
use std::{
    fs::{canonicalize, copy, read_dir, remove_file},
    process::{Command, Stdio},
};

pub fn build(args: BuildArgs) -> Status {
    let BuildArgs {
        syncdeps,
        force,
        pacman,
        log_dest,
        packager,
        deref_db,
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
    } = db_init.init()?;

    if error_count != 0 {
        eprintln!("{} error occurred", error_count);
        return Code::GenericFailure.into();
    }

    let build_order = match database.build_order() {
        Ok(build_order) => build_order,
        Err(error) => {
            eprintln!("⮾ {}", error);
            return error.code().into();
        }
    };

    let repository = manifest.global_settings.repository.as_path();
    let repository_directory = repository.parent().expect("get repository directory");
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

        let future_package_files: Vec<_> = srcinfo
            .package_file_base_names()
            .expect("get future package file base names")
            .map(|name| repository_directory.join(name.to_string()))
            .collect();

        if !force && future_package_files.iter().all(|path| path.exists()) {
            eprintln!("🛈 All packages are already built. Skip.");

            let status = pacman
                .as_deref()
                .unwrap_or("pacman")
                .pipe(Command::new)
                .with_arg("--upgrade")
                .with_args(future_package_files)
                .with_arg("--noconfirm")
                .with_arg("--asdeps")
                .spawn()
                .and_then(|mut child| child.wait())
                .map_err(|error| {
                    eprintln!("⮾ {}", error);
                    Failure::from(error)
                })?
                .code()
                .unwrap_or(1);
            if status != 0 {
                eprintln!("⮾ pacman -U exits with non-zero status code: {}", status);
                return Ok(status);
            }

            continue;
        }

        let status = makepkg()
            .with_current_dir(directory)
            .with_stdin(Stdio::null())
            .with_stdout(Stdio::inherit())
            .with_stderr(Stdio::inherit())
            .spawn()
            .and_then(|mut child| child.wait())
            .map_err(|error| {
                eprintln!("⮾ {}", error);
                Failure::from(error)
            })?
            .code()
            .unwrap_or(1);

        if status != 0 {
            eprintln!("⮾ makepkg exits with non-zero status code: {}", status);
            return Ok(status);
        }

        for package_name in srcinfo
            .package_file_base_names()
            .expect("get package file base names")
        {
            let package_name = &package_name.to_string();
            eprintln!("📦 made file {}", package_name);
            {
                eprintln!("  → copy to {}/", repository_directory.to_string_lossy());
                if let Err(error) = copy(
                    directory.join(package_name),
                    repository_directory.join(package_name),
                ) {
                    eprintln!("⮾ {}", error);
                    return error.pipe(Failure::from).into();
                }
            }

            {
                eprintln!("  → add to {}", repository.to_string_lossy());
                let status = Command::new("repo-add")
                    .with_arg("--quiet")
                    .with_arg("--nocolor")
                    .with_arg(repository)
                    .with_arg(repository_directory.join(package_name))
                    .with_stdin(Stdio::null())
                    .with_stdout(Stdio::inherit())
                    .with_stderr(Stdio::inherit())
                    .spawn()
                    .and_then(|mut child| child.wait())
                    .map_err(|error| {
                        eprintln!("⮾ {}", error);
                        Failure::from(error)
                    })?
                    .code()
                    .unwrap_or(1);
                if status != 0 {
                    eprintln!("⮾ repo-add exits with non-zero status code: {}", status);
                    return Ok(status);
                }
            }
        }
    }

    if deref_db {
        eprintln!("Resolving all symlinks to repository database into real files");
        let canon_repository = canonicalize(repository).expect("canonicalize repository directory");
        for entry in read_dir(repository_directory).expect("read repository repository") {
            let entry = entry.expect("read entry");
            if !entry
                .file_type()
                .map(|kind| kind.is_symlink())
                .unwrap_or(false)
            {
                continue;
            }
            let file_name = entry.file_name();
            if !file_name.to_string_lossy().ends_with(".db") {
                continue;
            }
            let canon_target = canon_repository
                .join(file_name)
                .pipe(canonicalize)
                .expect("canonicalize suspect");
            let canon_repository = canonicalize(repository).expect("canonicalize repository file");
            if canon_target != canon_repository {
                continue;
            }
            eprintln!("  → Delete {:?}", &canon_target);
            remove_file(&canon_target).map_err(Failure::from)?;
            eprintln!("  → Copy {:?} to {:?}", canon_repository, &canon_target);
            copy(canon_repository, canon_target).map_err(Failure::from)?;
        }
    }

    Ok(0)
}
