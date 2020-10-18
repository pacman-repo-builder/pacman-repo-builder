use argh::*;

#[derive(Debug, FromArgs)]
#[argh(
    subcommand,
    name = "deref-db",
    description = "Make *.db and *.files in repository directory real files"
)]
pub struct DerefDbArgs {}
