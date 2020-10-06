use argh::*;

#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "outdated", description = "List outdated packages")]
pub struct OutdatedArgs {
    #[argh(switch, description = "print greater details in yaml")]
    pub details: bool,
}
