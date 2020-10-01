use super::{build_metadata::BuildMetadata, repository::Repository};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Member<P: AsRef<Path>> {
    pub directory: P,
    pub read_build_metadata: Option<BuildMetadata>,
    pub repository: Option<Repository<P>>,
}
