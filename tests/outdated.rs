use command_extra::CommandExtra;
use pipe_trait::*;
use std::{path::PathBuf, process::Command};

const EXE: &str = env!("CARGO_BIN_EXE_build-pacman-repo");
const ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn work_dir() -> PathBuf {
    ROOT.pipe(PathBuf::from)
        .join("tests")
        .join("fixtures")
        .join("outdated")
}

fn init() -> Command {
    Command::new(EXE)
        .with_current_dir(work_dir())
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
    ($name:ident, $details:literal, $expected:literal) => {
        #[test]
        fn $name() {
            let (stdout, stderr, success) =
                init().with_arg("--details").with_arg($details).pipe(output);
            let actual = (stdout.as_str(), stderr.trim(), success);
            let expected = (include_str!($expected), "", true);
            assert_eq!(actual, expected);
        }
    };
}

test_case!(
    details_pkgname,
    "pkgname",
    "./expected-output/outdated/details-pkgname.stdout.txt"
);

test_case!(
    details_pkg_file_path,
    "pkg-file-path",
    "./expected-output/outdated/details-pkg-file-path.stdout.txt"
);

test_case!(
    details_lossy_yaml,
    "lossy-yaml",
    "./expected-output/outdated/details-lossy-yaml.stdout.yaml"
);

test_case!(
    details_strict_yaml,
    "strict-yaml",
    "./expected-output/outdated/details-strict-yaml.stdout.yaml"
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
        load!("./expected-output/outdated/details-lossy-yaml.stdout.yaml"),
        load!("./expected-output/outdated/details-strict-yaml.stdout.yaml"),
    );
}
