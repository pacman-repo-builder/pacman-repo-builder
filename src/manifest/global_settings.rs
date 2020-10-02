use super::{BuildMetadata, Repository};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GlobalSettings<P: AsRef<Path>> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<P>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_build_metadata: Option<BuildMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository<P>>,
}

impl<P: AsRef<Path>> GlobalSettings<P> {
    pub fn as_path(&self) -> GlobalSettings<&Path> {
        GlobalSettings {
            container: self.container.as_ref().map(AsRef::as_ref),
            read_build_metadata: self.read_build_metadata,
            repository: self.repository.as_ref().map(Repository::as_path),
        }
    }
}
