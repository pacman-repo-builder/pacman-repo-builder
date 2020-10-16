use super::super::{
    args::SortArgs,
    status::{Code, Failure, Status},
    utils::{DbInit, DbInitValue},
};
use pipe_trait::*;

pub fn sort(args: SortArgs) -> Status {
    let SortArgs {} = args;

    let mut db_init = DbInit::default();
    let DbInitValue {
        database,
        mut error_count,
        ..
    } = db_init.init()?;

    match database.build_order() {
        Err(error) => {
            eprintln!("⮾ {}", error);
            error_count += 1;
        }
        Ok(build_order) => {
            for pkgbase in build_order {
                println!("{}", pkgbase)
            }
        }
    }

    if error_count == 0 {
        Ok(0)
    } else {
        eprintln!("{} errors occurred", error_count);
        Code::GenericFailure.pipe(Failure::Expected).pipe(Err)
    }
}
