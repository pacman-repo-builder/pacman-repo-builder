use super::{ArchCollectionWrapper, BorrowedArchCollection, OwnedArchCollection, Wrapper};
use pipe_trait::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(
    from = "ArchFilterSerdeHelper<ArchCollection>",
    into = "ArchFilterSerdeHelper<ArchCollection>"
)]
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

/* SERDE HELPER */

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(untagged)]
pub enum ArchFilterSerdeHelper<ArchCollection>
where
    ArchCollection: ArchCollectionWrapper,
{
    MonoVariant(MonoVariantSerdeHelper),
    Selective(ArchCollection),
}

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum MonoVariantSerdeHelper {
    Any,
}

impl<ArchCollection> From<ArchFilterSerdeHelper<ArchCollection>> for ArchFilter<ArchCollection>
where
    ArchCollection: ArchCollectionWrapper,
{
    fn from(source: ArchFilterSerdeHelper<ArchCollection>) -> Self {
        match source {
            ArchFilterSerdeHelper::MonoVariant(MonoVariantSerdeHelper::Any) => ArchFilter::Any,
            ArchFilterSerdeHelper::Selective(collection) => ArchFilter::Selective(collection),
        }
    }
}

impl<ArchCollection> From<ArchFilter<ArchCollection>> for ArchFilterSerdeHelper<ArchCollection>
where
    ArchCollection: ArchCollectionWrapper,
{
    fn from(arch_filter: ArchFilter<ArchCollection>) -> Self {
        match arch_filter {
            ArchFilter::Any => ArchFilterSerdeHelper::MonoVariant(MonoVariantSerdeHelper::Any),
            ArchFilter::Selective(collection) => ArchFilterSerdeHelper::Selective(collection),
        }
    }
}

#[test]
fn test_serialize() {
    use super::super::utils::serialize_iter_yaml;
    use std::fmt::Write;

    let actual = serialize_iter_yaml(&[
        OwnedArchFilter::Any,
        OwnedArchFilter::from_str_iter(&["x86_64", "i686"]).unwrap(),
    ])
    .unwrap();
    eprintln!("\n\nACTUAL:\n\n{}\n\n", actual.trim());

    let mut expected = String::new();
    writeln!(expected, "---").unwrap();
    writeln!(expected, "any").unwrap();
    writeln!(expected, "---").unwrap();
    writeln!(expected, "- x86_64").unwrap();
    writeln!(expected, "- i686").unwrap();

    assert_eq!(&actual, &expected);
}

#[test]
fn test_deserialize() {
    let actual = (
        serde_yaml::from_str("any").unwrap(),
        serde_yaml::from_str("[x86_64, i686]").unwrap(),
    );
    let expected = (
        OwnedArchFilter::Any,
        OwnedArchFilter::from_str_iter(&["x86_64", "i686"]).unwrap(),
    );
    assert_eq!(&actual, &expected);
}
