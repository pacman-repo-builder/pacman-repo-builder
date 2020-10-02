use super::super::args::{Args, Command};
use super::App;

impl App {
    pub fn run(self) -> i32 {
        let Args { command } = self.args;
        match command {
            Command::PrintConfig(_) => unimplemented!(),
            Command::Sort(_) => unimplemented!(),
        }
    }
}
