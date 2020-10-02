pub mod app;
pub mod args;
pub mod makepkg;
pub mod manifest;
pub mod srcinfo;
pub mod utils;

pub fn main() {
    app::App::from_env().run()
}

pub use alpm;
pub use argh;
pub use indexmap;
pub use num_bigint;
pub use num_traits;
pub use topological_sort;
