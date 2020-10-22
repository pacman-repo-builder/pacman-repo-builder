use pipe_trait::*;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

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

pub trait Associations {
    type Inner;
    type OwnedInner;
    type BorrowedInner: ?Sized;
}

macro_rules! wrapper_type {
    ($name:ident, $owned_alias:ident, $borrowed_alias:ident, $owned_inner:ident, $borrowed_inner:ident) => {
        #[derive(
            Debug, Default, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Copy, Clone,
        )]
        pub struct $name<Inner: AsRef<$borrowed_inner>>(Inner);
        pub type $owned_alias = $name<$owned_inner>;
        pub type $borrowed_alias<'a> = $name<&'a $borrowed_inner>;

        impl<Inner: AsRef<$borrowed_inner>> $name<Inner> {
            pub fn as_ref_wrapper(&self) -> $name<&$borrowed_inner> {
                self.0.as_ref().pipe($name)
            }

            pub fn to_owned_wrapper(&self) -> $name<$owned_inner> {
                self.0.as_ref().to_owned().pipe($name)
            }
        }

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
    OwnedRepository,
    BorrowedRepository,
    PathBuf,
    Path
);
wrapper_type!(Container, OwnedContainer, BorrowedContainer, PathBuf, Path);
wrapper_type!(Directory, OwnedDirectory, BorrowedDirectory, PathBuf, Path);
wrapper_type!(Pacman, OwnedPacman, BorrowedPacman, String, str);
wrapper_type!(Packager, OwnedPackager, BorrowedPackager, String, str);
