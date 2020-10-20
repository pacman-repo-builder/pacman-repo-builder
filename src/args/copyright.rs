use argh::*;

#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "copyright", description = "Show license")]
pub struct CopyrightArgs {}
