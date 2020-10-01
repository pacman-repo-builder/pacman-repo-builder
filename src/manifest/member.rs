use super::{build_metadata::BuildMetadata, repository::Repository};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Member<P: AsRef<Path>> {
    pub directory: P,
    pub read_build_metadata: Option<BuildMetadata>,
    pub repository: Option<Repository<P>>,
}

impl Member<PathBuf> {
    pub fn as_path(&self) -> Member<&Path> {
        Member {
            directory: self.directory.as_path(),
            read_build_metadata: self.read_build_metadata,
            repository: self.repository.as_ref().map(|x| x.as_path()),
        }
    }
}
