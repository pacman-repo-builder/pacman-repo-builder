use super::super::{
    args::{Args, Command},
    cmd::print_config,
};
use super::App;

impl App {
    pub fn run(self) -> i32 {
        let Args { command } = self.args;
        match command {
            Command::PrintConfig(args) => print_config(args),
            Command::Sort(_) => unimplemented!(),
        }
    }
}
