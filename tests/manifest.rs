use pacman_repo_builder::{
    manifest::{
        build_metadata::BuildMetadata, global_settings::GlobalSettings, member::Member,
        repository::Repository, Manifest,
    },
    utils::{deserialize_multi_docs_yaml, serialize_iter_yaml},
};
use pipe_trait::*;
use std::path::PathBuf;

fn manifest_list_yaml() -> &'static str {
    include_str!("./assets/manifest-list.yaml").trim()
}

fn manifest_list() -> impl Iterator<Item = Manifest<PathBuf>> {
    let make_members = || {
        vec![
            Member {
                directory: PathBuf::from("foo"),
                read_build_metadata: None,
                repository: None,
            },
            Member {
                directory: PathBuf::from("bar"),
                read_build_metadata: Some(BuildMetadata::PkgBuild),
                repository: "single-repo"
                    .pipe(PathBuf::from)
                    .pipe(Repository::Single)
                    .pipe(Some),
            },
            Member {
                directory: PathBuf::from("baz"),
                read_build_metadata: Some(BuildMetadata::SrcInfo),
                repository: ["repo1", "repo2", "repo3"]
                    .iter()
                    .map(PathBuf::from)
                    .collect::<Vec<_>>()
                    .pipe(Repository::Multiple)
                    .pipe(Some),
            },
        ]
    };

    [
        || None,
        || {
            Some(GlobalSettings {
                container: None,
                read_build_metadata: None,
                repository: None,
            })
        },
        || {
            Some(GlobalSettings {
                container: "container".pipe(PathBuf::from).pipe(Some),
                read_build_metadata: Some(BuildMetadata::Either),
                repository: ["abc", "def", "ghi"]
                    .iter()
                    .map(PathBuf::from)
                    .collect::<Vec<_>>()
                    .pipe(Repository::Multiple)
                    .pipe(Some),
            })
        },
    ]
    .iter()
    .map(move |make_global_settings| Manifest {
        global_settings: make_global_settings(),
        members: make_members(),
    })
}

#[test]
fn serialize() {
    let yaml = serialize_iter_yaml(manifest_list());
    assert_eq!(yaml.trim(), manifest_list_yaml());
}

#[test]
fn deserialize() {
    let actual = manifest_list_yaml()
        .pipe(deserialize_multi_docs_yaml::<Manifest<PathBuf>>)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let expected: Vec<_> = manifest_list().collect();
    assert_eq!(&actual, &expected);
}

#[test]
fn as_path() {
    let manifest_list: Vec<_> = manifest_list().collect();
    let actual = manifest_list
        .iter()
        .map(Manifest::as_path)
        .pipe(serialize_iter_yaml);
    let expected = serialize_iter_yaml(&manifest_list);
    assert_eq!(&actual, &expected);
}
