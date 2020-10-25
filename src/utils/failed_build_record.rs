use super::PackageFileName;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, time::SystemTime};

pub type FailedBuildRecord<PkgName, Version, Arch> =
    Vec<FailedBuildRecordItem<PkgName, Version, Arch>>;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct FailedBuildRecordItem<PkgName, Version, Arch>
where
    PkgName: Display,
    Version: Display,
    Arch: Display,
{
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<SystemTime>,
    #[serde(flatten)]
    pub package_file_name: PackageFileName<PkgName, Version, Arch>,
}
