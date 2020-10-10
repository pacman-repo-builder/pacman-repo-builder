use argh::*;

#[derive(Debug, FromArgs)]
#[argh(
    subcommand,
    name = "sync-srcinfo",
    description = "Synchronization of PKGBUILD and .SRCINFO"
)]
pub struct SyncSrcInfoArgs {
    #[argh(
        switch,
        short = 'u',
        description = "update outdated build information files"
    )]
    pub update: bool,
}
