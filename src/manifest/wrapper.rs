use pipe_trait::*;
use serde::{Deserialize, Serialize};
use std::{
    iter::FromIterator,
    path::{Path, PathBuf},
};

pub trait Wrapper<Inner, OwnedInner, BorrowedInner: ?Sized> {
    fn from_inner(inner: Inner) -> Self;
    fn into_inner(self) -> Inner;
    fn inner(&self) -> &Inner;
    fn inner_mut(&mut self) -> &mut Inner;
}

pub trait OwnedWrapper<Inner: AsRef<BorrowedInner>, BorrowedInner: ?Sized>:
    Wrapper<Inner, Inner, BorrowedInner>
{
    fn new_owned_from(inner: impl AsRef<BorrowedInner>) -> Self;
}

pub trait BorrowedWrapper<'a, Inner: ?Sized + 'a, OwnedInner: AsRef<Inner>>:
    Wrapper<&'a Inner, OwnedInner, Inner>
{
    fn from_inner_ref(inner: &'a impl AsRef<Inner>) -> Self;
}

pub trait OwnedInner<'a, Return> {
    fn as_ref_wrapper(&'a self) -> Return;
}

pub trait BorrowedInner<Return>: Copy {
    fn to_owned_wrapper(self) -> Return;
}

pub trait Associations {
    type Inner;
    type OwnedInner;
    type BorrowedInner: ?Sized;
}

macro_rules! wrapper_type {
    ($name:ident, $trait_name:ident, $owned_alias:ident, $borrowed_alias:ident, $owned_inner:ident, $borrowed_inner:ident) => {
        #[derive(
            Debug, Default, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Copy, Clone,
        )]
        pub struct $name<Inner: AsRef<$borrowed_inner>>(Inner);
        pub type $owned_alias = $name<$owned_inner>;
        pub type $borrowed_alias<'a> = $name<&'a $borrowed_inner>;

        pub trait $trait_name: Associations + AsRef<$borrowed_inner> + Clone {}
        impl<Inner: AsRef<$borrowed_inner> + Clone> $trait_name for $name<Inner> {}

        impl<Inner: AsRef<$borrowed_inner>> Wrapper<Inner, $owned_inner, $borrowed_inner>
            for $name<Inner>
        {
            fn from_inner(inner: Inner) -> Self {
                $name(inner)
            }

            fn into_inner(self) -> Inner {
                self.0
            }

            fn inner(&self) -> &Inner {
                &self.0
            }

            fn inner_mut(&mut self) -> &mut Inner {
                &mut self.0
            }
        }

        impl OwnedWrapper<$owned_inner, $borrowed_inner> for $owned_alias {
            fn new_owned_from(source: impl AsRef<$borrowed_inner>) -> Self {
                source.as_ref().to_owned().pipe($name)
            }
        }

        impl<'a> BorrowedWrapper<'a, $borrowed_inner, $owned_inner> for $borrowed_alias<'a> {
            fn from_inner_ref(inner: &'a impl AsRef<$borrowed_inner>) -> Self {
                inner.as_ref().pipe($name)
            }
        }

        impl<'a> OwnedInner<'a, $borrowed_alias<'a>> for $owned_inner {
            fn as_ref_wrapper(&'a self) -> $borrowed_alias<'a> {
                $name(self.as_ref())
            }
        }

        impl<'a> BorrowedInner<$owned_alias> for &'a $borrowed_inner {
            fn to_owned_wrapper(self) -> $owned_alias {
                $name(self.to_owned())
            }
        }

        impl<Inner: AsRef<$borrowed_inner>> Associations for $name<Inner> {
            type Inner = Inner;
            type OwnedInner = $owned_inner;
            type BorrowedInner = $borrowed_inner;
        }

        impl<Inner: AsRef<$borrowed_inner>> AsRef<$borrowed_inner> for $name<Inner> {
            fn as_ref(&self) -> &$borrowed_inner {
                self.inner().as_ref()
            }
        }
    };
}

wrapper_type!(
    Repository,
    RepositoryWrapper,
    OwnedRepository,
    BorrowedRepository,
    PathBuf,
    Path
);

wrapper_type!(
    Container,
    ContainerWrapper,
    OwnedContainer,
    BorrowedContainer,
    PathBuf,
    Path
);

wrapper_type!(
    FailedBuildRecord,
    FailedBuildRecordWrapper,
    OwnedFailedBuildRecord,
    BorrowedFailedBuildRecord,
    PathBuf,
    Path
);

wrapper_type!(
    Directory,
    DirectoryWrapper,
    OwnedDirectory,
    BorrowedDirectory,
    PathBuf,
    Path
);

wrapper_type!(
    Pacman,
    PacmanWrapper,
    OwnedPacman,
    BorrowedPacman,
    String,
    str
);

wrapper_type!(
    Packager,
    PackagerWrapper,
    OwnedPackager,
    BorrowedPackager,
    String,
    str
);

wrapper_type!(
    GitUrl,
    GitUrlWrapper,
    OwnedGitUrl,
    BorrowedGitUrl,
    String,
    str
);

wrapper_type!(
    AurName,
    AurNameWrapper,
    OwnedAurName,
    BorrowedAurName,
    String,
    str
);

type OwnedArchVec = Vec<String>;
type OwnedArchArray = [String];
wrapper_type!(
    ArchCollection,
    ArchCollectionWrapper,
    OwnedArchCollection,
    BorrowedArchCollection,
    OwnedArchVec,
    OwnedArchArray
);
impl<Item: Into<String>> FromIterator<Item> for OwnedArchCollection {
    fn from_iter<Iter: IntoIterator<Item = Item>>(iter: Iter) -> Self {
        iter.into_iter()
            .map(Into::into)
            .collect::<Vec<_>>()
            .pipe(OwnedArchCollection::from_inner)
    }
}
