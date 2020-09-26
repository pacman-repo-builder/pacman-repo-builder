pub mod package_list;

use super::{dependency::unreasoned::UnreasonedDependency as Dependency, SrcInfo};
use package_list::PackageList;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Default)]
pub struct Database<PkgName, Range, SrcInfoContent>
where
    PkgName: AsRef<str> + Hash + Eq + Clone,
    Range: AsRef<str>,
    SrcInfoContent: AsRef<str>,
{
    infos: HashMap<PkgName, SrcInfo<SrcInfoContent>>,
    dependencies: HashMap<PkgName, HashSet<Dependency<PkgName, Range>>>,
    list: PackageList,
}

impl<PkgName, Range, SrcInfoContent> Database<PkgName, Range, SrcInfoContent>
where
    PkgName: AsRef<str> + Default + Hash + Eq + Clone,
    Range: AsRef<str> + Default,
    SrcInfoContent: AsRef<str> + Default,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn infos(&self) -> &HashMap<PkgName, SrcInfo<SrcInfoContent>> {
        &self.infos
    }

    pub fn dependencies(&self) -> &HashMap<PkgName, HashSet<Dependency<PkgName, Range>>> {
        &self.dependencies
    }

    pub fn list(&self) -> &PackageList {
        &self.list
    }
}

impl<'a> Database<&'a str, &'a str, &'a str> {
    pub fn insert_srcinfo(
        &'a mut self,
        srcinfo: &'a SrcInfo<&'a str>,
    ) -> Result<Option<SrcInfo<&'a str>>, String> {
        let pkgname = srcinfo
            .pkgname()
            .ok_or_else(|| "missing pkgname".to_string())?;

        let removed = self.infos.insert(&pkgname, *srcinfo);

        let dependency_list = if let Some(dependency_list) = self.dependencies.get_mut(&pkgname) {
            dependency_list
        } else {
            self.dependencies.insert(&pkgname, Default::default());
            self.dependencies.get_mut(&pkgname).unwrap()
        };

        for dependency in srcinfo.all_required_dependencies() {
            dependency_list.insert(dependency);
            self.list
                .insert(pkgname.to_string(), dependency.name().to_string());
        }

        Ok(removed)
    }
}
