use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum TriState {
    Enabled,
    Disabled,
    #[default]
    Inherit,
}

impl FromStr for TriState {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(match text {
            "enabled" => TriState::Enabled,
            "disabled" => TriState::Disabled,
            "inherit" => TriState::Inherit,
            _ => return Err(format!("{:?} is not a valid state", text)),
        })
    }
}
