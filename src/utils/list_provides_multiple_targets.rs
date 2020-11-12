use super::super::status::Status;
use super::list_provides_single_target;
use pipe_trait::*;
use rayon::prelude::*;

pub fn list_provides_multiple_targets<'a>(
    pacman: &str,
    targets: impl IntoParallelIterator<Item = &'a str>,
) -> Result<Vec<String>, Status> {
    targets
        .into_par_iter()
        .map(|target| list_provides_single_target(pacman, target))
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .pipe(Ok)
}
