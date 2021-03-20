use super::AlpmWrapper;
use alpm::{PackageReason, SigLevel};
use indexmap::IndexSet;
use std::{ffi::OsStr, ops::Deref, os::unix::prelude::OsStrExt};

#[derive(Debug)]
pub struct ExternalPackageList {
    pkg_load_params: Vec<LoadedPackageParam>,
    alpm_wrapper: AlpmWrapper,
}

impl ExternalPackageList {
    pub fn from_env() -> Self {
        ExternalPackageList {
            pkg_load_params: Vec::new(),
            alpm_wrapper: AlpmWrapper::from_env(),
        }
    }

    pub fn add_external_package(&mut self, filename: Vec<u8>) {
        self.pkg_load_params.push(LoadedPackageParam { filename })
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
                .pkg_load_params
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
                    .pkg_load_params
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
            .filter(|pkgname| {
                // NOTE: do not use self.is_installed since it also includes providers
                // NOTE: do not add explicitly installed packages to unwanted
                // TODO: only add orphan packages (direct or indirect) to unwanted

                self.installed_packages()
                    .find(|pkg| pkg.name() == *pkgname)
                    .map(|pkg| pkg.reason() == PackageReason::Depend)
                    .unwrap_or(false)
            })
            .map(ToString::to_string);

        let unwanted: IndexSet<String> = left_unwanted.chain(right_unwanted).collect();

        InstallationPlan { wanted, unwanted }
    }
}

impl Deref for ExternalPackageList {
    type Target = AlpmWrapper;

    fn deref(&self) -> &Self::Target {
        &self.alpm_wrapper
    }
}

#[derive(Debug)]
pub struct LoadedPackageParam {
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
