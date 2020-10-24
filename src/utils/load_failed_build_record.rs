use super::super::manifest::OwnedFailedBuildRecord;
use pipe_trait::*;
use std::{
    fs::read_to_string,
    io::{self, ErrorKind},
};

pub fn load_failed_build_record(
    failed_build_record: &Option<OwnedFailedBuildRecord>,
) -> Result<Vec<String>, io::Error> {
    let failed_build_record = if let Some(failed_build_record) = failed_build_record {
        failed_build_record
    } else {
        return Ok(Vec::new());
    };

    match read_to_string(failed_build_record.as_ref()) {
        Ok(content) => content
            .lines()
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .pipe(Ok),
        Err(error) => {
            if error.kind() == ErrorKind::NotFound {
                Ok(Vec::new())
            } else {
                Err(error)
            }
        }
    }
}
