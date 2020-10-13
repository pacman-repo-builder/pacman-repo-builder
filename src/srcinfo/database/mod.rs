pub mod build_order;
pub mod insert_srcinfo;
pub mod package_file_base_names;
pub mod text_wrapper;

use super::SrcInfo;
use indexmap::{IndexMap, IndexSet};
use smart_default::SmartDefault;
use std::{hash::Hash, path::Path};
use text_wrapper::{PkgBase, PkgName};

#[derive(Debug, SmartDefault)]
pub struct Database<PkgBase, PkgName, SrcInfoContent, BuildDir>
where
    PkgBase: AsRef<str> + Hash + Eq + Clone,
    PkgName: AsRef<str> + Hash + Eq + Clone,
    SrcInfoContent: AsRef<str>,
    BuildDir: AsRef<Path>,
{
    pkgbase: IndexMap<PkgBase, DatabaseValue<PkgBase, PkgName, SrcInfoContent, BuildDir>>,
    pkgname: IndexMap<PkgName, PkgBase>,
}

impl<PkgBase, PkgName, SrcInfoContent, BuildDir>
    Database<PkgBase, PkgName, SrcInfoContent, BuildDir>
where
    PkgBase: AsRef<str> + Hash + Eq + Clone,
    PkgName: AsRef<str> + Hash + Eq + Clone,
    SrcInfoContent: AsRef<str>,
    BuildDir: AsRef<Path>,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn pkgbase(
        &self,
    ) -> &IndexMap<PkgBase, DatabaseValue<PkgBase, PkgName, SrcInfoContent, BuildDir>> {
        &self.pkgbase
    }
}

#[derive(Debug)]
pub struct DatabaseValue<PkgBase, PkgName, SrcInfoContent, BuildDir>
where
    PkgBase: AsRef<str> + Hash + Eq + Clone,
    PkgName: AsRef<str> + Hash + Eq + Clone,
    SrcInfoContent: AsRef<str>,
    BuildDir: AsRef<Path>,
{
    pub names: IndexSet<PkgName>,
    pub dependencies: IndexSet<PkgBase>,
    pub srcinfo: SrcInfo<SrcInfoContent>,
    pub directory: BuildDir,
}

pub type SimpleDatabase<'a> = Database<PkgBase<'a>, PkgName<'a>, &'a str, &'a Path>;
pub type SimpleDatabaseValue<'a> = DatabaseValue<PkgBase<'a>, PkgName<'a>, &'a str, &'a Path>;
