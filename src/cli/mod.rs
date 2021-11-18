pub mod args;
pub mod env;
pub use args::Args;

pub fn parse_flags() -> Args {
    Args::read()
}
