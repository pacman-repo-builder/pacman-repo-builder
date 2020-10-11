use pacman_repo_builder::manifest::Manifest;
use pipe_trait::*;
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Display,
    fs::{read_to_string, File},
    path::{Path, PathBuf},
    process::Command,
};
use tempfile::TempDir;

const EXE: &str = env!("CARGO_BIN_EXE_build-pacman-repo");
const ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn fixtures(branch: &'static str) -> PathBuf {
    ROOT.pipe(PathBuf::from)
        .join("tests")
        .join("fixtures")
        .join("sync-srcinfo")
        .join(branch)
}

fn fixture_manifest(branch: &'static str) -> Manifest<PathBuf> {
    branch
        .pipe(fixtures)
        .join("build-pacman-repo.yaml")
        .pipe(File::open)
        .expect("open manifest file")
        .pipe(serde_yaml::from_reader)
        .expect("parse manifest")
}

struct Context {
    command: Command,
    work_dir: TempDir,
}

impl Context {
    fn new(branch: &'static str) -> Self {
        let mut command = Command::new(EXE);
        let work_dir = TempDir::new().expect("create temporary directory for context");
        eprintln!("Current Working Directory: {:?}", work_dir.path());
        fs_extra::dir::copy(
            fixtures(branch),
            work_dir.path(),
            &fs_extra::dir::CopyOptions {
                content_only: true,
                overwrite: true,
                ..Default::default()
            },
        )
        .expect("copy fixtures to working directory");
        command.current_dir(work_dir.path()).arg("sync-srcinfo");
        Context { command, work_dir }
    }

    fn output(&mut self) -> (String, String, i32) {
        let output = self.command.output().expect("get output from a command");
        let stdout = output
            .stdout
            .pipe(String::from_utf8)
            .expect("convert stdout to UTF-8");
        let stderr = output
            .stderr
            .pipe(String::from_utf8)
            .expect("convert stderr to UTF-8");
        let status = output.status.code().expect("get status code");
        (stdout, stderr, status)
    }

    fn arg(mut self, arg: &'static str) -> Self {
        self.command.arg(arg);
        self
    }

    fn manifest(&self) -> Manifest<PathBuf> {
        self.work_dir
            .path()
            .join("build-pacman-repo.yaml")
            .pipe(File::open)
            .expect("open manifest file")
            .pipe(serde_yaml::from_reader)
            .expect("parse manifest")
    }
}

macro_rules! test_check {
    ($name:ident, $branch:literal, $expected_outdated:expr, $expected_status:expr) => {
        #[test]
        fn $name() {
            let (stdout, stderr, status) = Context::new($branch).output();
            let actual_outdated: BTreeSet<_> = stdout.trim().lines().collect();
            let actual = (actual_outdated, stderr.trim(), status);
            let expected_outdated: BTreeSet<_> = $expected_outdated.into_iter().collect();
            let expected = (expected_outdated, "", $expected_status);
            assert_eq!(actual, expected);
        }
    };
}

test_check!(check_all_sync, "all-sync", Vec::new(), 0);
test_check!(
    check_none_sync,
    "none-sync",
    vec!["multiple-packages", "single-package"],
    3
);
test_check!(
    check_some_sync,
    "some-sync",
    vec!["outdated-multi", "outdated-single"],
    3
);

fn read_srcinfo_files<DirList>(
    prefix: impl AsRef<Path>,
    directories: DirList,
) -> BTreeMap<DirList::Item, String>
where
    DirList: IntoIterator,
    DirList::Item: AsRef<str> + Ord + Display,
{
    directories
        .into_iter()
        .map(|name| {
            (
                prefix
                    .as_ref()
                    .join(name.as_ref())
                    .join(".SRCINFO")
                    .pipe(read_to_string)
                    .unwrap_or_else(|error| panic!("fail to read {}/.SRCINFO: {}", name, error)),
                name,
            )
        })
        .map(|(value, key)| (key, value))
        .collect()
}

macro_rules! test_update {
    ($name:ident, $branch:literal, $expected_outdated:expr) => {
        #[test]
        fn $name() {
            let mut context = Context::new($branch).arg("--update");
            let (stdout, stderr, status) = context.output();
            let actual_outdated: BTreeSet<_> = stdout.trim().lines().collect();
            let actual_modified =
                read_srcinfo_files(&context.work_dir, actual_outdated.iter().copied());
            let actual_unmodified = read_srcinfo_files(
                &context.work_dir,
                context
                    .manifest()
                    .resolve_members()
                    .map(|member| member.directory.to_string_lossy().to_string())
                    .filter(|name| !actual_outdated.contains(name.as_str())),
            );
            let actual = (actual_outdated, stderr.trim(), status);

            let expected_outdated: BTreeSet<_> = $expected_outdated.into_iter().collect();
            let expected_modified = read_srcinfo_files(fixtures($branch), $expected_outdated);
            let expected_unmodified = read_srcinfo_files(
                fixtures($branch),
                $branch
                    .pipe(fixture_manifest)
                    .resolve_members()
                    .map(|member| member.directory.to_string_lossy().to_string())
                    .filter(|name| !expected_outdated.contains(name.as_str())),
            );
            let expected = (expected_outdated, "", 0);

            assert_eq!(actual, expected, "process output");
            if (actual_modified.len(), expected_modified.len()) != (0, 0) {
                assert_ne!(&actual_modified, &expected_modified, "modified parts");
            }
            assert_eq!(&actual_unmodified, &expected_unmodified, "unmodified parts");
        }
    };
}

test_update!(update_all_sync, "all-sync", Vec::new());
test_update!(
    update_none_sync,
    "none-sync",
    vec!["multiple-packages", "single-package"]
);
test_update!(
    update_some_sync,
    "some-sync",
    vec!["outdated-multi", "outdated-single"]
);
