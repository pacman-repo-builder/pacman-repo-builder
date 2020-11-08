use super::super::status::{Failure, Status};
use command_extra::CommandExtra;
use pipe_trait::*;
use std::process::{Command, Stdio};

pub fn list_all_native_packages(pacman: &str) -> Result<Vec<String>, Status> {
    pacman
        .pipe(Command::new)
        .with_arg("--sync")
        .with_arg("--list")
        .with_arg("--quiet")
        .with_stdin(Stdio::inherit())
        .with_stderr(Stdio::inherit())
        .with_stdout(Stdio::piped())
        .output()
        .map_err(|error| {
            eprintln!("⮾ Failed to query all native packages: {}", error);
            error.pipe(Failure::from).into()
        })?
        .pipe(|output| {
            if output.status.success() {
                Ok(output.stdout)
            } else {
                let status = output.status.code().unwrap_or(1);
                eprintln!(
                    "⮾ Failed to query all native packages: Process exits with status code {}\n",
                    status,
                );
                Err(Ok(status))
            }
        })?
        .pipe(String::from_utf8)
        .expect("decode output of pacman as UTF-8")
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .pipe(Ok)
}
