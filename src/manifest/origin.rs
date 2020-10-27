use super::{
    AurNameWrapper, BorrowedAurName, BorrowedGitUrl, GitUrlWrapper, OwnedAurName, OwnedGitUrl,
    Wrapper,
};
use pipe_trait::*;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, SmartDefault, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(tag = "origin", rename_all = "kebab-case")]
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
