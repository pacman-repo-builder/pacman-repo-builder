use super::{ArchCollectionWrapper, BorrowedArchCollection, OwnedArchCollection, Wrapper};
use pipe_trait::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(untagged, rename_all = "kebab-case")]
pub enum ArchFilter<ArchCollection>
where
    ArchCollection: ArchCollectionWrapper,
{
    Any,
    Selective(ArchCollection),
}

pub type OwnedArchFilter = ArchFilter<OwnedArchCollection>;
pub type BorrowedArchFilter<'a> = ArchFilter<BorrowedArchCollection<'a>>;

impl<ArchCollection> ArchFilter<ArchCollection>
where
    ArchCollection: ArchCollectionWrapper,
{
    pub fn as_slice(&self) -> BorrowedArchFilter<'_> {
        match self {
            ArchFilter::Any => ArchFilter::Any,
            ArchFilter::Selective(collection) => collection
                .as_ref()
                .pipe(BorrowedArchCollection::from_inner)
                .pipe(ArchFilter::Selective),
        }
    }

    pub fn to_vec(&self) -> OwnedArchFilter {
        match self {
            ArchFilter::Any => ArchFilter::Any,
            ArchFilter::Selective(collection) => collection
                .as_ref()
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .pipe(OwnedArchCollection::from_inner)
                .pipe(ArchFilter::Selective),
        }
    }
}

impl OwnedArchFilter {
    pub fn from_arch_vec(arch_vec: Vec<String>) -> Option<Self> {
        if arch_vec.is_empty() {
            return None;
        }

        Some(if arch_vec.iter().any(|x| x == "any") {
            ArchFilter::Any
        } else {
            arch_vec
                .pipe(OwnedArchCollection::from_inner)
                .pipe(ArchFilter::Selective)
        })
    }

    pub fn from_str_iter<Iter>(iter: Iter) -> Option<Self>
    where
        Iter: IntoIterator,
        Iter::Item: ToString,
    {
        iter.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .pipe(OwnedArchFilter::from_arch_vec)
    }
}
