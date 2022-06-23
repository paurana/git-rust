mod args;
mod blob;
mod commands;
mod commit;
mod object;
mod tree;
mod utils;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    commands::run()
}
