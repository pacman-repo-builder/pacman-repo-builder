pub mod build_order;
pub mod insert_srcinfo;
pub mod package_file_base_names;
pub mod text_wrapper;

use super::SrcInfo;
use indexmap::{IndexMap, IndexSet};
use std::hash::Hash;
use text_wrapper::{PkgBase, PkgName};

#[derive(Debug, Default)]
pub struct Database<PkgBase, PkgName, SrcInfoContent>
where
    PkgBase: AsRef<str> + Hash + Eq + Clone,
    PkgName: AsRef<str> + Hash + Eq + Clone,
    SrcInfoContent: AsRef<str>,
{
    base_to_name: IndexMap<PkgBase, IndexSet<PkgName>>,
    name_to_base: IndexMap<PkgName, PkgBase>,
    infos: IndexMap<PkgBase, SrcInfo<SrcInfoContent>>,
    dependencies: IndexMap<PkgBase, IndexSet<PkgBase>>,
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

    pub fn base_to_name(&self) -> &IndexMap<PkgBase, IndexSet<PkgName>> {
        &self.base_to_name
    }

    pub fn name_to_base(&self) -> &IndexMap<PkgName, PkgBase> {
        &self.name_to_base
    }

    pub fn infos(&self) -> &IndexMap<PkgBase, SrcInfo<SrcInfoContent>> {
        &self.infos
    }

    pub fn dependencies(&self) -> &IndexMap<PkgBase, IndexSet<PkgBase>> {
        &self.dependencies
    }
}

pub type SimpleDatabase<'a> = Database<PkgBase<'a>, PkgName<'a>, &'a str>;
