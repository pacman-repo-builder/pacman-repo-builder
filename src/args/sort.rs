use super::manifest_loader::ManifestLoader;
use argh::*;

#[derive(Debug, FromArgs)]
#[argh(
    subcommand,
    name = "sort",
    description = "List packages in build order"
)]
pub struct SortArgs {
    #[argh(option, short = 'C', description = "path to manifest file")]
    pub config: ManifestLoader,
}
