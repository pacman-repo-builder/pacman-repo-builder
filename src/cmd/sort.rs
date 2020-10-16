use super::super::{
    args::SortArgs,
    status::{Code, Status},
    utils::{DbInit, DbInitValue},
};

pub fn sort(args: SortArgs) -> Status {
    let SortArgs {} = args;

    let mut db_init = DbInit::default();
    let DbInitValue {
        database,
        mut error_count,
        ..
    } = match db_init.init() {
        Err(error) => return Err(error),
        Ok(value) => value,
    };

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
        Err(Code::GenericFailure)
    }
}
