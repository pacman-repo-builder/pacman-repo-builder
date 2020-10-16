use super::{Failure, Status};
use pipe_trait::*;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Code {
    GenericFailure = 1,
    ManifestLoadingFailure = 2,
    SrcInfoOutOfSync = 3,
    CyclicDependency = 4,
}

impl Into<Status> for Code {
    fn into(self) -> Status {
        self.pipe(Into::<Failure>::into).into()
    }
}
