use super::super::{
    args::DerefDbArgs,
    manifest::BuildPacmanRepo,
    status::{Code, Failure, Status},
    utils::run_deref_db,
};
use pipe_trait::*;

pub fn deref_db(args: DerefDbArgs) -> Status {
    let DerefDbArgs {} = args;

    BuildPacmanRepo::from_env()
        .map_err(|error| {
            eprintln!("⮾ {}", error);
            Failure::from(Code::ManifestLoadingFailure)
        })?
        .global_settings
        .repository
        .as_ref()
        .parent()
        .expect("get repository directory")
        .pipe(run_deref_db)
        .map_err(|error| {
            eprintln!("⮾ {}", error);
            Failure::from(error)
        })?;

    Ok(())
}
