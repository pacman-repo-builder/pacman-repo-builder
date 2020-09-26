pub mod utils;
pub mod version;

use std::str::Lines;
use utils::extract_value_from_line;
use version::Version;

#[derive(Debug, Copy, Clone)]
pub struct SrcInfo<Text: AsRef<str>>(pub Text);

impl<Text: AsRef<str>> SrcInfo<Text> {
    fn lines(&self) -> Lines {
        self.0.as_ref().lines()
    }

    pub fn pkgname(&self) -> Option<&str> {
        self.lines()
            .find_map(|line| extract_value_from_line("pkgname", line))
    }

    pub fn version(&self) -> Result<Version<&str, &str, &str>, &'static str> {
        let pkgver = self
            .lines()
            .find_map(|line| extract_value_from_line("pkgver", line))
            .ok_or("missing pkgver")?;
        let pkgrel = self
            .lines()
            .find_map(|line| extract_value_from_line("pkgrel", line))
            .ok_or("missing pkgrel")?;
        let epoch = self
            .lines()
            .find_map(|line| extract_value_from_line("epoch", line))
            .unwrap_or("");
        Ok(Version {
            pkgver,
            pkgrel,
            epoch,
        })
    }
}
