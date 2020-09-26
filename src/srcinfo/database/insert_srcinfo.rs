use super::super::SrcInfo;
use super::{
    text_wrapper::{PkgBase, PkgName},
    Database,
};
use pipe_trait::*;
use std::{
    collections::HashSet,
    fmt::{self, Display, Formatter},
};

impl<'a> Database<PkgBase<'a>, PkgName<'a>, &'a str> {
    pub fn insert_srcinfo(
        &'a mut self,
        srcinfo: &'a SrcInfo<&'a str>,
    ) -> Result<Option<RemovedInfo>, InsertionError> {
        let pkgbase = srcinfo
            .pkgbase()
            .ok_or(InsertionError::MissingPkgBase)?
            .pipe(PkgBase);

        let removed_srcinfo = self.infos.insert(pkgbase, *srcinfo);

        let dependency_list = if let Some(dependency_list) = self.dependencies.get_mut(&pkgbase) {
            dependency_list
        } else {
            self.dependencies.insert(pkgbase, Default::default());
            self.dependencies.get_mut(&pkgbase).unwrap()
        };

        let mut names_value = HashSet::<PkgName>::new();

        for pkgname in srcinfo.pkgname() {
            let pkgname = PkgName(pkgname);

            names_value.insert(pkgname);
            self.bases.insert(pkgname, pkgbase);

            for dependency in srcinfo.all_required_dependencies() {
                let bases = &self.bases;
                let dependency_pkgbase = dependency
                    .name()
                    .pipe(PkgName)
                    .pipe_ref(|name| bases.get(name));
                if let Some(dependency_pkgbase) = dependency_pkgbase {
                    dependency_list.insert(*dependency_pkgbase);
                    self.build_order
                        .insert(pkgbase.to_string(), dependency_pkgbase.to_string());
                }
            }
        }

        let removed_names = self.names.insert(pkgbase, names_value);

        Ok(match (removed_srcinfo, removed_names) {
            (None, None) => None,
            (Some(srcinfo), Some(names)) => Some(RemovedInfo { srcinfo, names }),
            (srcinfo, ref names) => {
                dbg!(srcinfo, names);
                panic!("impossible state reached");
            }
        })
    }
}

#[derive(Debug, Clone)]
pub struct RemovedInfo<'a> {
    pub srcinfo: SrcInfo<&'a str>,
    pub names: HashSet<PkgName<'a>>,
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
