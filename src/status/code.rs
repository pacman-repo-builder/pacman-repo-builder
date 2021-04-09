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

impl From<Code> for Status {
    fn from(code: Code) -> Self {
        code.pipe(Failure::from).into()
    }
}

impl From<Code> for NonZeroI32 {
    fn from(code: Code) -> Self {
        unsafe { NonZeroI32::new_unchecked(code as i32) }
    }
}
