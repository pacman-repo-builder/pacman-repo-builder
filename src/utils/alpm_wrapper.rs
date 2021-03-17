use alpm::{Alpm, Db, SigLevel};
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
            .filter(|pkgname| !self.is_installed(pkgname))
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
        self.is_installed(pkgname) || self.is_available(pkgname)
    }

    fn is_installed(&self, pkgname: &str) -> bool {
        db_list_provides(self.alpm.localdb().pipe(once), pkgname)
    }

    fn is_available(&self, pkgname: &str) -> bool {
        db_list_provides(self.alpm.syncdbs(), pkgname)
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

fn db_list_provides<'a>(db_list: impl IntoIterator<Item = Db<'a>>, pkgname: &str) -> bool {
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
