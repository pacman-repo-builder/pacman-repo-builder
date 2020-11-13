use super::{Code, Status};
use std::io::Error;

#[derive(Debug)]
pub enum Failure {
    Os(Error),
    Code(Code),
}

impl Failure {
    pub fn code(&self) -> i32 {
        match self {
            Failure::Os(error) => error.raw_os_error().unwrap_or(1),
            Failure::Code(code) => *code as i32,
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
        Failure::Code(code)
    }
}

impl Into<Status> for Failure {
    fn into(self) -> Status {
        Err(self)
    }
}
