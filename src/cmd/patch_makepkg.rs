use super::super::{args::PatchMakePkgArgs, utils::CUSTOM_MAKEPKG};
use std::fs::write;

pub fn patch_makepkg(args: PatchMakePkgArgs) -> i32 {
    let PatchMakePkgArgs { replace } = args;

    if replace {
        if let Err(error) = write("/usr/bin/makepkg", CUSTOM_MAKEPKG) {
            eprintln!("{}", error);
            return error.raw_os_error().unwrap_or(1);
        }
    } else {
        print!("{}", CUSTOM_MAKEPKG);
        eprintln!();
        eprintln!("# NOTE: Above is the content of custom makepkg script");
        eprintln!("# NOTE: Run again with --replace flag to replace system's makepkg");
    }

    0
}
