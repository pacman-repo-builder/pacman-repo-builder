use super::super::{
    manifest::{Manifest, Member},
    srcinfo::{database::SimpleDatabase, SrcInfo},
};
use super::{read_srcinfo_texts, Pair};
use indexmap::{IndexMap, IndexSet};
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct DbInit<'a> {
    srcinfo_texts: Vec<Pair<String, Member<PathBuf>>>,
    srcinfo_collection: Vec<Pair<SrcInfo<&'a str>, &'a Member<PathBuf>>>,
}

impl<'a> DbInit<'a> {
    pub fn init(&'a mut self) -> Result<DbInitValue<'a>, DbInitError> {
        let DbInit {
            srcinfo_texts,
            srcinfo_collection,
        } = self;

        let mut error_count = 0;

        let manifest = match Manifest::from_env() {
            Ok(manifest) => manifest,
            Err(error) => {
                eprintln!("{}", error);
                return Err(DbInitError::ManifestLoadingFailure);
            }
        };

        *srcinfo_texts = read_srcinfo_texts(&manifest, |error| {
            eprintln!("{}", error);
            error_count += 1;
        });

        *srcinfo_collection = srcinfo_texts
            .iter()
            .map(|x| x.to_ref().map(String::as_str).map(SrcInfo))
            .collect();
        let mut database = SimpleDatabase::default();
        let mut duplications: IndexMap<String, IndexSet<PathBuf>> = Default::default();
        for pair in srcinfo_collection {
            let (srcinfo, member) = pair.to_ref().into_tuple();
            match database.insert_srcinfo(srcinfo, member.directory.as_path()) {
                Err(error) => {
                    eprintln!("⮾ Error in directory {:?}: {}", member.directory, error);
                    error_count += 1;
                }
                Ok(Some(removal_info)) => {
                    let pkgbase = removal_info.pkgbase.to_string();
                    let values = if let Some(values) = duplications.get_mut(&pkgbase) {
                        values
                    } else {
                        duplications.insert(pkgbase.clone(), Default::default());
                        duplications.get_mut(&pkgbase).unwrap()
                    };
                    values.insert(removal_info.db_value.directory.to_path_buf());
                }
                Ok(None) => {}
            }
        }

        if !duplications.is_empty() {
            eprintln!("⮾ Duplication detected");
            for (pkgbase, directories) in duplications.iter() {
                eprintln!("  * pkgbase: {}", pkgbase);
                for directory in directories.iter() {
                    eprintln!("    - directory: {}", directory.to_string_lossy());
                }
            }
            error_count += duplications.len();
        }

        Ok(DbInitValue {
            manifest,
            database,
            error_count,
        })
    }
}

pub struct DbInitValue<'a> {
    pub manifest: Manifest<PathBuf>,
    pub database: SimpleDatabase<'a>,
    pub error_count: usize,
}

#[derive(Debug, Copy, Clone)]
pub enum DbInitError {
    ManifestLoadingFailure,
}

impl DbInitError {
    pub fn code(self) -> i32 {
        match self {
            DbInitError::ManifestLoadingFailure => 2,
        }
    }
}
