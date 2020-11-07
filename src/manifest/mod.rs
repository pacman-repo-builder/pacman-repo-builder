mod arch_filter;
mod build_metadata;
mod global_settings;
mod member;
mod tristate;
mod wrapper;

pub use arch_filter::{ArchFilter, BorrowedArchFilter, OwnedArchFilter};
pub use build_metadata::BuildMetadata;
pub use global_settings::{BorrowedGlobalSettings, GlobalSettings, OwnedGlobalSettings};
pub use member::{BorrowedMember, Member, OwnedMember};
pub use tristate::TriState;
pub use wrapper::{
    ArchCollection, ArchCollectionWrapper, Associations, BorrowedArchCollection, BorrowedContainer,
    BorrowedDirectory, BorrowedFailedBuildRecord, BorrowedInner, BorrowedPackager, BorrowedPacman,
    BorrowedRepository, BorrowedWrapper, Container, ContainerWrapper, Directory, DirectoryWrapper,
    FailedBuildRecord, FailedBuildRecordWrapper, OwnedArchCollection, OwnedContainer,
    OwnedDirectory, OwnedFailedBuildRecord, OwnedInner, OwnedPackager, OwnedPacman,
    OwnedRepository, OwnedWrapper, Packager, PackagerWrapper, Pacman, PacmanWrapper, Repository,
    RepositoryWrapper, Wrapper,
};

use pipe_trait::*;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::ErrorKind, path::Path};

pub const BUILD_PACMAN_REPO: &str = "build-pacman-repo.yaml";

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct BuildPacmanRepo<
    Repository,
    Container,
    FailedBuildRecord,
    ArchCollection,
    Pacman,
    Packager,
    Directory,
> where
    Repository: RepositoryWrapper,
    Container: ContainerWrapper,
    FailedBuildRecord: FailedBuildRecordWrapper,
    ArchCollection: ArchCollectionWrapper,
    Pacman: PacmanWrapper,
    Packager: PackagerWrapper,
    Directory: DirectoryWrapper,
{
    pub global_settings:
        GlobalSettings<Repository, Container, FailedBuildRecord, ArchCollection, Pacman, Packager>,
    pub members: Vec<Member<Directory, Pacman>>,
}

pub type OwnedBuildPacmanRepo = BuildPacmanRepo<
    OwnedRepository,
    OwnedContainer,
    OwnedFailedBuildRecord,
    OwnedArchCollection,
    OwnedPacman,
    OwnedPackager,
    OwnedDirectory,
>;
pub type BorrowedBuildPacmanRepo<'a> = BuildPacmanRepo<
    BorrowedRepository<'a>,
    BorrowedContainer<'a>,
    BorrowedFailedBuildRecord<'a>,
    BorrowedArchCollection<'a>,
    BorrowedPacman<'a>,
    BorrowedPackager<'a>,
    BorrowedDirectory<'a>,
>;

impl<Repository, Container, FailedBuildRecord, ArchCollection, Pacman, Packager, Directory>
    BuildPacmanRepo<
        Repository,
        Container,
        FailedBuildRecord,
        ArchCollection,
        Pacman,
        Packager,
        Directory,
    >
where
    Repository: RepositoryWrapper,
    Container: ContainerWrapper,
    FailedBuildRecord: FailedBuildRecordWrapper,
    ArchCollection: ArchCollectionWrapper,
    Pacman: PacmanWrapper,
    Packager: PackagerWrapper,
    Directory: DirectoryWrapper,
{
    pub fn as_borrowed(&self) -> BorrowedBuildPacmanRepo<'_> {
        BuildPacmanRepo {
            global_settings: self.global_settings.as_borrowed(),
            members: self.members.iter().map(Member::as_borrowed).collect(),
        }
    }

    pub fn resolve_members(&self) -> impl Iterator<Item = OwnedMember> + '_ {
        self.members
            .iter()
            .map(move |member| member.resolve(&self.global_settings))
    }
}

impl OwnedBuildPacmanRepo {
    pub fn from_env() -> Result<Self, String> {
        BuildPacmanRepo::from_file(BUILD_PACMAN_REPO.as_ref())
    }

    pub fn from_file(file: &Path) -> Result<Self, String> {
        match File::open(file) {
            Ok(content) => content
                .pipe(serde_yaml::from_reader::<_, OwnedBuildPacmanRepo>)
                .map_err(|error| format!("cannot deserialize {:?} as manifest: {}", file, error))?
                .pipe(Ok),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => Ok(BuildPacmanRepo::default()),
                _ => Err(format!("cannot open {:?} as a file: {}", file, error)),
            },
        }
    }
}
