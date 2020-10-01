pub mod build_metadata;
pub mod global_settings;
pub mod member;
pub mod repository;

use global_settings::GlobalSettings;
use member::Member;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Manifest<P: AsRef<Path>> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_settings: Option<GlobalSettings<P>>,
    pub members: Vec<Member<P>>,
}

impl Manifest<PathBuf> {
    pub fn as_path(&self) -> Manifest<&Path> {
        Manifest {
            global_settings: self.global_settings.as_ref().map(|x| x.as_path()),
            members: self.members.iter().map(Member::as_path).collect(),
        }
    }
}
