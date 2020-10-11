use argh::*;

#[derive(Debug, FromArgs)]
#[argh(
    subcommand,
    name = "patch-makepkg",
    description = "Print or generate custom makepkg that allows running as root"
)]
pub struct PatchMakePkgArgs {
    #[argh(
        switch,
        description = "replace system's default makepkg with custom one"
    )]
    pub replace: bool,
}
