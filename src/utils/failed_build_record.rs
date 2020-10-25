use super::PackageFileName;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

pub type FailedBuildRecord<PkgName, Version, Arch> =
    Vec<FailedBuildRecordItem<PkgName, Version, Arch>>;

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct FailedBuildRecordItem<PkgName, Version, Arch> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<SystemTime>,
    #[serde(flatten)]
    pub package_file_name: PackageFileName<PkgName, Version, Arch>,
}

impl<PkgName, Version, Arch> FailedBuildRecordItem<PkgName, Version, Arch> {
    pub fn with_date(self, date: SystemTime) -> Self {
        FailedBuildRecordItem {
            date: Some(date),
            ..self
        }
    }

    pub fn without_date(self) -> Self {
        FailedBuildRecordItem { date: None, ..self }
    }
}

impl<PkgName, Version, Arch> From<PackageFileName<PkgName, Version, Arch>>
    for FailedBuildRecordItem<PkgName, Version, Arch>
{
    fn from(package_file_name: PackageFileName<PkgName, Version, Arch>) -> Self {
        FailedBuildRecordItem {
            date: None,
            package_file_name,
        }
    }
}

impl<PkgName, Version, Arch> From<FailedBuildRecordItem<PkgName, Version, Arch>>
    for PackageFileName<PkgName, Version, Arch>
{
    fn from(failed_build_record_item: FailedBuildRecordItem<PkgName, Version, Arch>) -> Self {
        failed_build_record_item.package_file_name
    }
}
