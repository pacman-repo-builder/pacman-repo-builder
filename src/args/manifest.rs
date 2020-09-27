use super::super::manifest::Manifest as Content;
use pipe_trait::*;
use std::{fs::File, str::FromStr};

#[derive(Debug)]
pub struct Manifest(pub Content);

impl FromStr for Manifest {
    type Err = String;

    fn from_str(file: &str) -> Result<Self, Self::Err> {
        file.pipe(File::open)
            .map_err(|error| format!("cannot open {:?} as a file: {}", file, error))?
            .pipe(serde_yaml::from_reader::<_, Content>)
            .map_err(|error| format!("cannot deserialize {:?} as manifest: {}", file, error))?
            .pipe(Manifest)
            .pipe(Ok)
    }
}
