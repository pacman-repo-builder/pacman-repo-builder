use super::{Code, Status};
use std::{io::Error, num::NonZeroI32};

#[derive(Debug)]
pub enum Failure {
    Os(Error),
    Code(NonZeroI32),
}

impl Failure {
    pub fn code(&self) -> i32 {
        match self {
            Failure::Os(error) => error.raw_os_error().unwrap_or(1),
            Failure::Code(code) => code.get(),
        }
    }
}

impl From<Error> for Failure {
    fn from(error: Error) -> Self {
        Failure::Os(error)
    }
}

impl From<Code> for Failure {
    fn from(code: Code) -> Self {
        Failure::Code(code.into())
    }
}

impl From<Failure> for Status {
    fn from(failure: Failure) -> Self {
        Err(failure)
    }
}
