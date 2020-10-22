mod build_metadata;
mod global_settings;
mod member;
mod wrapper;

pub use build_metadata::BuildMetadata;
pub use global_settings::{BorrowedGlobalSettings, GlobalSettings, OwnedGlobalSettings};
pub use member::{BorrowedMember, Member, OwnedMember};
pub use wrapper::{
    Associations, BorrowedContainer, BorrowedDirectory, BorrowedRepository, Container, Directory,
    OwnedContainer, OwnedDirectory, OwnedRepository, Repository, Wrapper,
};

use pipe_trait::*;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::ErrorKind, path::Path};

pub const MANIFEST_BASENAME: &str = "build-pacman-repo.yaml";

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Manifest<Repository, Container, Directory>
where
    Repository: Associations + AsRef<Path>,
    Container: Associations + AsRef<Path>,
    Directory: Associations + AsRef<Path>,
{
    pub global_settings: GlobalSettings<Repository, Container>,
    pub members: Vec<Member<Directory>>,
}

pub type OwnedManifest = Manifest<OwnedRepository, OwnedContainer, OwnedDirectory>;
pub type BorrowedManifest<'a> =
    Manifest<BorrowedRepository<'a>, BorrowedContainer<'a>, BorrowedDirectory<'a>>;

impl<Repository, Container, Directory> Manifest<Repository, Container, Directory>
where
    Repository: Associations + AsRef<Path>,
    Container: Associations + AsRef<Path>,
    Directory: Associations + AsRef<Path>,
{
    pub fn as_path(&self) -> BorrowedManifest<'_> {
        Manifest {
            global_settings: self.global_settings.as_path(),
            members: self.members.iter().map(Member::as_path).collect(),
        }
    }

    pub fn resolve_members(&self) -> impl Iterator<Item = OwnedMember> + '_ {
        self.members
            .iter()
            .map(move |member| member.resolve(&self.global_settings))
    }
}

impl OwnedManifest {
    pub fn from_env() -> Result<Self, String> {
        Manifest::from_file(MANIFEST_BASENAME.as_ref())
    }

    pub fn from_file(file: &Path) -> Result<Self, String> {
        match File::open(file) {
            Ok(content) => content
                .pipe(serde_yaml::from_reader::<_, OwnedManifest>)
                .map_err(|error| format!("cannot deserialize {:?} as manifest: {}", file, error))?
                .pipe(Ok),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => Ok(Manifest::default()),
                _ => Err(format!("cannot open {:?} as a file: {}", file, error)),
            },
        }
    }
}
