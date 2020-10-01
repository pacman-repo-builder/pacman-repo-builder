use super::{build_metadata::BuildMetadata, repository::Repository};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GlobalSettings {
    pub container: Option<PathBuf>,
    pub read_build_metadata: Option<BuildMetadata>,
    pub repository: Option<Repository>,
}
