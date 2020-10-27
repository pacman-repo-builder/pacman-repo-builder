use super::{
    AurNameWrapper, BorrowedAurName, BorrowedGitUrl, GitUrlWrapper, OwnedAurName, OwnedGitUrl,
    Wrapper,
};
use pipe_trait::*;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, SmartDefault, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(
    from = "OriginSerdeHelper<GitUrl, AurName>",
    into = "OriginSerdeHelper<GitUrl, AurName>"
)]
pub enum Origin<GitUrl, AurName>
where
    GitUrl: GitUrlWrapper,
    AurName: AurNameWrapper,
{
    #[default]
    Local,
    Git(GitUrl),
    Aur(AurName),
}

pub type OwnedOrigin = Origin<OwnedGitUrl, OwnedAurName>;
pub type BorrowedOrigin<'a> = Origin<BorrowedGitUrl<'a>, BorrowedAurName<'a>>;

impl<GitUrl, AurName> Origin<GitUrl, AurName>
where
    GitUrl: GitUrlWrapper,
    AurName: AurNameWrapper,
{
    pub fn as_borrowed(&self) -> BorrowedOrigin<'_> {
        match self {
            Origin::Local => Origin::Local,
            Origin::Git(url) => url
                .as_ref()
                .pipe(BorrowedGitUrl::from_inner)
                .pipe(Origin::Git),
            Origin::Aur(name) => name
                .as_ref()
                .pipe(BorrowedAurName::from_inner)
                .pipe(Origin::Aur),
        }
    }

    pub fn to_owned(&self) -> OwnedOrigin {
        match self {
            Origin::Local => Origin::Local,
            Origin::Git(url) => url
                .as_ref()
                .to_string()
                .pipe(OwnedGitUrl::from_inner)
                .pipe(Origin::Git),
            Origin::Aur(name) => name
                .as_ref()
                .to_string()
                .pipe(OwnedAurName::from_inner)
                .pipe(Origin::Aur),
        }
    }
}

/* SERDE HELPER */

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(tag = "origin", rename_all = "kebab-case")]
enum OriginSerdeHelper<GitUrl, AurName> {
    Local,
    Git { url: GitUrl },
    Aur { name: AurName },
}

impl<GitUrl, AurName> From<OriginSerdeHelper<GitUrl, AurName>> for Origin<GitUrl, AurName>
where
    GitUrl: GitUrlWrapper,
    AurName: AurNameWrapper,
{
    fn from(source: OriginSerdeHelper<GitUrl, AurName>) -> Self {
        match source {
            OriginSerdeHelper::Local => Origin::Local,
            OriginSerdeHelper::Git { url } => Origin::Git(url),
            OriginSerdeHelper::Aur { name } => Origin::Aur(name),
        }
    }
}

impl<GitUrl, AurName> From<Origin<GitUrl, AurName>> for OriginSerdeHelper<GitUrl, AurName>
where
    GitUrl: GitUrlWrapper,
    AurName: AurNameWrapper,
{
    fn from(source: Origin<GitUrl, AurName>) -> Self {
        match source {
            Origin::Local => OriginSerdeHelper::Local,
            Origin::Git(url) => OriginSerdeHelper::Git { url },
            Origin::Aur(name) => OriginSerdeHelper::Aur { name },
        }
    }
}
