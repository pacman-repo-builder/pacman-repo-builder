pub mod build_metadata;
pub mod global_settings;
pub mod member;

use global_settings::GlobalSettings;
use member::Member;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Manifest {
    pub global_settings: Option<GlobalSettings>,
    pub members: Vec<Member>,
}
