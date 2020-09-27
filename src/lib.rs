pub mod app;
pub mod args;
pub mod makepkg;
pub mod srcinfo;
pub mod utils;

pub fn main() {
    let args: args::Args = argh::from_env();
    dbg!(args);
    unimplemented!()
}

pub use alpm;
pub use argh;
pub use num_bigint;
pub use num_traits;
pub use topological_sort;
