use command_extra::CommandExtra;
use std::process::Command;

pub fn create_makepkg_command() -> Command {
    Command::new("makepkg")
        .without_env("PACMAN")
        .without_env("MAKEPKG_CONF")
        .without_env("PKGDEST")
        .without_env("SRCDEST")
        .without_env("LOGDEST")
        .without_env("PACKAGER")
        .without_env("SRCPKGDEST")
        .without_env("BUILDDIR")
        .without_env("GNUPGHOME")
        .without_env("GPGKEY")
        .without_env("SOURCE_DATE_EPOCH")
        .with_env("PKGEXT", ".pkg.tar.zst")
        .with_env("SRCEXT", ".src.tar.gz")
}
