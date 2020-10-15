use pacman_repo_builder::{
    manifest::{BuildMetadata, GlobalSettings, Manifest, Member},
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
            },
            Member {
                directory: PathBuf::from("bar"),
                read_build_metadata: Some(BuildMetadata::PkgBuild),
            },
            Member {
                directory: PathBuf::from("baz"),
                read_build_metadata: Some(BuildMetadata::SrcInfo),
            },
        ]
    };

    [
        || GlobalSettings {
            container: None,
            read_build_metadata: None,
            repository: PathBuf::from("repo"),
        },
        || GlobalSettings {
            container: "container".pipe(PathBuf::from).pipe(Some),
            read_build_metadata: Some(BuildMetadata::Either),
            repository: PathBuf::from("repo"),
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
    let yaml = serialize_iter_yaml(manifest_list()).unwrap();
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
fn as_path_serialize() {
    let manifest_list: Vec<_> = manifest_list().collect();
    let actual = manifest_list
        .iter()
        .map(Manifest::as_path)
        .pipe(serialize_iter_yaml)
        .unwrap();
    let expected = serialize_iter_yaml(&manifest_list).unwrap();
    assert_eq!(&actual, &expected);
}

#[test]
fn resolve_members() {
    let actual = manifest_list()
        .map(|x| x.resolve_members().collect::<Vec<_>>())
        .pipe(serialize_iter_yaml)
        .unwrap();
    let expected = include_str!("./assets/resolved-members.yaml");
    assert_eq!(actual.trim(), expected.trim());
}
