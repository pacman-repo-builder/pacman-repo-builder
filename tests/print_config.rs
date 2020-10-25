use command_extra::CommandExtra;
use pipe_trait::*;
use std::{path::PathBuf, process::Command};

const EXE: &str = env!("CARGO_BIN_EXE_build-pacman-repo");
const ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn work_dir() -> PathBuf {
    ROOT.pipe(PathBuf::from)
        .join("tests")
        .join("fixtures")
        .join("print-config")
}

fn init() -> Command {
    Command::new(EXE)
        .with_current_dir(work_dir())
        .with_arg("print-config")
        .with_args(&["--repository", "repo/repo.db.tar.gz"])
        .with_args(&["--container", "mixed"])
        .with_args(&["--container", "pkgbuild-only"])
        .with_args(&["--container", "srcinfo-only"])
        .with_args(&["--container", "pkgbuild-and-srcinfo"])
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

fn inspect((stdout, stderr, success): (&str, &str, bool)) {
    eprintln!();
    eprintln!();
    eprintln!("STDOUT:\n\n{}\n\n", stdout);
    eprintln!("STDERR:\n\n{}\n\n", stderr);
    eprintln!("SUCCESS: {}\n\n", success);
}

#[test]
fn require_nothing() {
    let (stdout, stderr, success) = output(init());
    let actual = (stdout.trim(), stderr.trim(), success);
    inspect(actual);
    let expected = (
        include_str!("./expected-output/print-config/require-nothing.stdout.yaml").trim(),
        "",
        true,
    );
    assert_eq!(actual, expected);
}

#[test]
fn require_pkgbuild() {
    let (stdout, stderr, success) = init().with_arg("--require-pkgbuild").pipe(output);
    let actual = (stdout.trim(), stderr.trim(), success);
    inspect(actual);
    let expected = (
        include_str!("./expected-output/print-config/require-pkgbuild.stdout.yaml").trim(),
        "",
        true,
    );
    assert_eq!(actual, expected);
}

#[test]
fn require_srcinfo() {
    let (stdout, stderr, success) = init().with_arg("--require-srcinfo").pipe(output);
    let actual = (stdout.trim(), stderr.trim(), success);
    inspect(actual);
    let expected = (
        include_str!("./expected-output/print-config/require-srcinfo.stdout.yaml").trim(),
        "",
        true,
    );
    assert_eq!(actual, expected);
}

#[test]
fn require_pkgbuild_and_srcinfo() {
    let (stdout, stderr, success) = init()
        .with_arg("--require-pkgbuild")
        .with_arg("--require-srcinfo")
        .pipe(output);
    let actual = (stdout.trim(), stderr.trim(), success);
    inspect(actual);
    let expected = (
        include_str!("./expected-output/print-config/require-pkgbuild-and-srcinfo.stdout.yaml")
            .trim(),
        "",
        true,
    );
    assert_eq!(actual, expected);
}

#[test]
fn with_flags() {
    let (stdout, stderr, success) = init()
        .with_args(&["--with-record-failed-builds", "failed-builds.yaml"])
        .with_args(&["--with-install-missing-dependencies", "false"])
        .with_args(&["--with-clean-before-build", "true"])
        .with_args(&["--with-clean-after-build", "false"])
        .with_args(&["--with-force-rebuild", "true"])
        .with_args(&["--with-pacman", "pacman"])
        .with_args(&["--with-packager", "Bob <bob@example.com>"])
        .with_args(&["--with-allow-failure", "false"])
        .with_args(&["--with-dereference-database-symlinks", "true"])
        .pipe(output);
    let actual = (stdout.trim(), stderr.trim(), success);
    inspect(actual);
    let expected = (
        include_str!("./expected-output/print-config/with-flags.stdout.yaml").trim(),
        "",
        true,
    );
    assert_eq!(actual, expected);
}

#[test]
fn with_arch_filter_any() {
    let (stdout, stderr, success) = init()
        .with_args(&["--with-arch-filter", "any"])
        .pipe(output);
    let actual = (stdout.trim(), stderr.trim(), success);
    inspect(actual);
    let expected = (
        include_str!("./expected-output/print-config/with-arch-filter-any.stdout.yaml").trim(),
        "",
        true,
    );
    assert_eq!(actual, expected);
}

#[test]
fn with_arch_filter_x86_64_any_i686_any() {
    let (stdout, stderr, success) = init()
        .with_args(&["--with-arch-filter", "x86_64"])
        .with_args(&["--with-arch-filter", "any"])
        .with_args(&["--with-arch-filter", "i686"])
        .with_args(&["--with-arch-filter", "any"])
        .pipe(output);
    let actual = (stdout.trim(), stderr.trim(), success);
    inspect(actual);
    let expected = (
        include_str!("./expected-output/print-config/with-arch-filter-any.stdout.yaml").trim(),
        "",
        true,
    );
    assert_eq!(actual, expected);
}

#[test]
fn with_arch_filter_x86_64_i686() {
    let (stdout, stderr, success) = init()
        .with_args(&["--with-arch-filter", "x86_64"])
        .with_args(&["--with-arch-filter", "i686"])
        .pipe(output);
    let actual = (stdout.trim(), stderr.trim(), success);
    inspect(actual);
    let expected = (
        include_str!("./expected-output/print-config/with-arch-filter-x86_64-i686.stdout.yaml")
            .trim(),
        "",
        true,
    );
    assert_eq!(actual, expected);
}
