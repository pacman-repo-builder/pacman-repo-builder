use super::super::{
    args::BuildArgs,
    manifest::{GlobalSettings, Member, TriState},
    srcinfo::database::DatabaseValue,
    status::{status_of_code, Code, Failure, Status},
    utils::{
        create_makepkg_command, load_failed_build_record, run_deref_db, AlpmWrapper, CommandUtils,
        DbInit, DbInitValue, InstallationPlan, PackageFileName,
    },
};
use command_extra::CommandExtra;
use itertools::Itertools;
use pipe_trait::*;
use std::{
    ffi::OsString,
    fs::{copy, remove_file, write},
    os::unix::prelude::{OsStrExt, OsStringExt},
    path::Path,
    process::{Command, Stdio},
};

pub fn build(args: BuildArgs) -> Status {
    let BuildArgs {} = args;

    let mut db_init = DbInit::default();
    let DbInitValue {
        database,
        error_count,
        manifest,
    } = db_init.init()?;

    let GlobalSettings {
        record_failed_builds,
        packager,
        dereference_database_symlinks,
        arch_filter,
        ..
    } = &manifest.global_settings;
    let packager: Option<&str> = packager.as_ref().map(AsRef::as_ref);
    let dereference_database_symlinks = dereference_database_symlinks.unwrap_or(false);
    let default_arch_filter = Default::default();
    let arch_filter = arch_filter.as_ref().unwrap_or(&default_arch_filter);

    let failed_build_record = load_failed_build_record(record_failed_builds).map_err(|error| {
        eprintln!("⮾ {}", error);
        Failure::from(Code::FailedBuildRecordLoadingFailure)
    })?;

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

    let repository = manifest.global_settings.repository.as_ref();
    let repository_directory = repository.parent().expect("get repository directory");
    let members: Vec<_> = manifest.resolve_members().collect();
    let mut failed_builds = Vec::new();
    let mut alpm_wrapper = AlpmWrapper::from_env();

    for pkgbase in build_order {
        let DatabaseValue {
            directory, srcinfo, ..
        } = database.pkgbase().get(pkgbase).unwrap_or_else(|| {
            dbg!(pkgbase);
            panic!("cannot lookup value")
        });

        let Member {
            directory,
            install_missing_dependencies,
            clean_before_build,
            clean_after_build,
            force_rebuild,
            check,
            pacman,
            allow_failure,
            ..
        } = members
            .iter()
            .find(|member| member.directory.as_ref() == *directory)
            .unwrap_or_else(|| {
                dbg!(pkgbase, directory);
                panic!("cannot lookup member");
            });

        let directory: &Path = directory.as_ref();
        let force_rebuild = force_rebuild.unwrap_or(false);
        let check = check.unwrap_or(TriState::Inherit);
        let install_missing_dependencies = install_missing_dependencies.unwrap_or(false);
        let clean_before_build = clean_before_build.unwrap_or(false);
        let clean_after_build = clean_after_build.unwrap_or(false);
        let pacman: Option<&str> = pacman.as_ref().map(AsRef::as_ref);
        let allow_failure = allow_failure.unwrap_or(false);

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

        let future_package_file_base_names: Vec<_> = srcinfo
            .package_file_base_names(|arch| arch_filter.test(arch))
            .expect("get future package file base names")
            .collect();

        let future_package_file_paths: Vec<_> = future_package_file_base_names
            .iter()
            .map(|name| repository_directory.join(name.to_string()))
            .collect();

        if !force_rebuild && future_package_file_paths.iter().all(|path| path.exists()) {
            eprintln!("🛈 All packages are already built. Skip.");

            let status = pacman
                .unwrap_or("pacman")
                .pipe(Command::new)
                .with_arg("--upgrade")
                .with_args(future_package_file_paths)
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
                return status_of_code(status);
            }

            continue;
        }

        let is_failed = |name: &PackageFileName<&str, String, &str>| {
            failed_build_record.iter().any(|x| {
                name.pkgname == x.pkgname && name.arch == x.arch && name.version == x.version
            })
        };
        if !force_rebuild && future_package_file_base_names.iter().all(is_failed) {
            eprintln!("⚠ Failures had been recorded. Skip.");
            continue;
        }

        {
            eprintln!("🛈 Checking for missing dependencies...");
            let InstallationPlan { wanted, unwanted } = alpm_wrapper.needed(
                srcinfo.all_required_dependencies().map(|x| x.name),
                srcinfo.conflicts().map(|x| x.name),
            );
            let has_wanted = !wanted.is_empty();
            let has_unwanted = !unwanted.is_empty();
            if has_wanted {
                eprintln!(
                    "🛈 Missing dependencies: {:?}",
                    wanted.iter().map(|target| &target.name).join(" ")
                );
            }
            if has_unwanted {
                eprintln!("🛈 Conflicts: {:?}", unwanted.iter().join(" "));
            }

            macro_rules! spawn_and_warn {
                ($short:literal, $command:expr) => {
                    match $command
                        .spawn()
                        .and_then(|mut child| child.wait())
                        .map(|status| status.code().unwrap_or(1))
                    {
                        Ok(0) => {}
                        Ok(status) => {
                            debug_assert_ne!(status, 0);
                            eprintln!(
                                "⚠ pacman {} exits with non-zero status code: {}",
                                $short, status,
                            );
                        }
                        Err(error) => {
                            eprintln!("⚠ {}", error);
                        }
                    }
                };
            }

            if install_missing_dependencies && has_wanted {
                macro_rules! run_pacman {
                    ($long:literal, $short:literal, $target:expr) => {
                        spawn_and_warn!(
                            $short,
                            pacman
                                .unwrap_or("pacman")
                                .pipe(Command::new)
                                .with_arg($long)
                                .with_args($target)
                                .with_arg("--noconfirm")
                                .with_arg("--asdeps")
                                .with_arg("--needed")
                        );
                    };
                }

                let (upgrade_targets, sync_targets): (Vec<_>, Vec<_>) = wanted
                    .into_iter()
                    .partition(|target| target.external.is_some());

                if !upgrade_targets.is_empty() {
                    eprintln!("🛈 Installing missing dependencies from created package files...");
                    let upgrade_targets = upgrade_targets
                        .into_iter()
                        .flat_map(|target| target.external)
                        .map(OsString::from_vec);
                    run_pacman!("--upgrade", "-U", upgrade_targets);
                }

                if !sync_targets.is_empty() {
                    eprintln!("🛈 Installing missing dependencies from sync database...");
                    let sync_targets = sync_targets.into_iter().map(|target| target.name);
                    run_pacman!("--sync", "-S", sync_targets);
                }
            }

            if install_missing_dependencies && has_unwanted {
                eprintln!("🛈 Removing conflicts...");
                spawn_and_warn!(
                    "-R",
                    pacman
                        .unwrap_or("pacman")
                        .pipe(Command::new)
                        .with_arg("--remove")
                        .with_args(unwanted)
                        .with_arg("--unneeded")
                        .with_arg("--assumed-installed")
                        .with_arg("--noconfirm")
                );
            }
        }

        let mut build_failed = false;
        for arch in srcinfo.arch() {
            if !arch_filter.test(arch) {
                eprintln!("🛈 Skip architecture {}.", arch);
                continue;
            }

            eprintln!("🛈 Building for architecture {}...", arch);

            let status = create_makepkg_command()
                .with_arg("--install")
                .with_arg("--noconfirm")
                .with_arg("--asdeps")
                .arg_if("--syncdeps", install_missing_dependencies)
                .arg_if("--clean", clean_after_build)
                .arg_if("--cleanbuild", clean_before_build)
                .arg_if("--force", force_rebuild)
                .arg_if("--check", check == TriState::Enabled)
                .arg_if("--nocheck", check == TriState::Disabled)
                .may_env("PACMAN", pacman)
                .may_env("PACKAGER", packager)
                .with_env("CARCH", arch)
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
                build_failed = true;

                if allow_failure {
                    eprintln!("⚠ makepkg exits with non-zero status code: {}", status);
                    eprintln!("⚠ skip {}", pkgbase);
                    continue;
                } else {
                    eprintln!("⮾ makepkg exits with non-zero status code: {}", status);
                    return status_of_code(status);
                }
            }
        }

        if build_failed {
            failed_builds.push((*pkgbase, directory, future_package_file_base_names));
        }

        for pkg_file_name in srcinfo
            .package_file_base_names(|arch| arch_filter.test(arch))
            .expect("get package file base names")
        {
            let pkg_file_name = &pkg_file_name.to_string();
            let pkg_src_file = directory.join(pkg_file_name);
            let pkg_dst_file = repository_directory.join(pkg_file_name);

            if !pkg_src_file.exists() {
                eprintln!("⚠ File {:?} does not exist. Skip.", &pkg_src_file);
                continue;
            }

            eprintln!("📦 made file {}", pkg_file_name);

            {
                alpm_wrapper.add_external_package(pkg_src_file.as_os_str().as_bytes().to_vec());
            }

            {
                eprintln!("  → copy to {}/", repository_directory.to_string_lossy());
                if let Err(error) = copy(&pkg_src_file, pkg_dst_file) {
                    eprintln!("⮾ {}", error);
                    return error.pipe(Failure::from).into();
                }
            }

            if clean_after_build {
                eprintln!("  → clean");
                if let Err(error) = remove_file(pkg_src_file) {
                    eprintln!("⚠ {}", error);
                }
            }

            {
                eprintln!("  → add to {}", repository.to_string_lossy());
                let status = Command::new("repo-add")
                    .with_arg("--quiet")
                    .with_arg("--nocolor")
                    .with_arg(repository)
                    .with_arg(repository_directory.join(pkg_file_name))
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
                    return status_of_code(status);
                }
            }
        }
    }

    if dereference_database_symlinks {
        eprintln!();
        eprintln!();
        eprintln!("🛈 Resolving all symlinks to repository database into real files");
        run_deref_db(repository_directory).map_err(|error| {
            eprintln!("⮾ {}", error);
            Failure::from(error)
        })?;
    }

    if !failed_builds.is_empty() {
        eprintln!();
        eprintln!();
        eprintln!("🛈 Some builds failed:");
        for (pkgbase, directory, _) in &failed_builds {
            eprintln!("  ● {} ({})", pkgbase, directory.to_string_lossy());
        }

        if let Some(record_path) = record_failed_builds {
            let mut failed_build_record = failed_build_record;
            for (_, _, record) in failed_builds {
                for PackageFileName {
                    pkgname,
                    version,
                    arch,
                } in record
                {
                    failed_build_record.push(PackageFileName {
                        pkgname: pkgname.to_string(),
                        version,
                        arch: arch.to_string(),
                    });
                }
            }

            let content = serde_yaml::to_string(&failed_build_record).unwrap();
            write(record_path.as_ref(), content).map_err(|error| {
                eprintln!("⮾ {}", error);
                Failure::from(Code::FailedBuildRecordWritingFailure)
            })?;
        }
    }

    Ok(())
}
