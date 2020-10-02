use super::{concat_repository_options, BuildMetadata, GlobalSettings, Repository};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub struct Member<P: AsRef<Path>> {
    pub directory: P,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_build_metadata: Option<BuildMetadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repository: Option<Repository<P>>,
}

impl<P: AsRef<Path>> Member<P> {
    pub fn as_path(&self) -> Member<&Path> {
        Member {
            directory: self.directory.as_ref(),
            read_build_metadata: self.read_build_metadata,
            repository: self.repository.as_ref().map(|x| x.as_path()),
        }
    }

    pub fn resolve<Q: AsRef<Path>>(&self, global_settings: &GlobalSettings<Q>) -> Member<PathBuf> {
        Member {
            directory: if let Some(container) = &global_settings.container {
                container.as_ref().join(self.directory.as_ref())
            } else {
                self.directory.as_ref().to_path_buf()
            },

            read_build_metadata: self
                .read_build_metadata
                .or(global_settings.read_build_metadata),

            repository: concat_repository_options(
                self.repository.as_ref().map(Repository::to_path_buf),
                global_settings
                    .repository
                    .as_ref()
                    .map(Repository::to_path_buf),
            ),
        }
    }
}
