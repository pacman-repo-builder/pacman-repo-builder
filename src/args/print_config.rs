use argh::*;
use std::path::PathBuf;

#[derive(Debug, FromArgs)]
#[argh(
    subcommand,
    name = "print-config",
    description = "Generate manifest YAML from a list of directories"
)]
pub struct PrintConfigArgs {
    #[argh(
        option,
        long = "container",
        short = 'D',
        description = "containers of build directories"
    )]
    pub containers: Vec<PathBuf>,
    #[argh(
        option,
        long = "repository",
        short = 'T',
        description = "paths to repository file"
    )]
    pub repository: PathBuf,
    #[argh(
        switch,
        description = "skip if directory does not contains build script"
    )]
    pub require_pkgbuild: bool,
    #[argh(
        switch,
        description = "skip if directory does not contains package info file"
    )]
    pub require_srcinfo: bool,
    #[argh(option, description = "set install-missing-dependencies")]
    pub with_install_missing_dependencies: Option<bool>,
    #[argh(option, description = "set clean-before-build")]
    pub with_clean_before_build: Option<bool>,
    #[argh(option, description = "set clean-after-build")]
    pub with_clean_after_build: Option<bool>,
    #[argh(option, description = "set force-rebuild")]
    pub with_force_rebuild: Option<bool>,
    #[argh(option, description = "set pacman")]
    pub with_pacman: Option<String>,
    #[argh(option, description = "set packager")]
    pub with_packager: Option<String>,
    #[argh(option, description = "set allow-failure")]
    pub with_allow_failure: Option<bool>,
    #[argh(option, description = "set dereference-database-symlinks")]
    pub with_dereference_database_symlinks: Option<bool>,
}
