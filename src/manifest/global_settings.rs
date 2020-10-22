use super::{
    Associations, BorrowedContainer, BorrowedRepository, BuildMetadata, OwnedContainer,
    OwnedRepository, Wrapper,
};
use pipe_trait::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GlobalSettings<Repository, Container>
where
    Repository: Associations + AsRef<Path>,
    Container: Associations + AsRef<Path>,
{
    pub repository: Repository,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<Container>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_build_metadata: Option<BuildMetadata>,
}

pub type OwnedGlobalSettings = GlobalSettings<OwnedRepository, OwnedContainer>;
pub type BorrowedGlobalSettings<'a> = GlobalSettings<BorrowedRepository<'a>, BorrowedContainer<'a>>;

impl<Repository, Container> GlobalSettings<Repository, Container>
where
    Repository: Associations + AsRef<Path>,
    Container: Associations + AsRef<Path>,
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
        }
    }
}
