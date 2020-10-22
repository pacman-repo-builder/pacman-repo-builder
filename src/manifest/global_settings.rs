use super::{
    Associations, BorrowedContainer, BorrowedPackager, BorrowedPacman, BorrowedRepository,
    BuildMetadata, OwnedContainer, OwnedPackager, OwnedPacman, OwnedRepository, Wrapper,
};
use pipe_trait::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GlobalSettings<Repository, Container, Pacman, Packager>
where
    Repository: Associations + AsRef<Path>,
    Container: Associations + AsRef<Path>,
    Pacman: Associations + AsRef<str>,
    Packager: Associations + AsRef<str>,
{
    pub repository: Repository,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<Container>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_build_metadata: Option<BuildMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub install_missing_dependencies: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean_before_build: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clean_after_build: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_rebuild: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pacman: Option<Pacman>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packager: Option<Packager>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_failure: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dereference_database_symlinks: Option<bool>,
}

pub type OwnedGlobalSettings =
    GlobalSettings<OwnedRepository, OwnedContainer, OwnedPacman, OwnedPackager>;
pub type BorrowedGlobalSettings<'a> = GlobalSettings<
    BorrowedRepository<'a>,
    BorrowedContainer<'a>,
    BorrowedPacman<'a>,
    BorrowedPackager<'a>,
>;

impl<Repository, Container, Pacman, Packager>
    GlobalSettings<Repository, Container, Pacman, Packager>
where
    Repository: Associations + AsRef<Path>,
    Container: Associations + AsRef<Path>,
    Pacman: Associations + AsRef<str>,
    Packager: Associations + AsRef<str>,
{
    pub fn as_path(&self) -> BorrowedGlobalSettings<'_> {
        GlobalSettings {
            repository: self.repository.as_ref().pipe(Wrapper::from_inner),
            container: self
                .container
                .as_ref()
                .map(AsRef::as_ref)
                .map(Wrapper::from_inner),
            read_build_metadata: self.read_build_metadata,
            install_missing_dependencies: self.install_missing_dependencies,
            clean_before_build: self.clean_before_build,
            clean_after_build: self.clean_after_build,
            force_rebuild: self.force_rebuild,
            pacman: self
                .pacman
                .as_ref()
                .map(AsRef::as_ref)
                .map(Wrapper::from_inner),
            packager: self
                .packager
                .as_ref()
                .map(AsRef::as_ref)
                .map(Wrapper::from_inner),
            allow_failure: self.allow_failure,
            dereference_database_symlinks: self.dereference_database_symlinks,
        }
    }

    pub fn to_path_buf(&self) -> OwnedGlobalSettings {
        GlobalSettings {
            repository: self
                .repository
                .as_ref()
                .to_path_buf()
                .pipe(Wrapper::from_inner),
            container: self
                .container
                .as_ref()
                .map(AsRef::as_ref)
                .map(Path::to_path_buf)
                .map(Wrapper::from_inner),
            read_build_metadata: self.read_build_metadata,
            install_missing_dependencies: self.install_missing_dependencies,
            clean_before_build: self.clean_before_build,
            clean_after_build: self.clean_after_build,
            force_rebuild: self.force_rebuild,
            pacman: self
                .pacman
                .as_ref()
                .map(AsRef::as_ref)
                .map(ToString::to_string)
                .map(Wrapper::from_inner),
            packager: self
                .packager
                .as_ref()
                .map(AsRef::as_ref)
                .map(ToString::to_string)
                .map(Wrapper::from_inner),
            allow_failure: self.allow_failure,
            dereference_database_symlinks: self.dereference_database_symlinks,
        }
    }
}
