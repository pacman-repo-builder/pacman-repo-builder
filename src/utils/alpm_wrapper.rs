use alpm::{Alpm, Error, SigLevel};
use pacman::pacman_conf::get_config;

const DATABASE_PATH: &str = "/var/lib/pacman";

#[derive(Debug)]
pub struct AlpmWrapper {
    alpm: Alpm,
}

impl AlpmWrapper {
    pub fn from_env() -> Self {
        let alpm = Alpm::new("/", DATABASE_PATH).expect("get alpm database");
        for repo in get_config().repos {
            alpm.register_syncdb(repo.name, SigLevel::NONE)
                .expect("register syncdb");
        }
        AlpmWrapper { alpm }
    }

    pub fn provides(&self, pkgname: &str) -> bool {
        let by_name = self
            .alpm
            .syncdbs()
            .into_iter()
            .any(|db| match db.pkg(pkgname) {
                Ok(_) => true,
                Err(Error::PkgNotFound) => false,
                Err(error) => panic!("Cannot check {:?}: {}", pkgname, error),
            });

        if by_name {
            return true;
        }

        self.alpm
            .syncdbs()
            .into_iter()
            .flat_map(|db| db.pkgs())
            .flat_map(|pkg| pkg.provides())
            .any(|pkg| pkg.name() == pkgname)
    }
}
