use super::{
    ArchCollectionWrapper, ArchFilter, BorrowedArchCollection, BorrowedContainer,
    BorrowedFailedBuildRecord, BorrowedPackager, BorrowedPacman, BorrowedRepository,
    BorrowedWrapper, BuildMetadata, ContainerWrapper, FailedBuildRecordWrapper,
    OwnedArchCollection, OwnedContainer, OwnedFailedBuildRecord, OwnedPackager, OwnedPacman,
    OwnedRepository, OwnedWrapper, PackagerWrapper, PacmanWrapper, RepositoryWrapper, Wrapper,
};
use pipe_trait::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GlobalSettings<
    Repository,
    Container,
    FailedBuildRecord,
    ArchCollection,
    Pacman,
    Packager,
> where
    Repository: RepositoryWrapper,
    Container: ContainerWrapper,
    FailedBuildRecord: FailedBuildRecordWrapper,
    ArchCollection: ArchCollectionWrapper,
    Pacman: PacmanWrapper,
    Packager: PackagerWrapper,
{
    pub repository: Repository,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<Container>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_build_metadata: Option<BuildMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_failed_builds: Option<FailedBuildRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_missing_dependencies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean_before_build: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean_after_build: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_rebuild: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arch_filter: Option<ArchFilter<ArchCollection>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pacman: Option<Pacman>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packager: Option<Packager>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_failure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dereference_database_symlinks: Option<bool>,
}

pub type OwnedGlobalSettings = GlobalSettings<
    OwnedRepository,
    OwnedContainer,
    OwnedFailedBuildRecord,
    OwnedArchCollection,
    OwnedPacman,
    OwnedPackager,
>;
pub type BorrowedGlobalSettings<'a> = GlobalSettings<
    BorrowedRepository<'a>,
    BorrowedContainer<'a>,
    BorrowedFailedBuildRecord<'a>,
    BorrowedArchCollection<'a>,
    BorrowedPacman<'a>,
    BorrowedPackager<'a>,
>;

impl<Repository, Container, FailedBuildRecord, ArchCollection, Pacman, Packager>
    GlobalSettings<Repository, Container, FailedBuildRecord, ArchCollection, Pacman, Packager>
where
    Repository: RepositoryWrapper,
    Container: ContainerWrapper,
    FailedBuildRecord: FailedBuildRecordWrapper,
    ArchCollection: ArchCollectionWrapper,
    Pacman: PacmanWrapper,
    Packager: PackagerWrapper,
{
    pub fn as_borrowed(&self) -> BorrowedGlobalSettings<'_> {
        macro_rules! convert_option {
            ($name:ident) => {
                self.$name.as_ref().map(BorrowedWrapper::from_inner_ref)
            };
        }

        GlobalSettings {
            repository: self.repository.as_ref().pipe(Wrapper::from_inner),
            container: convert_option!(container),
            read_build_metadata: self.read_build_metadata,
            record_failed_builds: convert_option!(record_failed_builds),
            install_missing_dependencies: self.install_missing_dependencies,
            clean_before_build: self.clean_before_build,
            clean_after_build: self.clean_after_build,
            force_rebuild: self.force_rebuild,
            arch_filter: self.arch_filter.as_ref().map(ArchFilter::as_borrowed),
            pacman: convert_option!(pacman),
            packager: convert_option!(packager),
            allow_failure: self.allow_failure,
            dereference_database_symlinks: self.dereference_database_symlinks,
        }
    }

    pub fn to_owned(&self) -> OwnedGlobalSettings {
        macro_rules! convert_option {
            ($name:ident) => {
                self.$name.as_ref().map(OwnedWrapper::new_owned_from)
            };
        }

        GlobalSettings {
            repository: self.repository.as_ref().pipe(OwnedWrapper::new_owned_from),
            container: convert_option!(container),
            read_build_metadata: self.read_build_metadata,
            record_failed_builds: convert_option!(record_failed_builds),
            install_missing_dependencies: self.install_missing_dependencies,
            clean_before_build: self.clean_before_build,
            clean_after_build: self.clean_after_build,
            force_rebuild: self.force_rebuild,
            arch_filter: self.arch_filter.as_ref().map(ArchFilter::to_owned),
            pacman: convert_option!(pacman),
            packager: convert_option!(packager),
            allow_failure: self.allow_failure,
            dereference_database_symlinks: self.dereference_database_symlinks,
        }
    }
}
