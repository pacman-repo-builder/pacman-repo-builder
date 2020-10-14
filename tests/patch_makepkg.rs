use pacman_repo_builder::utils::{CommandUtils, CUSTOM_MAKEPKG};
use pipe_trait::*;
use std::process::Command;

const EXE: &str = env!("CARGO_BIN_EXE_build-pacman-repo");

fn init() -> Command {
    Command::new(EXE).with_arg("patch-makepkg")
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

#[test]
fn print_makepkg() {
    let (stdout, stderr, success) = output(init());
    let actual_stderr_lines = stderr.lines().collect::<Vec<_>>();
    let actual = (stdout.as_str(), actual_stderr_lines.as_slice(), success);
    let expected_stderr_lines: &[&str] = &[
        "",
        "# NOTE: Above is the content of custom makepkg script",
        "# NOTE: Run again with --replace flag to replace system's makepkg",
    ];
    let expected = (CUSTOM_MAKEPKG, expected_stderr_lines, true);
    assert_eq!(actual, expected);
}
