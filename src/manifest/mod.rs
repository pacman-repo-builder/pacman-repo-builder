pub mod build_metadata;
pub mod global_settings;
pub mod member;
pub mod repository;

use global_settings::GlobalSettings;
use member::Member;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Manifest<P: AsRef<Path>> {
    pub global_settings: Option<GlobalSettings<P>>,
    pub members: Vec<Member<P>>,
}
