use super::{
    Associations, BorrowedDirectory, BuildMetadata, GlobalSettings, OwnedDirectory, Wrapper,
};
use pipe_trait::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Member<Directory>
where
    Directory: Associations + AsRef<Path>,
{
    pub directory: Directory,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_build_metadata: Option<BuildMetadata>,
}

pub type OwnedMember = Member<OwnedDirectory>;
pub type BorrowedMember<'a> = Member<BorrowedDirectory<'a>>;

impl<Directory> Member<Directory>
where
    Directory: Associations + AsRef<Path>,
{
    pub fn as_path(&self) -> BorrowedMember<'_> {
        Member {
            directory: self.directory.as_ref().pipe(Wrapper::from_inner),
            read_build_metadata: self.read_build_metadata,
        }
    }

    pub fn to_path_buf(&self) -> OwnedMember {
        Member {
            directory: self
                .directory
                .as_ref()
                .to_path_buf()
                .pipe(Wrapper::from_inner),
            read_build_metadata: self.read_build_metadata,
        }
    }

    pub fn resolve<P: Associations + AsRef<Path>, Q: Associations + AsRef<Path>>(
        &self,
        global_settings: &GlobalSettings<P, Q>,
    ) -> OwnedMember {
        Member {
            directory: Wrapper::from_inner(if let Some(container) = &global_settings.container {
                container.as_ref().join(self.directory.as_ref())
            } else {
                self.directory.as_ref().to_path_buf()
            }),

            read_build_metadata: self
                .read_build_metadata
                .or(global_settings.read_build_metadata),
        }
    }
}
