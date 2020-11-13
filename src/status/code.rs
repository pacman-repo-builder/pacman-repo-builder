use super::{Failure, Status};
use pipe_trait::*;
use std::num::NonZeroI32;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Code {
    GenericFailure = 1,
    ManifestLoadingFailure = 2,
    SrcInfoOutOfSync = 3,
    CyclicDependency = 4,
    UnrecognizedMakePkg = 5,
    FailedBuildRecordLoadingFailure = 6,
    FailedBuildRecordWritingFailure = 7,
}

impl Into<Status> for Code {
    fn into(self) -> Status {
        self.pipe(Into::<Failure>::into).into()
    }
}

impl Into<NonZeroI32> for Code {
    fn into(self) -> NonZeroI32 {
        unsafe { NonZeroI32::new_unchecked(self as i32) }
    }
}
