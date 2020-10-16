mod code;

pub use code::Code;

pub type Status = Result<i32, Code>;

pub fn get_code(status: Status) -> i32 {
    match status {
        Ok(code) => code,
        Err(code) => code as i32,
    }
}
