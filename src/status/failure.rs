use super::Code;
use std::io::Error;

#[derive(Debug)]
pub enum Failure {
    Unexpected(Error),
    Expected(Code),
}

impl Failure {
    pub fn code(&self) -> i32 {
        match self {
            Failure::Unexpected(error) => error.raw_os_error().unwrap_or(1),
            Failure::Expected(code) => *code as i32,
        }
    }
}
