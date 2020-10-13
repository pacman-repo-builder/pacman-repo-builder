mod build;
mod outdated;
mod patch_makepkg;
mod print_config;
mod sort;
mod sync_srcinfo;

pub use build::BuildArgs;
pub use outdated::{OutdatedArgs, OutdatedDetails};
pub use patch_makepkg::PatchMakePkgArgs;
pub use print_config::PrintConfigArgs;
pub use sort::SortArgs;
pub use sync_srcinfo::SyncSrcInfoArgs;

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
    Sort(SortArgs),
    PrintConfig(PrintConfigArgs),
    Outdated(OutdatedArgs),
    SyncSrcInfo(SyncSrcInfoArgs),
    PatchMakePkg(PatchMakePkgArgs),
    Build(BuildArgs),
}
