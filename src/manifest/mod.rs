mod arch_filter;
mod build_metadata;
mod global_settings;
mod member;
mod origin;
mod wrapper;

pub use arch_filter::{ArchFilter, BorrowedArchFilter, OwnedArchFilter};
pub use build_metadata::BuildMetadata;
pub use global_settings::{BorrowedGlobalSettings, GlobalSettings, OwnedGlobalSettings};
pub use member::{BorrowedMember, Member, OwnedMember};
pub use origin::{BorrowedOrigin, Origin, OwnedOrigin};
pub use wrapper::{
    ArchCollection, ArchCollectionWrapper, Associations, AurName, AurNameWrapper,
    BorrowedArchCollection, BorrowedAurName, BorrowedContainer, BorrowedDirectory,
    BorrowedFailedBuildRecord, BorrowedGitUrl, BorrowedInner, BorrowedPackager, BorrowedPacman,
    BorrowedRepository, BorrowedWrapper, Container, ContainerWrapper, Directory, DirectoryWrapper,
    FailedBuildRecord, FailedBuildRecordWrapper, GitUrl, GitUrlWrapper, OwnedArchCollection,
    OwnedAurName, OwnedContainer, OwnedDirectory, OwnedFailedBuildRecord, OwnedGitUrl, OwnedInner,
    OwnedPackager, OwnedPacman, OwnedRepository, OwnedWrapper, Packager, PackagerWrapper, Pacman,
    PacmanWrapper, Repository, RepositoryWrapper, Wrapper,
};

use pipe_trait::*;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::ErrorKind, path::Path};

pub const MANIFEST_BASENAME: &str = "build-pacman-repo.yaml";

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Manifest<
    Repository,
    Container,
    FailedBuildRecord,
    ArchCollection,
    Pacman,
    Packager,
    Directory,
    GitUrl,
    AurName,
> where
    Repository: RepositoryWrapper,
    Container: ContainerWrapper,
    FailedBuildRecord: FailedBuildRecordWrapper,
    ArchCollection: ArchCollectionWrapper,
    Pacman: PacmanWrapper,
    Packager: PackagerWrapper,
    Directory: DirectoryWrapper,
    GitUrl: GitUrlWrapper,
    AurName: AurNameWrapper,
{
    pub global_settings:
        GlobalSettings<Repository, Container, FailedBuildRecord, ArchCollection, Pacman, Packager>,
    pub members: Vec<Member<Directory, GitUrl, AurName, Pacman>>,
}

pub type OwnedManifest = Manifest<
    OwnedRepository,
    OwnedContainer,
    OwnedFailedBuildRecord,
    OwnedArchCollection,
    OwnedPacman,
    OwnedPackager,
    OwnedDirectory,
    OwnedGitUrl,
    OwnedAurName,
>;
pub type BorrowedManifest<'a> = Manifest<
    BorrowedRepository<'a>,
    BorrowedContainer<'a>,
    BorrowedFailedBuildRecord<'a>,
    BorrowedArchCollection<'a>,
    BorrowedPacman<'a>,
    BorrowedPackager<'a>,
    BorrowedDirectory<'a>,
    BorrowedGitUrl<'a>,
    BorrowedAurName<'a>,
>;

impl<
        Repository,
        Container,
        FailedBuildRecord,
        ArchCollection,
        Pacman,
        Packager,
        Directory,
        GitUrl,
        AurName,
    >
    Manifest<
        Repository,
        Container,
        FailedBuildRecord,
        ArchCollection,
        Pacman,
        Packager,
        Directory,
        GitUrl,
        AurName,
    >
where
    Repository: RepositoryWrapper,
    Container: ContainerWrapper,
    FailedBuildRecord: FailedBuildRecordWrapper,
    ArchCollection: ArchCollectionWrapper,
    Pacman: PacmanWrapper,
    Packager: PackagerWrapper,
    Directory: DirectoryWrapper,
    GitUrl: GitUrlWrapper,
    AurName: AurNameWrapper,
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
