use super::{
    AurNameWrapper, BorrowedAurName, BorrowedGitUrl, GitUrlWrapper, OwnedAurName, OwnedGitUrl,
    Wrapper,
};
use pipe_trait::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Origin<GitUrl, AurName>
where
    GitUrl: GitUrlWrapper,
    AurName: AurNameWrapper,
{
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

impl OwnedOrigin {
    pub fn new_owned_git(url: impl AsRef<str>) -> Self {
        url.as_ref()
            .to_string()
            .pipe(OwnedGitUrl::from_inner)
            .pipe(Origin::Git)
    }

    pub fn new_owned_aur(name: impl AsRef<str>) -> Self {
        name.as_ref()
            .to_string()
            .pipe(OwnedAurName::from_inner)
            .pipe(Origin::Aur)
    }
}
