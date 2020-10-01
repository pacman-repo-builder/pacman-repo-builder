use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum BuildMetadata {
    SrcInfo,
    PkgBuild,
    Either,
}
