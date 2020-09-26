pub mod package_list;

use super::{dependency::unreasoned::UnreasonedDependency as Dependency, SrcInfo};
use package_list::PackageList;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Default)]
pub struct List<PkgName, Range, SrcInfoContent>
where
    PkgName: AsRef<str> + Hash + Eq + Clone,
    Range: AsRef<str>,
    SrcInfoContent: AsRef<str>,
{
    packages: HashMap<PkgName, SrcInfo<SrcInfoContent>>,
    dependencies: HashMap<PkgName, HashSet<Dependency<PkgName, Range>>>,
    list: PackageList,
}

impl<PkgName, Range, SrcInfoContent> List<PkgName, Range, SrcInfoContent>
where
    PkgName: AsRef<str> + Default + Hash + Eq + Clone,
    Range: AsRef<str> + Default,
    SrcInfoContent: AsRef<str> + Default,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn packages(&self) -> &HashMap<PkgName, SrcInfo<SrcInfoContent>> {
        &self.packages
    }

    pub fn dependencies(&self) -> &HashMap<PkgName, HashSet<Dependency<PkgName, Range>>> {
        &self.dependencies
    }

    pub fn list(&self) -> &PackageList {
        &self.list
    }
}

impl<'a> List<&'a str, &'a str, &'a str> {
    pub fn insert_srcinfo(
        &'a mut self,
        srcinfo: &'a SrcInfo<&'a str>,
    ) -> Result<Option<SrcInfo<&'a str>>, String> {
        let pkgname = srcinfo
            .pkgname()
            .ok_or_else(|| "missing pkgname".to_string())?;

        let removed = self.packages.insert(&pkgname, *srcinfo);

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
