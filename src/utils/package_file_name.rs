use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct PackageFileName<PkgName, Version, Arch>
where
    PkgName: Display,
    Version: Display,
    Arch: Display,
{
    pub pkgname: PkgName,
    pub version: Version,
    pub arch: Arch,
}

impl<PkgName, Version, Arch> PackageFileName<PkgName, Version, Arch>
where
    PkgName: Display,
    Version: Display,
    Arch: Display,
{
    pub fn fmt_without_ext(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}-{}-{}", self.pkgname, self.version, self.arch)
    }

    pub fn fmt_with_ext(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_without_ext(formatter)?;
        write!(formatter, ".pkg.tar.zst")
    }
}

impl<PkgName, Version, Arch> Display for PackageFileName<PkgName, Version, Arch>
where
    PkgName: Display,
    Version: Display,
    Arch: Display,
{
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        self.fmt_with_ext(formatter)
    }
}

#[test]
fn test_to_string() {
    let actual = PackageFileName {
        pkgname: "foo",
        version: "0.1.2-3",
        arch: "x86_64",
    }
    .to_string();
    let expected = "foo-0.1.2-3-x86_64.pkg.tar.zst";
    assert_eq!(actual, expected);
}
