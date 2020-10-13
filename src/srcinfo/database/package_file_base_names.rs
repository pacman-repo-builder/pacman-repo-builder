use super::super::super::utils::PackageFileName;
use super::super::SrcInfo;
use super::{Database, DatabaseValue};
use pipe_trait::*;
use std::{hash::Hash, path::Path};

impl<PkgBase, PkgName, SrcInfoContent, BuildDir>
    Database<PkgBase, PkgName, SrcInfoContent, BuildDir>
where
    PkgBase: AsRef<str> + Hash + Eq + Clone,
    PkgName: AsRef<str> + Hash + Eq + Clone,
    SrcInfoContent: AsRef<str>,
    BuildDir: AsRef<Path>,
{
    pub fn package_file_base_names(
        &self,
    ) -> impl Iterator<
        Item = Result<PackageFileName<&str, String, &str>, Error<PkgBase, SrcInfoContent>>,
    > + '_ {
        self.pkgbase()
            .iter()
            .flat_map(|(pkgbase, value)| -> Box<dyn Iterator<Item = _>> {
                let DatabaseValue { srcinfo, .. } = value;
                match srcinfo.package_file_base_names() {
                    Ok(iter) => iter.map(Ok).pipe(Box::new),
                    Err(message) => Error {
                        pkgbase,
                        srcinfo,
                        message,
                    }
                    .pipe(Err)
                    .pipe(std::iter::once)
                    .pipe(Box::new),
                }
            })
    }
}

#[derive(Debug, Clone)]
pub struct Error<'a, PkgBase, SrcInfoContent>
where
    SrcInfoContent: AsRef<str>,
{
    pub pkgbase: &'a PkgBase,
    pub srcinfo: &'a SrcInfo<SrcInfoContent>,
    pub message: String,
}
