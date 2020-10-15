mod build_metadata;
mod global_settings;
mod member;

pub use build_metadata::BuildMetadata;
pub use global_settings::GlobalSettings;
pub use member::Member;

use pipe_trait::*;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::ErrorKind,
    path::{Path, PathBuf},
};

pub const MANIFEST_BASENAME: &str = "build-pacman-repo.yaml";

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Manifest<P: AsRef<Path>> {
    pub global_settings: GlobalSettings<P>,
    pub members: Vec<Member<P>>,
}

impl<P: AsRef<Path>> Manifest<P> {
    pub fn as_path(&self) -> Manifest<&Path> {
        Manifest {
            global_settings: self.global_settings.as_path(),
            members: self.members.iter().map(Member::as_path).collect(),
        }
    }

    pub fn resolve_members(&self) -> impl Iterator<Item = Member<PathBuf>> + '_ {
        self.members.iter().map(|x| x.to_path_buf())
    }
}

impl Manifest<PathBuf> {
    pub fn from_env() -> Result<Self, String> {
        Manifest::from_file(MANIFEST_BASENAME.as_ref())
    }

    pub fn from_file(file: &Path) -> Result<Self, String> {
        match File::open(file) {
            Ok(content) => content
                .pipe(serde_yaml::from_reader::<_, Manifest<PathBuf>>)
                .map_err(|error| format!("cannot deserialize {:?} as manifest: {}", file, error))?
                .pipe(Ok),
            Err(error) => match error.kind() {
                ErrorKind::NotFound => Ok(Manifest::default()),
                _ => Err(format!("cannot open {:?} as a file: {}", file, error)),
            },
        }
    }
}
