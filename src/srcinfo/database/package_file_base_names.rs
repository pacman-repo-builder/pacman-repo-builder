use super::super::SrcInfo;
use super::Database;
use pipe_trait::*;
use std::hash::Hash;

impl<PkgBase, PkgName, SrcInfoContent> Database<PkgBase, PkgName, SrcInfoContent>
where
    PkgBase: AsRef<str> + Default + Hash + Eq + Clone,
    PkgName: AsRef<str> + Default + Hash + Eq + Clone,
    SrcInfoContent: AsRef<str> + Default,
{
    pub fn package_file_base_names(
        &self,
    ) -> impl Iterator<Item = Result<String, Error<PkgBase, SrcInfoContent>>> + '_ {
        self.infos()
            .iter()
            .flat_map(|(pkgbase, srcinfo)| -> Box<dyn Iterator<Item = _>> {
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
