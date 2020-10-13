use super::super::{
    args::{Args, Command},
    cmd::{build, outdated, patch_makepkg, print_config, sort, sync_srcinfo},
};
use super::App;

impl App {
    pub fn run(self) -> i32 {
        let Args { command } = self.args;
        match command {
            Command::PrintConfig(args) => print_config(args),
            Command::Sort(args) => sort(args),
            Command::Outdated(args) => outdated(args),
            Command::SyncSrcInfo(args) => sync_srcinfo(args),
            Command::PatchMakePkg(args) => patch_makepkg(args),
            Command::Build(args) => build(args),
        }
    }
}
