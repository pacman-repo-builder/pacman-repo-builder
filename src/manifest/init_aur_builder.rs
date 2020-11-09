use super::{
    ArchCollectionWrapper, AurCollectionWrapper, BorrowedArchCollection, BorrowedAurCollection,
    BorrowedContainer, BorrowedFailedBuildRecord, BorrowedPackager, BorrowedPacman,
    BorrowedRepository, ContainerWrapper, FailedBuildRecordWrapper, GlobalSettings,
    OwnedArchCollection, OwnedAurCollection, OwnedContainer, OwnedFailedBuildRecord, OwnedPackager,
    OwnedPacman, OwnedRepository, PackagerWrapper, PacmanWrapper, RepositoryWrapper, Wrapper,
};
use pipe_trait::*;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::ErrorKind, path::Path};

pub const INIT_AUR_BUILDER: &str = "init-aur-builder.yaml";

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct InitAurBuilder<
    Repository,
    Container,
    FailedBuildRecord,
    ArchCollection,
    Pacman,
    Packager,
    AurCollection,
> where
    Repository: RepositoryWrapper,
    Container: ContainerWrapper,
    FailedBuildRecord: FailedBuildRecordWrapper,
    ArchCollection: ArchCollectionWrapper,
    Pacman: PacmanWrapper,
    Packager: PackagerWrapper,
    AurCollection: AurCollectionWrapper,
{
    pub global_settings:
        GlobalSettings<Repository, Container, FailedBuildRecord, ArchCollection, Pacman, Packager>,
    pub aur_package_names: AurCollection,
}

pub type OwnedInitAurBuilder = InitAurBuilder<
    OwnedRepository,
    OwnedContainer,
    OwnedFailedBuildRecord,
    OwnedArchCollection,
    OwnedPacman,
    OwnedPackager,
    OwnedAurCollection,
>;

pub type BorrowedInitAurBuilder<'a> = InitAurBuilder<
    BorrowedRepository<'a>,
    BorrowedContainer<'a>,
    BorrowedFailedBuildRecord<'a>,
    BorrowedArchCollection<'a>,
    BorrowedPacman<'a>,
    BorrowedPackager<'a>,
    BorrowedAurCollection<'a>,
>;

impl<Repository, Container, FailedBuildRecord, ArchCollection, Pacman, Packager, AurCollection>
    InitAurBuilder<
        Repository,
        Container,
        FailedBuildRecord,
        ArchCollection,
        Pacman,
        Packager,
        AurCollection,
    >
where
    Repository: RepositoryWrapper,
    Container: ContainerWrapper,
    FailedBuildRecord: FailedBuildRecordWrapper,
    ArchCollection: ArchCollectionWrapper,
    Pacman: PacmanWrapper,
    Packager: PackagerWrapper,
    AurCollection: AurCollectionWrapper,
{
    pub fn with_global_settings(
        mut self,
        global_settings: GlobalSettings<
            Repository,
            Container,
            FailedBuildRecord,
            ArchCollection,
            Pacman,
            Packager,
        >,
    ) -> Self {
        self.global_settings = global_settings;
        self
    }
}

impl OwnedInitAurBuilder {
    pub fn from_env() -> Result<Self, String> {
        InitAurBuilder::from_file(INIT_AUR_BUILDER.as_ref())
    }

    pub fn from_file(file: &Path) -> Result<Self, String> {
        match File::open(file) {
            Ok(content) => content
                .pipe(serde_yaml::from_reader::<_, OwnedInitAurBuilder>)
                .map_err(|error| {
                    format!("cannot deserialize {:?} as InitAurBuilder: {}", file, error)
                })?
                .pipe(Ok),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => Ok(InitAurBuilder::default()),
                _ => Err(format!("cannot open {:?} as a file: {}", file, error)),
            },
        }
    }

    pub fn with_package(mut self, package_name: String) -> Self {
        self.aur_package_names.inner_mut().push(package_name);
        self
    }
}
