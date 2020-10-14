use super::super::{
    args::SortArgs,
    utils::{DbInit, DbInitValue},
};

pub fn sort(args: SortArgs) -> i32 {
    let SortArgs {} = args;

    let mut db_init = DbInit::default();
    let DbInitValue {
        database,
        mut error_count,
        ..
    } = match db_init.init() {
        Err(error) => return error.code(),
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
        0
    } else {
        eprintln!("{} errors occurred", error_count);
        1
    }
}
