use super::{build_metadata::BuildMetadata, repository::Repository};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

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

impl GlobalSettings<PathBuf> {
    pub fn as_path(&self) -> GlobalSettings<&Path> {
        GlobalSettings {
            container: self.container.as_deref(),
            read_build_metadata: self.read_build_metadata,
            repository: self.repository.as_ref().map(|x| x.as_path()),
        }
    }
}
