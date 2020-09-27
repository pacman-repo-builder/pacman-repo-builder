pub mod print_config;
pub mod sort;

use argh::*;
use print_config::PrintConfig;
use sort::SortArgs;

#[derive(Debug, FromArgs)]
#[argh(description = "Build a custom pacman repository from a collection of PKGBUILD directories")]
pub struct Args {
    #[argh(subcommand)]
    pub command: Command,
}

#[derive(Debug, FromArgs)]
#[argh(subcommand)]
pub enum Command {
    Sort(SortArgs),
    PrintConfig(PrintConfig),
}
