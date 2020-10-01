use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Repository<P: AsRef<Path>> {
    Single(P),
    Multiple(Vec<P>),
}

impl<P: AsRef<Path>> Repository<P> {
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

pub fn concat_options<P: AsRef<Path>>(
    left: Option<Repository<P>>,
    right: Option<Repository<P>>,
) -> Option<Repository<P>> {
    match (left, right) {
        (None, None) => None,
        (None, right) => right,
        (left, None) => left,
        (Some(left), Some(right)) => Some(left.concat(right)),
    }
}
