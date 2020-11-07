use super::{
    ArchCollectionWrapper, AurCollectionWrapper, BorrowedArchCollection, BorrowedAurCollection,
    BorrowedContainer, BorrowedFailedBuildRecord, BorrowedPackager, BorrowedPacman,
    BorrowedRepository, ContainerWrapper, FailedBuildRecordWrapper, GlobalSettings,
    OwnedArchCollection, OwnedAurCollection, OwnedContainer, OwnedFailedBuildRecord, OwnedPackager,
    OwnedPacman, OwnedRepository, PackagerWrapper, PacmanWrapper, RepositoryWrapper,
};
use serde::{Deserialize, Serialize};

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
