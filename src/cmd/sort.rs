use super::super::{
    args::SortArgs,
    utils::{DbInit, DbInitValue},
};

pub fn sort(args: SortArgs) -> i32 {
    let SortArgs {} = args;

    let mut srcinfo_texts = Default::default();
    let mut srcinfo_collection = Default::default();
    let DbInitValue {
        database,
        error_count,
        ..
    } = match (DbInit {
        srcinfo_texts: &mut srcinfo_texts,
        srcinfo_collection: &mut srcinfo_collection,
    })
    .init()
    {
        Err(error) => return error.code(),
        Ok(value) => value,
    };

    for pkgbase in database.into_build_order().0 {
        println!("{}", pkgbase);
    }

    if error_count == 0 {
        0
    } else {
        eprintln!("{} errors occurred", error_count);
        1
    }
}
