mod code;
mod failure;

pub use code::Code;
pub use failure::Failure;

pub type Status = Result<i32, Failure>;

pub fn get_code(status: Status) -> i32 {
    match status {
        Ok(code) => code,
        Err(failure) => failure.code(),
    }
}
