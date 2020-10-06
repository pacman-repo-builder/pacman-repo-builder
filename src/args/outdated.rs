use argh::*;
use std::str::FromStr;

#[derive(Debug, FromArgs)]
#[argh(subcommand, name = "outdated", description = "List outdated packages")]
pub struct OutdatedArgs {
    #[argh(
        option,
        description = "level of details of information (pkg-file-path|lossy-yaml|strict-yaml)"
    )]
    pub details: Option<OutdatedDetails>,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum OutdatedDetails {
    PkgFilePath,
    LossyYaml,
    StrictYaml,
}

impl Default for OutdatedDetails {
    fn default() -> Self {
        OutdatedDetails::PkgFilePath
    }
}

impl FromStr for OutdatedDetails {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(match text {
            "pkg-file-path" => OutdatedDetails::PkgFilePath,
            "lossy-yaml" => OutdatedDetails::LossyYaml,
            "strict-yaml" => OutdatedDetails::StrictYaml,
            _ => return Err(format!("invalid choice: {}", text)),
        })
    }
}
