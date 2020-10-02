use super::super::{
    args::{Args, Command, PrintConfigArgs},
    manifest::{GlobalSettings, Manifest, Member, Repository},
};
use super::App;
use pipe_trait::*;
use std::{
    fs::{metadata, read_dir},
    io::stdout,
    path::Path,
};

impl App {
    pub fn run(self) -> i32 {
        let Args { command } = self.args;
        match command {
            Command::PrintConfig(args) => print_config(args),
            Command::Sort(_) => unimplemented!(),
        }
    }
}

fn print_config(args: PrintConfigArgs) -> i32 {
    let mut error_count = 0u32;

    let PrintConfigArgs {
        containers,
        repositories,
    } = args;

    let repository: Option<Repository<&Path>> = match &repositories[..] {
        [] => None,
        [repository] => Some(Repository::Single(repository)),
        repositories => repositories
            .iter()
            .map(|x| x.as_path())
            .collect::<Vec<_>>()
            .pipe(Repository::Multiple)
            .pipe(Some),
    };

    let global_settings = Some(GlobalSettings {
        container: None,
        read_build_metadata: None,
        repository,
    });

    let mut members = Vec::new();
    for container in containers {
        match read_dir(&container) {
            Err(error) => {
                eprintln!("cannot read directory {:?}: {}", &container, error);
                error_count += 1;
                continue;
            }
            Ok(list) => {
                for entry in list {
                    let directory = match entry {
                        Err(error) => {
                            eprintln!("cannot read an entry of {:?}: {}", &container, error);
                            error_count += 1;
                            continue;
                        }
                        Ok(entry) => entry,
                    }
                    .path()
                    .pipe(|name| container.join(name));
                    match metadata(&directory) {
                        Err(error) => {
                            eprintln!("cannot stat {:?}: {}", &directory, error);
                            error_count += 1;
                            continue;
                        }
                        Ok(metadata) => {
                            if !metadata.is_dir() {
                                continue;
                            }
                        }
                    }
                    members.push(Member {
                        repository: None,
                        read_build_metadata: None,
                        directory,
                    });
                }
            }
        }
    }
    let members: Vec<_> = members.iter().map(Member::as_path).collect();

    let manifest = Manifest {
        global_settings,
        members,
    };
    if let Err(error) = serde_yaml::to_writer(stdout(), &manifest) {
        eprintln!("cannot write yaml to stdout: {}", error);
        error_count += 1;
    };

    if error_count == 0 {
        0
    } else {
        eprintln!("{} errors occurred.", error_count);
        1
    }
}
