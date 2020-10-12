use std::process::Command;

pub fn create_makepkg_command() -> Command {
    let mut command = Command::new("makepkg");
    command
        .env("PKGEXT", ".pkg.tar.zst")
        .env("SRCEXT", ".src.tar.gz");
    command
}
