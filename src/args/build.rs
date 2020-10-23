use argh::*;

#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "build", description = "Build a pacman repository")]
pub struct BuildArgs {}
