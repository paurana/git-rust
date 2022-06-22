use std::env;

mod args;
mod blob;
mod clone;
mod commit;
mod object;
mod packfile;
mod tree;
mod utils;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    args::parse(args).await
}
