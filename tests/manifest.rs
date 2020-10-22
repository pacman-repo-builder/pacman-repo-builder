use pacman_repo_builder::{
    manifest::{
        BuildMetadata, Manifest, OwnedContainer, OwnedGlobalSettings, OwnedManifest, OwnedMember,
        Wrapper,
    },
    utils::{deserialize_multi_docs_yaml, serialize_iter_yaml},
};
use pipe_trait::*;
use std::path::PathBuf;

fn manifest_list_yaml() -> &'static str {
    include_str!("./assets/manifest-list.yaml").trim()
}

fn manifest_list() -> impl Iterator<Item = OwnedManifest> {
    let make_members = || {
        vec![
            OwnedMember {
                directory: "foo".pipe(PathBuf::from).pipe(Wrapper::from_inner),
                read_build_metadata: None,
            },
            OwnedMember {
                directory: "bar".pipe(PathBuf::from).pipe(Wrapper::from_inner),
                read_build_metadata: Some(BuildMetadata::PkgBuild),
            },
            OwnedMember {
                directory: "baz".pipe(PathBuf::from).pipe(Wrapper::from_inner),
                read_build_metadata: Some(BuildMetadata::SrcInfo),
            },
        ]
    };

    [
        || OwnedGlobalSettings {
            container: None,
            read_build_metadata: None,
            repository: "repo".pipe(PathBuf::from).pipe(Wrapper::from_inner),
            ..Default::default()
        },
        || OwnedGlobalSettings {
            container: "container"
                .pipe(PathBuf::from)
                .pipe(OwnedContainer::from_inner)
                .pipe(Some),
            read_build_metadata: Some(BuildMetadata::Either),
            repository: "repo".pipe(PathBuf::from).pipe(Wrapper::from_inner),
            ..Default::default()
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
        .pipe(deserialize_multi_docs_yaml::<OwnedManifest>)
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
