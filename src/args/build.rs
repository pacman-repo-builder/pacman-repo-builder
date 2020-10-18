use argh::*;

#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "build", description = "Build a pacman repository")]
pub struct BuildArgs {
    #[argh(switch, description = "install missing dependencies")]
    pub syncdeps: bool,
    #[argh(switch, description = "overwrite built package if there's any")]
    pub force: bool,
    #[argh(option, description = "customize package manager program")]
    pub pacman: Option<String>,
    #[argh(option, description = "directory where log files will be stored")]
    pub log_dest: Option<String>,
    #[argh(option, description = "identify the creator of resulting packages")]
    pub packager: Option<String>,
    #[argh(switch, description = "convert *.db from symlink into real file")]
    pub deref_db: bool,
}
