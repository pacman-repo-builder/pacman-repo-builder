use super::super::SrcInfo;
use super::{
    text_wrapper::{PkgBase, PkgName},
    SimpleDatabase, SimpleDatabaseValue,
};
use indexmap::IndexSet;
use pipe_trait::*;
use std::{
    fmt::{self, Display, Formatter},
    path::Path,
};

impl<'a> SimpleDatabase<'a> {
    pub fn insert_srcinfo(
        &mut self,
        srcinfo: &'a SrcInfo<&'a str>,
        directory: &'a Path,
    ) -> Result<Option<SimpleDatabaseValue<'a>>, InsertionError> {
        let pkgbase = srcinfo
            .pkgbase()
            .ok_or(InsertionError::MissingPkgBase)?
            .pipe(PkgBase);

        let mut dependencies = IndexSet::<PkgBase>::new();
        let mut names = IndexSet::<PkgName>::new();

        for pkgname in srcinfo.pkgname() {
            let pkgname = PkgName(pkgname);
            names.insert(pkgname);
            self.pkgname.insert(pkgname, pkgbase);

            for dependency in srcinfo.all_required_dependencies() {
                if let Some(dependency_pkgbase) = dependency
                    .name()
                    .pipe(PkgName)
                    .pipe_ref(|name| self.pkgname.get(name))
                    .copied()
                {
                    dependencies.insert(dependency_pkgbase);
                }
            }
        }

        Ok(self.pkgbase.insert(
            pkgbase,
            SimpleDatabaseValue {
                names,
                dependencies,
                srcinfo: *srcinfo,
                directory,
            },
        ))
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum InsertionError {
    MissingPkgBase,
}

impl Display for InsertionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            InsertionError::MissingPkgBase => write!(f, "missing pkgbase"),
        }
    }
}
