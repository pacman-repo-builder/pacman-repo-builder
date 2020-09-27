use super::build_metadata::BuildMetadata;
use serde::{Deserialize, Serialize};
use std::ffi::OsString;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct GlobalSettings {
    pub container: Option<OsString>,
    pub read_build_metadata: Option<BuildMetadata>,
}
