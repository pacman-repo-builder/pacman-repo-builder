use super::super::{
    args::DerefDbArgs,
    manifest::Manifest,
    status::{Code, Failure, Status},
    utils::run_deref_db,
};
use pipe_trait::*;

pub fn deref_db(args: DerefDbArgs) -> Status {
    let DerefDbArgs {} = args;

    Manifest::from_env()
        .map_err(|error| {
            eprintln!("⮾ {}", error);
            Failure::from(Code::ManifestLoadingFailure)
        })?
        .global_settings
        .repository
        .parent()
        .expect("get repository directory")
        .pipe(run_deref_db)
        .map_err(|error| {
            eprintln!("⮾ {}", error);
            Failure::from(error)
        })?;

    Ok(0)
}
