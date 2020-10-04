use super::super::{
    args::SortArgs,
    manifest::Manifest,
    srcinfo::{database::SimpleDatabase, SrcInfo},
    utils::read_srcinfo_texts,
};

pub fn sort(args: SortArgs) -> i32 {
    let mut error_count = 0u32;

    let SortArgs {} = args;
    let manifest = Manifest::from_env().unwrap_or_default();

    let srcinfo_texts = read_srcinfo_texts(&manifest, |error| {
        eprintln!("{}", error);
        error_count += 1;
    });

    let srcinfo_collection: Vec<_> = srcinfo_texts
        .iter()
        .map(|x| x.to_ref().map(String::as_str).map(SrcInfo))
        .collect();
    let mut database = SimpleDatabase::default();
    for pair in &srcinfo_collection {
        let (srcinfo, directory) = pair.to_ref().into_tuple();
        if let Err(error) = database.insert_srcinfo(srcinfo) {
            eprintln!("error in directory {:?}: {}", directory, error);
            error_count += 1;
        }
    }

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
