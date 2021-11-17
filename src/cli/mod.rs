pub mod args;

use args::Args;

pub fn parse_flags() -> Args {
    Args::read()
}
