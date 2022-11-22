use pacman_repo_builder::{
    manifest::{
        ArchFilter, BorrowedInner, BuildMetadata, BuildPacmanRepo, OwnedBuildPacmanRepo,
        OwnedContainer, OwnedFailedBuildRecord, OwnedGlobalSettings, OwnedInitAurBuilder,
        OwnedMember, TriState, Wrapper,
    },
    utils::{deserialize_multi_docs_yaml, serialize_iter_yaml},
};
use pipe_trait::*;
use pretty_assertions::assert_eq;
use std::path::PathBuf;

fn manifest_list_yaml() -> &'static str {
    include_str!("./assets/manifest-list.yaml").trim()
}

fn manifest_list() -> impl Iterator<Item = OwnedBuildPacmanRepo> {
    let make_members = || {
        vec![
            OwnedMember {
                directory: "foo".pipe(PathBuf::from).pipe(Wrapper::from_inner),
                read_build_metadata: None,
                install_missing_dependencies: None,
                clean_before_build: None,
                clean_after_build: None,
                force_rebuild: None,
                check: None,
                pacman: None,
                allow_failure: None,
            },
            OwnedMember {
                directory: "bar".pipe(PathBuf::from).pipe(Wrapper::from_inner),
                read_build_metadata: Some(BuildMetadata::PkgBuild),
                install_missing_dependencies: None,
                clean_before_build: Some(false),
                clean_after_build: None,
                force_rebuild: Some(true),
                check: Some(TriState::Inherit),
                pacman: None,
                allow_failure: Some(false),
            },
            OwnedMember {
                directory: "bar".pipe(PathBuf::from).pipe(Wrapper::from_inner),
                read_build_metadata: None,
                install_missing_dependencies: Some(true),
                clean_before_build: None,
                clean_after_build: Some(false),
                force_rebuild: None,
                check: Some(TriState::Enabled),
                pacman: Some("yay".to_owned_wrapper()),
                allow_failure: None,
            },
            OwnedMember {
                directory: "baz".pipe(PathBuf::from).pipe(Wrapper::from_inner),
                read_build_metadata: Some(BuildMetadata::SrcInfo),
                install_missing_dependencies: Some(false),
                clean_before_build: Some(true),
                clean_after_build: Some(false),
                force_rebuild: Some(true),
                check: Some(TriState::Disabled),
                pacman: Some("yay".to_owned_wrapper()),
                allow_failure: Some(false),
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
            record_failed_builds: None,
            install_missing_dependencies: None,
            clean_before_build: None,
            clean_after_build: None,
            force_rebuild: None,
            arch_filter: None,
            check: None,
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
            record_failed_builds: None,
            install_missing_dependencies: Some(false),
            clean_before_build: None,
            clean_after_build: Some(false),
            force_rebuild: None,
            arch_filter: Some(ArchFilter::Any),
            check: Some(TriState::Inherit),
            pacman: Some("pacman".to_owned_wrapper()),
            packager: None,
            allow_failure: Some(true),
            dereference_database_symlinks: None,
        },
        || OwnedGlobalSettings {
            container: None,
            read_build_metadata: None,
            repository: "repo/repo.db.tar.gz"
                .pipe(PathBuf::from)
                .pipe(Wrapper::from_inner),
            record_failed_builds: "failed-builds.yaml"
                .pipe(PathBuf::from)
                .pipe(OwnedFailedBuildRecord::from_inner)
                .pipe(Some),
            install_missing_dependencies: None,
            clean_before_build: Some(true),
            clean_after_build: None,
            force_rebuild: Some(false),
            arch_filter: ArchFilter::from_str_iter(["x86_64", "i686"]),
            check: Some(TriState::Enabled),
            pacman: None,
            packager: Some("Bob <bob@example.com>".to_owned_wrapper()),
            allow_failure: None,
            dereference_database_symlinks: Some(false),
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
            record_failed_builds: "failed-builds.yaml"
                .pipe(PathBuf::from)
                .pipe(OwnedFailedBuildRecord::from_inner)
                .pipe(Some),
            install_missing_dependencies: Some(false),
            clean_before_build: Some(false),
            clean_after_build: Some(false),
            force_rebuild: Some(true),
            arch_filter: ArchFilter::from_str_iter(["x86_64", "i686"]),
            check: Some(TriState::Disabled),
            pacman: Some("pacman".to_owned_wrapper()),
            packager: Some("Bob <bob@example.com>".to_owned_wrapper()),
            allow_failure: Some(true),
            dereference_database_symlinks: Some(true),
        },
    ]
    .iter()
    .map(move |make_global_settings| BuildPacmanRepo {
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
        .pipe(deserialize_multi_docs_yaml::<OwnedBuildPacmanRepo>)
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
        .map(BuildPacmanRepo::as_borrowed)
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
    let actual = actual.trim();
    let expected = include_str!("./assets/resolved-members.yaml").trim();
    eprintln!("\n\nACTUAL:\n\n{}\n\n", actual);
    assert_eq!(actual, expected);
}

fn init_aur_builder() -> OwnedInitAurBuilder {
    OwnedInitAurBuilder::default()
        .with_global_settings(OwnedGlobalSettings {
            repository: "repo/repo.db.tar.gz"
                .pipe(PathBuf::from)
                .pipe(Wrapper::from_inner),
            ..Default::default()
        })
        .with_package("rust".to_string())
        .with_package("python".to_string())
        .with_package("node".to_string())
}

#[test]
fn init_aur_builder_deserialize() {
    let actual: OwnedInitAurBuilder = include_str!("./assets/init-aur-builder.yaml")
        .pipe(serde_yaml::from_str)
        .unwrap();
    let expected = init_aur_builder();
    assert_eq!(actual, expected);
}

#[test]
fn init_aur_builder_serialize() {
    let actual = init_aur_builder().pipe_ref(serde_yaml::to_string).unwrap();
    let actual = actual.trim();
    let expected = include_str!("./assets/init-aur-builder.yaml").trim();
    eprintln!("\n\nACTUAL:\n\n{}\n\n", actual);
    assert_eq!(actual, expected);
}
