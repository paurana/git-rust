mod args;
mod blob;
mod clone;
mod commands;
mod commit;
mod object;
mod packfile;
mod tree;
mod utils;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[tokio::main]
async fn main() -> Result<()> {
    commands::run().await
}
