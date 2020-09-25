use super::super::args::Args;
use structopt_utilities::StructOptUtils;

pub fn main() {
    Args::run_completion_generator("strip-ansi-completions", "strip-ansi")
}
