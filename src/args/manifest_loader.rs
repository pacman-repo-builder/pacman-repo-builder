use super::super::manifest::Manifest;
use pipe_trait::*;
use std::{fs::File, str::FromStr};

#[derive(Debug)]
pub struct ManifestLoader(pub Manifest);

impl FromStr for ManifestLoader {
    type Err = String;

    fn from_str(file: &str) -> Result<Self, Self::Err> {
        file.pipe(File::open)
            .map_err(|error| format!("cannot open {:?} as a file: {}", file, error))?
            .pipe(serde_yaml::from_reader::<_, Manifest>)
            .map_err(|error| format!("cannot deserialize {:?} as manifest: {}", file, error))?
            .pipe(ManifestLoader)
            .pipe(Ok)
    }
}
