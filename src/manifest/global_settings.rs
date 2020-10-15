use super::BuildMetadata;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct GlobalSettings<P: AsRef<Path>> {
    pub repository: P,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<P>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_build_metadata: Option<BuildMetadata>,
}

impl<P: AsRef<Path>> GlobalSettings<P> {
    pub fn as_path(&self) -> GlobalSettings<&Path> {
        GlobalSettings {
            repository: self.repository.as_ref(),
            container: self.container.as_ref().map(AsRef::as_ref),
            read_build_metadata: self.read_build_metadata,
        }
    }

    pub fn to_path_buf(&self) -> GlobalSettings<PathBuf> {
        GlobalSettings {
            repository: self.repository.as_ref().to_path_buf(),
            container: self
                .container
                .as_ref()
                .map(AsRef::as_ref)
                .map(Path::to_path_buf),
            read_build_metadata: self.read_build_metadata,
        }
    }
}
