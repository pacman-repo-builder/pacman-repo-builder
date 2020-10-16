pub mod app;
pub mod args;
pub mod cmd;
pub mod manifest;
pub mod srcinfo;
pub mod status;
pub mod utils;

pub fn main() {
    use pipe_trait::*;
    app::App::from_env()
        .run()
        .pipe(status::get_code)
        .pipe(std::process::exit)
}

pub use alpm;
pub use argh;
pub use indexmap;
pub use num_bigint;
pub use num_traits;
pub use petgraph;
