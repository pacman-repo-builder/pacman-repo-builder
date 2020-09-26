use std::fmt::{self, Display, Formatter};

macro_rules! make_wrapper {
    ($typename:ident) => {
        #[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
        pub struct $typename<'a>(pub &'a str);

        impl<'a> AsRef<str> for $typename<'a> {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl<'a> Display for $typename<'a> {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.as_ref())
            }
        }
    };
}

make_wrapper!(PkgBase);
make_wrapper!(PkgName);
