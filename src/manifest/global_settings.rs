use super::{build_metadata::BuildMetadata, repository::Repository};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GlobalSettings<P: AsRef<Path>> {
    pub container: Option<P>,
    pub read_build_metadata: Option<BuildMetadata>,
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
