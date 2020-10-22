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
                install_missing_dependencies: None,
                clean_before_build: None,
                clean_after_build: None,
                force_rebuild: None,
                pacman: None,
                packager: None,
                allow_failure: None,
            },
            OwnedMember {
                directory: "bar".pipe(PathBuf::from).pipe(Wrapper::from_inner),
                read_build_metadata: Some(BuildMetadata::PkgBuild),
                install_missing_dependencies: None,
                clean_before_build: None,
                clean_after_build: None,
                force_rebuild: None,
                pacman: None,
                packager: None,
                allow_failure: None,
            },
            OwnedMember {
                directory: "baz".pipe(PathBuf::from).pipe(Wrapper::from_inner),
                read_build_metadata: Some(BuildMetadata::SrcInfo),
                install_missing_dependencies: None,
                clean_before_build: None,
                clean_after_build: None,
                force_rebuild: None,
                pacman: None,
                packager: None,
                allow_failure: None,
            },
        ]
    };

    [
        || OwnedGlobalSettings {
            container: None,
            read_build_metadata: None,
            repository: "repo/repo.db.tar.gz"
                .pipe(PathBuf::from)
                .pipe(Wrapper::from_inner),
            install_missing_dependencies: None,
            clean_before_build: None,
            clean_after_build: None,
            force_rebuild: None,
            pacman: None,
            packager: None,
            allow_failure: None,
            dereference_database_symlinks: None,
        },
        || OwnedGlobalSettings {
            container: "container"
                .pipe(PathBuf::from)
                .pipe(OwnedContainer::from_inner)
                .pipe(Some),
            read_build_metadata: Some(BuildMetadata::Either),
            repository: "repo/repo.db.tar.gz"
                .pipe(PathBuf::from)
                .pipe(Wrapper::from_inner),
            install_missing_dependencies: None,
            clean_before_build: None,
            clean_after_build: None,
            force_rebuild: None,
            pacman: None,
            packager: None,
            allow_failure: None,
            dereference_database_symlinks: None,
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
    let actual = yaml.trim();
    eprintln!("\n\nACTUAL:\n\n{}\n\n", actual);
    assert_eq!(actual, manifest_list_yaml());
}

#[test]
fn deserialize() {
    let actual = manifest_list_yaml()
        .pipe(deserialize_multi_docs_yaml::<OwnedManifest>)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let expected: Vec<_> = manifest_list().collect();
    dbg!(&actual);
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
