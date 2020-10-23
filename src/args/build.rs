use argh::*;

#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "build", description = "Build a pacman repository")]
pub struct BuildArgs {
    #[argh(option, description = "directory where log files will be stored")]
    pub log_dest: Option<String>,
}
