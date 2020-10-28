use super::{
    AurNameWrapper, BorrowedAurName, BorrowedGitUrl, GitUrlWrapper, OwnedAurName, OwnedGitUrl,
    Wrapper,
};
use pipe_trait::*;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, SmartDefault, Serialize, Deserialize, Eq, PartialEq, Copy, Clone)]
#[serde(
    from = "SerdeHelper<GitUrl, AurName>",
    into = "SerdeHelper<GitUrl, AurName>"
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

/* SERDE HELPER */

#[derive(Serialize, Deserialize, Copy, Clone)]
#[serde(tag = "origin", rename_all = "kebab-case")]
enum SerdeHelper<GitUrl, AurName> {
    Local,
    Git {
        #[serde(rename = "git-url")]
        url: GitUrl,
    },
    Aur {
        #[serde(rename = "aur-name")]
        name: AurName,
    },
}

impl<GitUrl, AurName> From<SerdeHelper<GitUrl, AurName>> for Origin<GitUrl, AurName>
where
    GitUrl: GitUrlWrapper,
    AurName: AurNameWrapper,
{
    fn from(source: SerdeHelper<GitUrl, AurName>) -> Self {
        match source {
            SerdeHelper::Local => Origin::Local,
            SerdeHelper::Git { url } => Origin::Git(url),
            SerdeHelper::Aur { name } => Origin::Aur(name),
        }
    }
}

impl<GitUrl, AurName> From<Origin<GitUrl, AurName>> for SerdeHelper<GitUrl, AurName>
where
    GitUrl: GitUrlWrapper,
    AurName: AurNameWrapper,
{
    fn from(source: Origin<GitUrl, AurName>) -> Self {
        match source {
            Origin::Local => SerdeHelper::Local,
            Origin::Git(url) => SerdeHelper::Git { url },
            Origin::Aur(name) => SerdeHelper::Aur { name },
        }
    }
}
