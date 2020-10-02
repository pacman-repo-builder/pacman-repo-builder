use super::{BuildMetadata, Repository};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Member<P: AsRef<Path>> {
    pub directory: P,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_build_metadata: Option<BuildMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository<P>>,
}

impl<P: AsRef<Path>> Member<P> {
    pub fn as_path(&self) -> Member<&Path> {
        Member {
            directory: self.directory.as_ref(),
            read_build_metadata: self.read_build_metadata,
            repository: self.repository.as_ref().map(|x| x.as_path()),
        }
    }
}
