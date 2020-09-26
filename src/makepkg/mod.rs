use std::ffi::OsString;

#[derive(Debug, Default)]
pub struct MakePkg {
    pub pacman: Option<OsString>,
    pub config: Option<OsString>,
    pub pkgbuild: Option<OsString>,
    pub syncdeps: bool,
    pub asdeps: bool,
    pub packager: Option<OsString>,
}
