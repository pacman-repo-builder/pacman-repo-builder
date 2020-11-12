use super::super::status::Status;
use super::{list_all_native_packages, list_provides_multiple_targets};
use pipe_trait::*;

pub fn list_all_native_targets(pacman: &str) -> Result<Vec<String>, Status> {
    let mut targets = list_all_native_packages(pacman)?;
    let provides = targets
        .iter()
        .map(|x| x.as_str())
        .collect::<Vec<_>>()
        .pipe(|targets| list_provides_multiple_targets(pacman, targets))?;
    targets.extend(provides);
    Ok(targets)
}
