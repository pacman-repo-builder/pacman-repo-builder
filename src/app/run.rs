use super::super::{
    args::{Args, Command},
    cmd::{outdated, print_config, sort},
};
use super::App;

impl App {
    pub fn run(self) -> i32 {
        let Args { command } = self.args;
        match command {
            Command::PrintConfig(args) => print_config(args),
            Command::Sort(args) => sort(args),
            Command::Outdated(args) => outdated(args),
        }
    }
}
