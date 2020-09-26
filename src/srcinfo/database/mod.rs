pub mod insert_srcinfo;
pub mod package_list;
pub mod text_wrapper;

use super::SrcInfo;
use package_list::PackageList;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

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
