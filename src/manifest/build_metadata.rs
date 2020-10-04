use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BuildMetadata {
    SrcInfo,
    PkgBuild,
    Either,
}

impl Default for BuildMetadata {
    fn default() -> Self {
        BuildMetadata::Either
    }
}
