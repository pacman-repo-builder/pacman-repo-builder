pub mod app;
pub mod args;
pub mod cmd;
pub mod makepkg;
pub mod manifest;
pub mod srcinfo;
pub mod utils;

pub fn main() {
    use pipe_trait::*;
    app::App::from_env().run().pipe(std::process::exit)
}

pub use alpm;
pub use argh;
pub use indexmap;
pub use num_bigint;
pub use num_traits;
pub use petgraph;
