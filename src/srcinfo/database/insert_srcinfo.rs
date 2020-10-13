use super::super::SrcInfo;
use super::{
    text_wrapper::{PkgBase, PkgName},
    SimpleDatabase,
};
use indexmap::IndexSet;
use pipe_trait::*;
use std::{
    fmt::{self, Display, Formatter},
    path::PathBuf,
};

impl<'a> SimpleDatabase<'a> {
    pub fn insert_srcinfo(
        &mut self,
        srcinfo: &'a SrcInfo<&'a str>,
        build_directory: PathBuf,
    ) -> Result<Option<RemovedInfo>, InsertionError> {
        let pkgbase = srcinfo
            .pkgbase()
            .ok_or(InsertionError::MissingPkgBase)?
            .pipe(PkgBase);

        let removed_srcinfo = self.infos.insert(pkgbase, *srcinfo);
        let removed_build_directory = self.build_directories.insert(pkgbase, build_directory);

        let dependency_list = if let Some(dependency_list) = self.dependencies.get_mut(&pkgbase) {
            dependency_list
        } else {
            self.dependencies.insert(pkgbase, Default::default());
            self.dependencies.get_mut(&pkgbase).unwrap()
        };

        let mut new_names = IndexSet::<PkgName>::new();

        for pkgname in srcinfo.pkgname() {
            let pkgname = PkgName(pkgname);

            new_names.insert(pkgname);
            self.name_to_base.insert(pkgname, pkgbase);

            for dependency in srcinfo.all_required_dependencies() {
                let bases = &self.name_to_base;
                let dependency_pkgbase = dependency
                    .name()
                    .pipe(PkgName)
                    .pipe_ref(|name| bases.get(name));
                if let Some(dependency_pkgbase) = dependency_pkgbase {
                    dependency_list.insert(*dependency_pkgbase);
                }
            }
        }

        let removed_names = self.base_to_name.insert(pkgbase, new_names);

        Ok(
            match (removed_srcinfo, removed_names, removed_build_directory) {
                (None, None, None) => None,
                (Some(srcinfo), Some(names), Some(directory)) => Some(RemovedInfo {
                    srcinfo,
                    names,
                    directory,
                }),
                (srcinfo, ref names, ref directory) => {
                    dbg!(srcinfo, names, directory);
                    panic!("impossible state reached");
                }
            },
        )
    }
}

#[derive(Debug, Clone)]
pub struct RemovedInfo<'a> {
    pub srcinfo: SrcInfo<&'a str>,
    pub names: IndexSet<PkgName<'a>>,
    pub directory: PathBuf,
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
