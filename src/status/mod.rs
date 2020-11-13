use std::num::NonZeroI32;

mod code;
mod failure;

pub use code::Code;
pub use failure::Failure;

pub type Status = Result<(), Failure>;

pub fn get_code(status: Status) -> i32 {
    match status {
        Ok(()) => 0,
        Err(failure) => failure.code(),
    }
}

pub fn status_of_code(code: i32) -> Status {
    if let Some(code) = NonZeroI32::new(code) {
        Failure::Code(code).into()
    } else {
        Ok(())
    }
}
