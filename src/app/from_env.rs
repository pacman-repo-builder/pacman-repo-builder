use super::App;

impl App {
    pub fn from_env() -> Self {
        App {
            args: argh::from_env(),
        }
    }
}
