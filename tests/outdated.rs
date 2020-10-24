use command_extra::CommandExtra;
use pipe_trait::*;
use std::{path::PathBuf, process::Command};

const EXE: &str = env!("CARGO_BIN_EXE_build-pacman-repo");
const ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn work_dir(branch: &'static str) -> PathBuf {
    ROOT.pipe(PathBuf::from)
        .join("tests")
        .join("fixtures")
        .join("outdated")
        .join(branch)
}

fn init(branch: &'static str) -> Command {
    Command::new(EXE)
        .with_current_dir(work_dir(branch))
        .with_arg("outdated")
}

fn output(mut command: Command) -> (String, String, bool) {
    let output = command.output().expect("get output from a command");
    let stdout = output
        .stdout
        .pipe(String::from_utf8)
        .expect("convert stdout to UTF-8");
    let stderr = output
        .stderr
        .pipe(String::from_utf8)
        .expect("convert stderr to UTF-8");
    let success = output.status.success();
    (stdout, stderr, success)
}

macro_rules! test_case {
    ($name:ident, $branch:literal, $details:literal, $expected:literal) => {
        #[test]
        fn $name() {
            let (stdout, stderr, success) = init($branch)
                .with_arg("--details")
                .with_arg($details)
                .pipe(output);
            eprintln!("    ==> command stdout\n{}", stdout.as_str());
            eprintln!("    ==> command stderr\n{}", stderr.as_str());
            let actual = (stdout.as_str(), stderr.trim(), success);
            let expected = (include_str!($expected), "", true);
            assert_eq!(actual, expected);
        }
    };
}

test_case!(
    details_pkgname,
    "simple",
    "pkgname",
    "./expected-output/outdated/simple/details-pkgname.stdout.txt"
);

test_case!(
    details_pkg_file_path,
    "simple",
    "pkg-file-path",
    "./expected-output/outdated/simple/details-pkg-file-path.stdout.txt"
);

test_case!(
    details_lossy_yaml,
    "simple",
    "lossy-yaml",
    "./expected-output/outdated/simple/details-lossy-yaml.stdout.yaml"
);

test_case!(
    details_strict_yaml,
    "simple",
    "strict-yaml",
    "./expected-output/outdated/simple/details-strict-yaml.stdout.yaml"
);

test_case!(
    details_pkgname_arch_filter_any,
    "arch-filter-any",
    "pkgname",
    "./expected-output/outdated/arch-filter-any/details-pkgname.stdout.txt"
);

test_case!(
    details_pkg_file_path_arch_filter_any,
    "arch-filter-any",
    "pkg-file-path",
    "./expected-output/outdated/arch-filter-any/details-pkg-file-path.stdout.txt"
);

test_case!(
    details_lossy_yaml_arch_filter_any,
    "arch-filter-any",
    "lossy-yaml",
    "./expected-output/outdated/arch-filter-any/details-lossy-yaml.stdout.yaml"
);

test_case!(
    details_strict_yaml_arch_filter_any,
    "arch-filter-any",
    "strict-yaml",
    "./expected-output/outdated/arch-filter-any/details-strict-yaml.stdout.yaml"
);

test_case!(
    details_pkgname_arch_filter_x86_64,
    "arch-filter-x86_64",
    "pkgname",
    "./expected-output/outdated/arch-filter-x86_64/details-pkgname.stdout.txt"
);

test_case!(
    details_pkg_file_path_arch_filter_x86_64,
    "arch-filter-x86_64",
    "pkg-file-path",
    "./expected-output/outdated/arch-filter-x86_64/details-pkg-file-path.stdout.txt"
);

test_case!(
    details_lossy_yaml_arch_filter_x86_64,
    "arch-filter-x86_64",
    "lossy-yaml",
    "./expected-output/outdated/arch-filter-x86_64/details-lossy-yaml.stdout.yaml"
);

test_case!(
    details_strict_yaml_arch_filter_x86_64,
    "arch-filter-x86_64",
    "strict-yaml",
    "./expected-output/outdated/arch-filter-x86_64/details-strict-yaml.stdout.yaml"
);

#[test]
fn validate_yaml_output() {
    use serde_yaml::{from_str, Value};
    macro_rules! load {
        ($path:literal) => {
            include_str!($path)
                .split("---")
                .skip(1)
                .map(from_str::<Value>)
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
        };
    }
    assert_eq!(
        load!("./expected-output/outdated/simple/details-lossy-yaml.stdout.yaml"),
        load!("./expected-output/outdated/simple/details-strict-yaml.stdout.yaml"),
    );
}
