use pacman_repo_builder::{args::Args, clap::Shell::*, structopt::StructOpt};

macro_rules! test_case {
    ($test_name:ident, $shell:expr, $path:literal) => {
        #[test]
        fn $test_name() {
            let expected: &[u8] = include_bytes!($path);
            let mut actual = Vec::new();
            Args::clap().gen_completions_to("strip-ansi", $shell, &mut actual);
            let actual = actual.as_slice();
            assert_eq!(actual, expected);
        }
    };
}

test_case!(bash, Bash, "../exports/completion.bash");
test_case!(fish, Fish, "../exports/completion.fish");
test_case!(zsh, Zsh, "../exports/completion.zsh");
test_case!(powershell, PowerShell, "../exports/completion.ps1");
test_case!(elvish, Elvish, "../exports/completion.elv");
