use super::super::{
    manifest::Manifest,
    srcinfo::{database::SimpleDatabase, SrcInfo},
};
use super::{read_srcinfo_texts, Pair};
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct DbInit<'a> {
    srcinfo_texts: Vec<Pair<String, PathBuf>>,
    srcinfo_collection: Vec<Pair<SrcInfo<&'a str>, &'a PathBuf>>,
}

impl<'a> DbInit<'a> {
    pub fn init(&'a mut self) -> Result<DbInitValue<'a>, DbInitError> {
        let DbInit {
            srcinfo_texts,
            srcinfo_collection,
        } = self;

        let mut error_count = 0u32;

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
        for pair in srcinfo_collection {
            let (srcinfo, directory) = pair.to_ref().into_tuple();
            if let Err(error) = database.insert_srcinfo(srcinfo, directory.to_path_buf()) {
                eprintln!("error in directory {:?}: {}", directory, error);
                error_count += 1;
            }
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
    pub error_count: u32,
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
