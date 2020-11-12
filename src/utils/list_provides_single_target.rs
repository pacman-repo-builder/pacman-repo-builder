use super::super::status::{Failure, Status};
use super::list_provides_from_info;
use command_extra::CommandExtra;
use pipe_trait::*;
use std::process::{Command, Stdio};

pub fn list_provides_single_target(pacman: &str, pkgname: &str) -> Result<Vec<String>, Status> {
    let output = pacman
        .pipe(Command::new)
        .with_arg("--sync")
        .with_arg("--info")
        .with_arg("--quiet")
        .with_arg(pkgname)
        .with_stderr(Stdio::inherit())
        .with_stdout(Stdio::piped())
        .output()
        .map_err(|error| error.pipe(Failure::from).pipe(Err))?;

    let status = output.status.code().unwrap();
    if status != 0 {
        return Err(Ok(status));
    }

    output
        .stdout
        .pipe(String::from_utf8)
        .expect("decode output of pacman as UTF-8")
        .as_str()
        .pipe(list_provides_from_info)
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .pipe(Ok)
}
