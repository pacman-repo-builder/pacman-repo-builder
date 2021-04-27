use alpm::{Alpm, Db, Package, SigLevel};
use pipe_trait::Pipe;
use std::iter::once;

const DATABASE_PATH: &str = "/var/lib/pacman";

#[derive(Debug)]
pub struct AlpmWrapper {
    pub(super) alpm: Alpm,
}

impl AlpmWrapper {
    pub fn from_env() -> Self {
        let alpm = Alpm::new("/", DATABASE_PATH).expect("get alpm database");
        let pacman = pacmanconf::Config::new().expect("failed to read pacman.conf");

        for repo in pacman.repos {
            alpm.register_syncdb(repo.name, SigLevel::NONE)
                .expect("register syncdb");
        }
        AlpmWrapper { alpm }
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
