use super::super::{
    args::{ManifestLoader, SortArgs},
    manifest::{BuildMetadata, Member},
    srcinfo::{database::SimpleDatabase, SrcInfo},
};
use pipe_trait::*;
use std::fs::read;

pub fn sort(args: SortArgs) -> i32 {
    let mut error_count = 0u32;

    let SortArgs { config } = args;
    let ManifestLoader(manifest) = config;

    #[derive(Copy, Clone)]
    struct Tracker<X, D> {
        pub value: X,
        pub directory: D,
    }

    impl<X, D> Tracker<X, D> {
        fn new(directory: D, value: X) -> Self {
            Tracker { directory, value }
        }

        fn map<Y>(self, f: impl FnOnce(X) -> Y) -> Tracker<Y, D> {
            let Tracker { directory, value } = self;
            Tracker {
                directory,
                value: f(value),
            }
        }

        fn to_ref(&self) -> Tracker<&X, &D> {
            Tracker::new(&self.directory, &self.value)
        }
    }

    let mut srcinfo_texts = Vec::new();
    for member in manifest.resolve_members() {
        let Member {
            directory,
            read_build_metadata,
            ..
        } = member;

        let srcinfo_result = match read_build_metadata.unwrap_or(BuildMetadata::Either) {
            BuildMetadata::Either => unimplemented!(),
            BuildMetadata::PkgBuild => unimplemented!(),
            BuildMetadata::SrcInfo => directory
                .join(".SRCINFO")
                .pipe_ref(read)
                .map_err(|error| {
                    format!(
                        "cannot read file {:?}: {}",
                        directory.join(".SRCINFO"),
                        error,
                    )
                })
                .and_then(|content| {
                    content.pipe(String::from_utf8).map_err(|error| {
                        format!(
                            "cannot convert content of file {:?} into a UTF-8 text: {}",
                            directory.join(".SRCINFO"),
                            error
                        )
                    })
                }),
        };

        match srcinfo_result {
            Ok(content) => srcinfo_texts.push(Tracker::new(directory, content)),
            Err(error) => {
                eprintln!("{}", error);
                error_count += 1;
                continue;
            }
        };
    }

    let srcinfo_collection: Vec<_> = srcinfo_texts
        .iter()
        .map(|x| x.to_ref().map(String::as_str).map(SrcInfo))
        .collect();
    let mut database = SimpleDatabase::default();
    for Tracker { directory, value } in &srcinfo_collection {
        if let Err(error) = database.insert_srcinfo(value) {
            eprintln!("error in directory {:?}: {}", directory, error);
            error_count += 1;
        }
    }

    for item in database.into_build_order().0 {
        println!("{}", item);
    }

    if error_count == 0 {
        0
    } else {
        eprintln!("{} errors occurred", error_count);
        1
    }
}
