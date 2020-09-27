use std::str::FromStr;

#[derive(Debug)]
pub enum SrcInfo {
    SrcInfo,
    PkgBuild,
    Either,
}

impl FromStr for SrcInfo {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Ok(match text {
            "srcinfo" => SrcInfo::SrcInfo,
            "pkgbuild" => SrcInfo::PkgBuild,
            "either" => SrcInfo::Either,
            _ => return Err(format!("{:?} is not a valid choice", text)),
        })
    }
}
