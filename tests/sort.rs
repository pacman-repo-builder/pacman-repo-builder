use pipe_trait::*;
use std::{collections::BTreeSet, iter::FromIterator, path::PathBuf, process::Command};

const EXE: &str = env!("CARGO_BIN_EXE_build-pacman-repo");
const ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn work_dir() -> PathBuf {
    ROOT.pipe(PathBuf::from)
        .join("tests")
        .join("fixtures")
        .join("sort")
}

fn init() -> Command {
    let mut command = Command::new(EXE);
    command.current_dir(work_dir()).arg("sort");
    command
}

fn output(command: &mut Command) -> (String, String, bool) {
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

fn collect<'a, Result>(stdout: &'a str, filter: impl Fn(&&str) -> bool) -> Result
where
    Result: FromIterator<&'a str>,
{
    stdout
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .filter(filter)
        .collect()
}

macro_rules! test_case {
    ($name:ident, $typename:ident, $filter:expr, $expected:expr) => {
        #[test]
        fn $name() {
            let (stdout, _, _) = output(&mut init());
            let actual: $typename<_> = collect(&stdout, $filter);
            let expected: $typename<_> = $expected;
            assert_eq!(actual, expected);
        }
    };
}

macro_rules! test_order {
    ($name:ident, $filter:expr, $expected:expr) => {
        test_case!($name, Vec, $filter, $expected);
    };
}

test_case!(
    standalone,
    BTreeSet,
    |line| line.starts_with("standalone-"),
    vec!["standalone-multi", "standalone-single"]
        .into_iter()
        .collect()
);

test_order!(
    level_multi_postfix,
    |line| line.ends_with("-level-multi"),
    vec![
        "top-level-multi",
        "middle-level-multi",
        "bottom-level-multi",
    ]
);

test_order!(
    level_single_postfix,
    |line| line.ends_with("-level-single"),
    vec![
        "top-level-single",
        "middle-level-single",
        "bottom-level-single",
    ]
);

test_order!(
    pkgbuild_prefix,
    |line| line.starts_with("pkgbuild-"),
    vec!["pkgbuild-top", "pkgbuild-middle", "pkgbuild-bottom"]
);

test_order!(
    require_external_prefix,
    |line| line.starts_with("require-external-"),
    vec![
        "require-external-top",
        "require-external-middle",
        "require-external-bottom",
    ]
);

#[test]
fn stderr() {
    let (_, stderr, _) = output(&mut init());
    assert!(stderr.trim().is_empty(), "stderr is empty");
}

#[test]
fn success() {
    let (_, _, success) = output(&mut init());
    assert!(success, "process exit with success status");
}
