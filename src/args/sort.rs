use argh::*;
use std::path::PathBuf;

#[derive(Debug, FromArgs)]
#[argh(
    subcommand,
    name = "sort",
    description = "List packages in build order"
)]
pub struct SortArgs {
    #[argh(option, short = 'C', description = "path to manifest file")]
    pub config: PathBuf,
}
