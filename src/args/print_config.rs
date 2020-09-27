use argh::*;
use std::path::PathBuf;

#[derive(Debug, FromArgs)]
#[argh(
    subcommand,
    name = "print-config",
    description = "Generate manifest YAML from a list of directories"
)]
pub struct PrintConfig {
    #[argh(positional, description = "containers of build directories")]
    pub containers: Vec<PathBuf>,
}
