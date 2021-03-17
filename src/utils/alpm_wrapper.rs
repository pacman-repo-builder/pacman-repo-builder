use alpm::{Alpm, Error, SigLevel};
use pacman::pacman_conf::get_config;
use pipe_trait::Pipe;
use std::iter::once;

const DATABASE_PATH: &str = "/var/lib/pacman";

#[derive(Debug)]
pub struct AlpmWrapper {
    alpm: Alpm,
    loaded_packages: Vec<LoadedPackageParam>,
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
            loaded_packages: Default::default(),
        }
    }

    pub fn load_package(&mut self, filename: Vec<u8>) {
        self.loaded_packages.push(LoadedPackageParam { filename })
    }

    pub fn needed<'a>(&self, packages: impl Iterator<Item = &'a str>) -> InstallationPlan {
        let local_packages = || self.alpm.localdb().pkgs().into_iter();

        let wanted: Vec<String> = packages
            .filter(|target| {
                local_packages().all(|pkg| {
                    pkg.name() != *target
                        && pkg.provides().into_iter().all(|pkg| pkg.name() != *target)
                })
            })
            .map(ToString::to_string)
            .collect();

        let unwanted: Vec<String> = local_packages()
            .filter(|pkg| {
                pkg.conflicts()
                    .into_iter()
                    .any(|dep| wanted.iter().any(|pkgname| dep.name() == pkgname))
            })
            .map(|pkg| pkg.name().to_string())
            .collect();

        InstallationPlan { wanted, unwanted }
    }

    pub fn provides(&self, pkgname: &str) -> bool {
        let db_list = || {
            let local = self.alpm.localdb().pipe(once);
            let sync = self.alpm.syncdbs().into_iter();
            local.chain(sync)
        };

        for db in db_list() {
            match db.pkg(pkgname) {
                Ok(_) => return true,
                Err(Error::PkgNotFound) => continue,
                Err(error) => panic!("Cannot check {:?}: {}", pkgname, error),
            }
        }

        db_list()
            .flat_map(|db| db.pkgs())
            .flat_map(|pkg| pkg.provides())
            .any(|pkg| pkg.name() == pkgname)
    }
}

#[derive(Debug)]
struct LoadedPackageParam {
    filename: Vec<u8>,
}

#[derive(Debug)]
pub struct InstallationPlan {
    pub wanted: Vec<String>,
    pub unwanted: Vec<String>,
}
