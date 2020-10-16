use super::super::{
    args::PatchMakePkgArgs,
    status::{Code, Status},
    utils::{CUSTOM_MAKEPKG, CUSTOM_MAKEPKG_SHA1SUM, ORIGINAL_MAKEPKG_SHA1SUM},
};
use hex_fmt::HexFmt;
use pipe_trait::*;
use sha1::{Digest, Sha1};
use std::fs::{read, write};

pub fn patch_makepkg(args: PatchMakePkgArgs) -> Status {
    let PatchMakePkgArgs {
        replace,
        unsafe_ignore_unknown_changes,
    } = args;

    if !unsafe_ignore_unknown_changes {
        let mut hasher = Sha1::new();
        let makepkg = match read("/usr/bin/makepkg") {
            Ok(content) => content,
            Err(error) => {
                eprintln!("⮾ {}", error);
                return error.raw_os_error().unwrap_or(1).pipe(Ok);
            }
        };
        hasher.update(&makepkg);
        let hash = hasher.finalize();
        let hash = hash.as_slice();
        if hash != ORIGINAL_MAKEPKG_SHA1SUM && hash != CUSTOM_MAKEPKG_SHA1SUM {
            eprintln!(
                "🛈 sha1sum of expected default system makepkg: {}",
                HexFmt(ORIGINAL_MAKEPKG_SHA1SUM),
            );
            eprintln!(
                "🛈 sha1sum of custom makepkg: {}",
                HexFmt(CUSTOM_MAKEPKG_SHA1SUM),
            );
            eprintln!("🛈 sha1sum of actual system makepkg: {}", HexFmt(hash));
            eprintln!("⮾ makepkg had been modified by an unknown party");
            eprintln!("⮾ it is not safe to proceed");
            eprintln!("🛈 run again with --unsafe-ignore-unknown-changes to ignore this error");
            return Err(Code::GenericFailure);
        }
    }

    if replace {
        if let Err(error) = write("/usr/bin/makepkg", CUSTOM_MAKEPKG) {
            eprintln!("⮾ {}", error);
            return error.raw_os_error().unwrap_or(1).pipe(Ok);
        }
    } else {
        print!("{}", CUSTOM_MAKEPKG);
        eprintln!();
        eprintln!("# NOTE: Above is the content of custom makepkg script");
        eprintln!("# NOTE: Run again with --replace flag to replace system's makepkg");
    }

    Ok(0)
}
