use super::super::{
    args::{Args, Command},
    cmd::{build, copyright, deref_db, outdated, patch_makepkg, print_config, sort, sync_srcinfo},
    status::Status,
};
use super::App;

impl App {
    pub fn run(self) -> Status {
        let Args { command } = self.args;
        match command {
            Command::PrintConfig(args) => print_config(args),
            Command::Sort(args) => sort(args),
            Command::Outdated(args) => outdated(args),
            Command::SyncSrcInfo(args) => sync_srcinfo(args),
            Command::PatchMakePkg(args) => patch_makepkg(args),
            Command::DerefDb(args) => deref_db(args),
            Command::Build(args) => build(args),
            Command::Copyright(args) => copyright(args),
        }
    }
}
