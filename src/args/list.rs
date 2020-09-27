use argh::*;
use std::ffi::OsString;

#[derive(Debug, FromArgs)]
#[argh(
    subcommand,
    name = "list",
    description = "List packages in build order"
)]
pub struct ListArgs {
    #[argh(positional, description = "paths to build directories")]
    pub directories: Vec<OsString>,
}
