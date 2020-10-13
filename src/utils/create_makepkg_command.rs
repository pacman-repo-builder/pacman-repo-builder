use std::process::Command;

pub fn create_makepkg_command() -> Command {
    let mut command = Command::new("makepkg");
    command
        .env_remove("PACMAN")
        .env_remove("MAKEPKG_CONF")
        .env_remove("PKGDEST")
        .env_remove("SRCDEST")
        .env_remove("LOGDEST")
        .env_remove("PACKAGER")
        .env_remove("SRCPKGDEST")
        .env_remove("BUILDDIR")
        .env_remove("GNUPGHOME")
        .env_remove("GPGKEY")
        .env_remove("SOURCE_DATE_EPOCH")
        .env("PKGEXT", ".pkg.tar.zst")
        .env("SRCEXT", ".src.tar.gz");
    command
}
