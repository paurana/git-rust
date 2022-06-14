use std::env;

mod args;
mod blob;
mod object;
mod tree;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    args::parse(args)
}
