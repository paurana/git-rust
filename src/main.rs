use std::env;

mod args;
mod blob;
mod commit;
mod object;
mod tree;
mod utils;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    args::parse(args)
}
