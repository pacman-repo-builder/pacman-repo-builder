use argh::*;

#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "outdated", description = "List outdated packages")]
pub struct OutdatedArgs {}
