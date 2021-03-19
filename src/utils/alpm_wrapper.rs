use alpm::{Alpm, Db, Package, SigLevel};
use indexmap::IndexSet;
use pacman::pacman_conf::get_config;
use pipe_trait::Pipe;
use std::{ffi::OsStr, iter::once, os::unix::prelude::OsStrExt};

const DATABASE_PATH: &str = "/var/lib/pacman";

#[derive(Debug)]
pub struct AlpmWrapper {
    alpm: Alpm,
    external_packages: Vec<LoadedPackageParam>,
}

impl AlpmWrapper {
    pub fn from_env() -> Self {
        let alpm = Alpm::new("/", DATABASE_PATH).expect("get alpm database");
        for repo in get_config().repos {
            alpm.register_syncdb(repo.name, SigLevel::NONE)
                .expect("register syncdb");
        }
        AlpmWrapper {
            alpm,
            external_packages: Default::default(),
        }
    }

    pub fn add_external_package(&mut self, filename: Vec<u8>) {
        self.external_packages.push(LoadedPackageParam { filename })
    }

    pub fn needed<'a>(
        &self,
        srcinfo_all_depends: impl Iterator<Item = &'a str>,
        srcinfo_conflicts: impl Iterator<Item = &'a str>,
    ) -> InstallationPlan {
        // TODO: consider version ranges (how to check version satisfaction?)

        let make_installation_target = |name: String| {
            if self.is_available(&name) {
                return InstallationTarget {
                    name,
                    external: None,
                };
            }

            let external = self
                .external_packages
                .iter()
                .find(|param| {
                    match self
                        .alpm
                        .pkg_load(param.filename.clone(), true, SigLevel::NONE)
                    {
                        Err(error) => {
                            eprintln!(
                                "⚠ Failed to load {:?} as an alpm package: {}",
                                OsStr::from_bytes(&param.filename),
                                error,
                            );
                            false
                        }
                        Ok(pkg) => pkg.name() == name,
                    }
                })
                .map(|param| param.filename.clone());

            InstallationTarget { name, external }
        };

        let mut wanted: IndexSet<InstallationTarget> = srcinfo_all_depends
            .filter(|pkgname| !self.is_installed(pkgname))
            .map(ToString::to_string)
            .map(make_installation_target)
            .collect();

        // Q: Why also add indirect dependencies?
        // A: To enable finding all possible conflicts later.
        let addend: Vec<InstallationTarget> = wanted
            .iter()
            .flat_map(|InstallationTarget { name, .. }| -> Vec<String> {
                macro_rules! find_pkg {
                    ($list:expr) => {{
                        let find_by_name = || $list.find(|pkg| pkg.name() == name);
                        let find_by_provider = || {
                            $list.find(|pkg| {
                                pkg.provides().into_iter().any(|dep| dep.name() == name)
                            })
                        };
                        find_by_name().or_else(find_by_provider)
                    }};
                }

                macro_rules! get_result {
                    ($pkg:expr) => {
                        $pkg.depends()
                            .into_iter()
                            .chain($pkg.makedepends())
                            .chain($pkg.checkdepends())
                            .map(|pkg| pkg.name())
                            .filter(|pkgname| !self.is_installed(pkgname))
                            .map(ToString::to_string)
                            .collect()
                    };
                }

                if let Some(pkg) = find_pkg!(self.available_packages()) {
                    return get_result!(pkg);
                }

                let external_packages: Vec<_> = self
                    .external_packages
                    .iter()
                    .filter_map(|LoadedPackageParam { filename }| {
                        match self.alpm.pkg_load(filename.clone(), true, SigLevel::NONE) {
                            Err(error) => {
                                eprintln!(
                                    "⚠ Failed to load {:?} as an alpm package: {}",
                                    OsStr::from_bytes(&filename),
                                    error,
                                );
                                None
                            }
                            Ok(pkg) => Some(pkg),
                        }
                    })
                    .collect();

                if let Some(pkg) = find_pkg!(external_packages.iter()) {
                    return get_result!(pkg);
                }

                Vec::new()
            })
            .map(make_installation_target)
            .collect();

        wanted.extend(addend);

        let left_unwanted = self
            .installed_packages()
            .filter(|pkg| {
                pkg.conflicts()
                    .into_iter()
                    .any(|dep| wanted.iter().any(|target| dep.name() == target.name))
            })
            .map(|pkg| pkg.name().to_string());

        let right_unwanted = srcinfo_conflicts
            .filter(|pkgname| self.is_installed(pkgname))
            .map(ToString::to_string);

        let unwanted: IndexSet<String> = left_unwanted.chain(right_unwanted).collect();

        InstallationPlan { wanted, unwanted }
    }

    pub fn is_provided(&self, pkgname: &str) -> bool {
        self.is_installed(pkgname) || self.is_available(pkgname)
    }

    pub fn is_installed(&self, pkgname: &str) -> bool {
        does_db_list_provide(self.alpm.localdb().pipe(once), pkgname)
    }

    pub fn is_available(&self, pkgname: &str) -> bool {
        does_db_list_provide(self.alpm.syncdbs(), pkgname)
    }

    pub fn installed_packages(&self) -> impl Iterator<Item = Package<'_>> {
        self.alpm.localdb().pkgs().into_iter()
    }

    pub fn available_packages(&self) -> impl Iterator<Item = Package<'_>> {
        self.alpm.syncdbs().into_iter().flat_map(|db| db.pkgs())
    }
}

#[derive(Debug)]
struct LoadedPackageParam {
    filename: Vec<u8>,
}

#[derive(Debug)]
pub struct InstallationPlan {
    pub wanted: IndexSet<InstallationTarget>,
    pub unwanted: IndexSet<String>,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstallationTarget {
    pub name: String,
    pub external: Option<Vec<u8>>,
}

fn does_db_list_provide<'a>(db_list: impl IntoIterator<Item = Db<'a>>, pkgname: &str) -> bool {
    db_list
        .into_iter()
        .flat_map(|db| db.pkgs())
        .map(|pkg| {
            (
                pkg.name().pipe(once),
                pkg.provides().into_iter().map(|target| target.name()),
            )
        })
        .flat_map(|(names, provides)| names.chain(provides))
        .any(|name| name == pkgname)
}
