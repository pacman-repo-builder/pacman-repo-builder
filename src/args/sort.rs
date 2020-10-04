use argh::*;

#[derive(Debug, FromArgs)]
#[argh(
    subcommand,
    name = "sort",
    description = "List packages in build order"
)]
pub struct SortArgs {}
