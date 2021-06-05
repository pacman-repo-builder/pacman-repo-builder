mod build;
mod copyright;
mod deref_db;
mod init_aur_builder;
mod outdated;
mod patch_makepkg;
mod print_config;
mod sort;
mod sync_srcinfo;

pub use build::BuildArgs;
pub use copyright::CopyrightArgs;
pub use deref_db::DerefDbArgs;
pub use init_aur_builder::InitAurBuilderArgs;
pub use outdated::{OutdatedArgs, OutdatedDetails};
pub use patch_makepkg::PatchMakepkgArgs;
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
    InitAurBuilder(InitAurBuilderArgs),
    Outdated(OutdatedArgs),
    SyncSrcInfo(SyncSrcInfoArgs),
    PatchMakepkg(PatchMakepkgArgs),
    DerefDb(DerefDbArgs),
    Build(BuildArgs),
    Copyright(CopyrightArgs),
}
