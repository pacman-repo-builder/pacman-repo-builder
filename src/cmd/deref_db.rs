use super::super::{
    args::DerefDbArgs,
    manifest::Manifest,
    status::{Code, Failure, Status},
    utils::dereference_database_symlinks,
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
        .pipe(dereference_database_symlinks)
        .map_err(|error| {
            eprintln!("⮾ {}", error);
            Failure::from(error)
        })?;

    Ok(0)
}
