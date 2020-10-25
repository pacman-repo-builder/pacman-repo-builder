use super::FailedBuildRecordItem;
use std::fmt::Display;

pub fn outdated_packages<'a, Latest: ToString>(
    latest_packages: impl IntoIterator<Item = Latest> + 'a,
    current_packages: &'a [String],
    failed_builds: &'a [FailedBuildRecordItem<impl Display, impl Display, impl Display>],
) -> impl Iterator<Item = (String, Latest)> + 'a {
    let not_failed = move |filename: &String| {
        failed_builds
            .iter()
            .map(|x| &x.package_file_name)
            .all(|x| &x.to_string() != filename)
    };

    latest_packages
        .into_iter()
        .map(|latest| (latest.to_string(), latest))
        .filter(move |(filename, _)| !current_packages.contains(filename))
        .filter(move |(filename, _)| not_failed(filename))
}

#[test]
fn test() {
    use super::PackageFileName;

    let latest_packages = [
        PackageFileName {
            pkgname: "abc",
            version: "1.2.3-4",
            arch: "x86_64",
        },
        PackageFileName {
            pkgname: "def",
            version: "4.3.2-1",
            arch: "any",
        },
        PackageFileName {
            pkgname: "ghi",
            version: "0.0.0-1",
            arch: "i686",
        },
        PackageFileName {
            pkgname: "jkl",
            version: "3.3.3-3",
            arch: "any",
        },
    ];

    let current_packages = [
        "abc-1.2.3-4-x86_64.pkg.tar.zst".to_string(),
        "def-1.2.3-4-any.pkg.tar.zst".to_string(),
        "ghi-0.0.0-1-x86_64.pkg.tar.zst".to_string(),
        "jkl-0.0.0-1-any.pkg.tar.zst".to_string(),
    ];

    let failed_builds = [PackageFileName {
        pkgname: "jkl",
        version: "3.3.3-3",
        arch: "any",
    }
    .into()];

    let actual: Vec<_> =
        outdated_packages(&latest_packages, &current_packages, &failed_builds).collect();

    let expected = [
        (
            "def-4.3.2-1-any.pkg.tar.zst".to_string(),
            &PackageFileName {
                pkgname: "def",
                version: "4.3.2-1",
                arch: "any",
            },
        ),
        (
            "ghi-0.0.0-1-i686.pkg.tar.zst".to_string(),
            &PackageFileName {
                pkgname: "ghi",
                version: "0.0.0-1",
                arch: "i686",
            },
        ),
    ];

    assert_eq!(&actual, &expected);
}
