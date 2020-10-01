use super::{build_metadata::BuildMetadata, repository::Repository};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GlobalSettings<P: AsRef<Path>> {
    pub container: Option<P>,
    pub read_build_metadata: Option<BuildMetadata>,
    pub repository: Option<Repository<P>>,
}
