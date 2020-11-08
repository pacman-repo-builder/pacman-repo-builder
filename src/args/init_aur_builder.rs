use argh::FromArgs;

#[derive(Debug, FromArgs)]
#[argh(
    subcommand,
    name = "init-aur-builder",
    description = "Initialize build directories with PKGBUILDs from the AUR"
)]
pub struct InitAurBuilderArgs {}
