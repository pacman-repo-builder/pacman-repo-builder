use super::super::manifest::OwnedFailedBuildRecord;
use super::PackageFileName;
use pipe_trait::*;
use std::{fs::File, io::ErrorKind};

pub fn load_failed_build_record(
    failed_build_record: &Option<OwnedFailedBuildRecord>,
) -> Result<Vec<PackageFileName<String, String, String>>, String> {
    let Some(failed_build_record) = failed_build_record else {
        return Ok(Default::default());
    };

    let record_path = failed_build_record.as_ref();
    match record_path.pipe(File::open).map(serde_yaml::from_reader) {
        Ok(Ok(record)) => Ok(record),
        Ok(Err(error)) => {
            eprintln!(
                "⚠ Cannot parse file {:?} as a FailedBuildRecord: {}",
                record_path, error,
            );
            Ok(Default::default())
        }
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                Ok(Default::default())
            } else {
                Err(format!(
                    "Cannot read {:?} as a file: {}",
                    record_path, error,
                ))
            }
        }
    }
}
