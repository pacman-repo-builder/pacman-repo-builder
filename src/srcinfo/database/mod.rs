pub mod package_list;
pub mod text_wrapper;

use super::SrcInfo;
use package_list::PackageList;
use pipe_trait::*;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};
use text_wrapper::{PkgBase, PkgName};

#[derive(Debug, Default)]
pub struct Database<PkgBase, PkgName, SrcInfoContent>
where
    PkgBase: AsRef<str> + Hash + Eq + Clone,
    PkgName: AsRef<str> + Hash + Eq + Clone,
    SrcInfoContent: AsRef<str>,
{
    names: HashMap<PkgBase, HashSet<PkgName>>,
    bases: HashMap<PkgName, PkgBase>,
    infos: HashMap<PkgBase, SrcInfo<SrcInfoContent>>,
    dependencies: HashMap<PkgBase, HashSet<PkgBase>>,
    list: PackageList,
}

impl<PkgBase, PkgName, SrcInfoContent> Database<PkgBase, PkgName, SrcInfoContent>
where
    PkgBase: AsRef<str> + Default + Hash + Eq + Clone,
    PkgName: AsRef<str> + Default + Hash + Eq + Clone,
    SrcInfoContent: AsRef<str> + Default,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn infos(&self) -> &HashMap<PkgBase, SrcInfo<SrcInfoContent>> {
        &self.infos
    }

    pub fn dependencies(&self) -> &HashMap<PkgBase, HashSet<PkgBase>> {
        &self.dependencies
    }

    pub fn list(&self) -> &PackageList {
        &self.list
    }
}

impl<'a> Database<PkgBase<'a>, PkgName<'a>, &'a str> {
    pub fn insert_srcinfo(
        &'a mut self,
        srcinfo: &'a SrcInfo<&'a str>,
    ) -> Result<Option<SrcInfo<&'a str>>, String> {
        let pkgbase = srcinfo
            .pkgbase()
            .ok_or_else(|| "missing pkgbase".to_string())?
            .pipe(PkgBase);

        let removed = self.infos.insert(pkgbase, *srcinfo);

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
                    self.list
                        .insert(pkgbase.to_string(), dependency_pkgbase.to_string());
                }
            }
        }

        self.names.insert(pkgbase, names_value);

        Ok(removed)
    }
}
