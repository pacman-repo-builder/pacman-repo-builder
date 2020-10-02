use super::args::{Args, Command};

pub struct App {
    pub args: Args,
}

impl App {
    pub fn from_env() -> Self {
        App {
            args: argh::from_env(),
        }
    }

    pub fn run(self) {
        let Args { command } = self.args;
        match command {
            Command::PrintConfig(_) => unimplemented!(),
            Command::Sort(_) => unimplemented!(),
        };
    }
}
