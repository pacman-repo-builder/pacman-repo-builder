use super::srcinfo::SrcInfo;
use argh::*;
use std::ffi::OsString;

#[derive(Debug, FromArgs)]
#[argh(
    subcommand,
    name = "sort",
    description = "List packages in build order"
)]
pub struct SortArgs {
    #[argh(positional, description = "paths to build directories")]
    pub directories: Vec<OsString>,

    #[argh(
        option,
        description = "how to read build metadata (srcinfo|pkgbuild|either)",
        default = "SrcInfo::Either"
    )]
    pub srcinfo: SrcInfo,
}
