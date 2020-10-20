use super::super::{args::CopyrightArgs, status::Status, utils::COPYING};

pub fn copyright(args: CopyrightArgs) -> Status {
    let CopyrightArgs {} = args;
    print!("{}", COPYING);
    Ok(0)
}
