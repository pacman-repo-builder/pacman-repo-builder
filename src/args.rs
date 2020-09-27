use argh::*;

#[derive(Debug, FromArgs)]
#[argh(description = "Build a custom pacman repository from a collection of PKGBUILD directories")]
pub struct Args {
    #[argh(subcommand)]
    pub command: Command,
}

#[derive(Debug, FromArgs)]
#[argh(subcommand)]
pub enum Command {
    List(ListArgs),
}

#[derive(Debug, FromArgs)]
#[argh(
    subcommand,
    name = "list",
    description = "List packages in build order"
)]
pub struct ListArgs {}
