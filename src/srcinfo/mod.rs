pub mod dependency;
pub mod optional_dependency;
pub mod utils;
pub mod version;

use dependency::Dependency;
use optional_dependency::OptionalDependency;
use std::str::Lines;
use utils::extract_value_from_line;
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

    pub fn pkgname(&self) -> Option<&str> {
        self.lines().find_map(line_extractor!("pkgname"))
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

    fn get_dependencies(&self, key: &'static str) -> impl Iterator<Item = Dependency<&str>> {
        self.lines()
            .filter_map(line_extractor!(key))
            .map(Dependency)
    }

    pub fn depends(&self) -> impl Iterator<Item = Dependency<&str>> {
        self.get_dependencies("depends")
    }

    pub fn makedepends(&self) -> impl Iterator<Item = Dependency<&str>> {
        self.get_dependencies("makedepends")
    }

    pub fn optdepends(&self) -> impl Iterator<Item = OptionalDependency<&str, &str>> {
        self.lines()
            .filter_map(line_extractor!("optdepends"))
            .map(OptionalDependency::new)
    }

    pub fn all_required_dependencies(&self) -> impl Iterator<Item = Dependency<&str>> {
        self.depends().chain(self.makedepends())
    }
}
