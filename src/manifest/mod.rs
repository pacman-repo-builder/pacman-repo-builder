mod build_metadata;
mod global_settings;
mod member;
mod repository;

pub use build_metadata::BuildMetadata;
pub use global_settings::GlobalSettings;
pub use member::Member;
pub use repository::{concat_repository_options, Repository};

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Manifest<P: AsRef<Path>> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub global_settings: Option<GlobalSettings<P>>,
    pub members: Vec<Member<P>>,
}

impl<P: AsRef<Path>> Manifest<P> {
    pub fn as_path(&self) -> Manifest<&Path> {
        Manifest {
            global_settings: self.global_settings.as_ref().map(GlobalSettings::as_path),
            members: self.members.iter().map(Member::as_path).collect(),
        }
    }

    pub fn resolve_members(&self) -> impl Iterator<Item = Member<PathBuf>> + '_ {
        macro_rules! box_fn {
            ($function:expr) => {
                Box::new($function) as Box<dyn Fn(_) -> _>
            };
        }

        self.members
            .iter()
            .map(if let Some(global_settings) = &self.global_settings {
                box_fn!(move |x: &Member<P>| x.resolve(global_settings))
            } else {
                box_fn!(move |x: &Member<P>| x.to_path_buf())
            })
    }
}
