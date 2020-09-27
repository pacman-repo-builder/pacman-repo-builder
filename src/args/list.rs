use argh::*;

#[derive(Debug, FromArgs)]
#[argh(
    subcommand,
    name = "list",
    description = "List packages in build order"
)]
pub struct ListArgs {}
