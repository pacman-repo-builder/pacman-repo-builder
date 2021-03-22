pub mod database;
pub mod dependency;
pub mod version;

use super::utils::{extract_value_from_line, PackageFileName};
use dependency::{ReasonedDependency, UnreasonedDependency};
use itertools::Itertools;
use pipe_trait::*;
use std::str::Lines;
use version::Version;

#[derive(Debug, Copy, Clone)]
pub struct SrcInfo<Text: AsRef<str>>(pub Text);

macro_rules! line_extractor {
    ($prefix:expr) => {
        move |line| extract_value_from_line($prefix, line)
    };
}

impl<Text: AsRef<str>> SrcInfo<Text> {
    fn lines(&self) -> Lines {
        self.0.as_ref().lines()
    }

    pub fn pkgbase(&self) -> Option<&str> {
        self.lines().find_map(line_extractor!("pkgbase"))
    }

    pub fn pkgname(&self) -> impl Iterator<Item = &str> {
        self.lines().filter_map(line_extractor!("pkgname"))
    }

    pub fn arch(&self) -> impl Iterator<Item = &str> {
        self.lines().filter_map(line_extractor!("arch"))
    }

    pub fn version(&self) -> Result<Version<&str, &str, &str>, &'static str> {
        let pkgver = self
            .lines()
            .find_map(line_extractor!("pkgver"))
            .ok_or("missing pkgver")?;
        let pkgrel = self
            .lines()
            .find_map(line_extractor!("pkgrel"))
            .ok_or("missing pkgrel")?;
        let epoch = self
            .lines()
            .find_map(line_extractor!("epoch"))
            .unwrap_or("");
        Ok(Version {
            pkgver,
            pkgrel,
            epoch,
        })
    }

    fn get_dependencies(
        &self,
        key: &'static str,
    ) -> impl Iterator<Item = UnreasonedDependency<&str, &str>> {
        self.lines()
            .filter_map(line_extractor!(key))
            .map(UnreasonedDependency::new)
    }

    pub fn depends(&self) -> impl Iterator<Item = UnreasonedDependency<&str, &str>> {
        self.get_dependencies("depends")
    }

    pub fn makedepends(&self) -> impl Iterator<Item = UnreasonedDependency<&str, &str>> {
        self.get_dependencies("makedepends")
    }

    pub fn checkdepends(&self) -> impl Iterator<Item = UnreasonedDependency<&str, &str>> {
        self.get_dependencies("checkdepends")
    }

    pub fn optdepends(&self) -> impl Iterator<Item = ReasonedDependency<&str, &str, &str>> {
        self.lines()
            .filter_map(line_extractor!("optdepends"))
            .map(ReasonedDependency::new)
    }

    pub fn conflicts(&self) -> impl Iterator<Item = UnreasonedDependency<&str, &str>> {
        self.get_dependencies("conflicts")
    }

    pub fn all_required_dependencies(
        &self,
    ) -> impl Iterator<Item = UnreasonedDependency<&str, &str>> {
        let depends = self.depends();
        let makedepends = self.makedepends();
        let checkdepends = self.checkdepends();
        depends.chain(makedepends).chain(checkdepends)
    }

    pub fn package_file_base_names(
        &self,
        filter_arch: impl Fn(&&str) -> bool,
    ) -> Result<impl Iterator<Item = PackageFileName<&str, String, &str>> + '_, String> {
        let version = self.version().map_err(String::from)?.try_to_string()?;

        self.pkgname()
            .cartesian_product(self.arch().filter(filter_arch).collect::<Vec<_>>())
            .map(move |(pkgname, arch)| PackageFileName {
                pkgname,
                arch,
                version: version.clone(),
            })
            .pipe(Ok)
    }
}
