use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Repository {
    Single(PathBuf),
    Multiple(Vec<PathBuf>),
}

impl Repository {
    pub fn concat(self, other: Self) -> Self {
        use Repository::*;
        Multiple(match (self, other) {
            (Single(left), Single(right)) => vec![left, right],
            (Single(left), Multiple(right)) => {
                let mut result = vec![left];
                result.extend(right);
                result
            }
            (Multiple(mut left), Single(right)) => {
                left.push(right);
                left
            }
            (Multiple(mut left), Multiple(right)) => {
                left.extend(right);
                left
            }
        })
    }
}

pub fn concat_options(left: Option<Repository>, right: Option<Repository>) -> Option<Repository> {
    match (left, right) {
        (None, None) => None,
        (None, right) => right,
        (left, None) => left,
        (Some(left), Some(right)) => Some(left.concat(right)),
    }
}
