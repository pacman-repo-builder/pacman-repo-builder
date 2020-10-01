use super::super::manifest::Manifest;
use pipe_trait::*;
use std::{
    fs::File,
    path::{Path, PathBuf},
    str::FromStr,
};

#[derive(Debug)]
pub struct ManifestLoader(pub Manifest<PathBuf>);

impl ManifestLoader {
    pub fn load(file: &Path) -> Result<Self, String> {
        file.pipe(File::open)
            .map_err(|error| format!("cannot open {:?} as a file: {}", file, error))?
            .pipe(serde_yaml::from_reader::<_, Manifest<PathBuf>>)
            .map_err(|error| format!("cannot deserialize {:?} as manifest: {}", file, error))?
            .pipe(ManifestLoader)
            .pipe(Ok)
    }
}

impl FromStr for ManifestLoader {
    type Err = String;

    fn from_str(file: &str) -> Result<Self, Self::Err> {
        ManifestLoader::load(file.as_ref())
    }
}
