use crate::args::*;
use crate::blob::Blob;
use crate::commit::Commit;
use crate::object::{Object, ObjectType};
use crate::tree::Tree;
use clap::Parser;
use std::fs;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn run() -> Result<()> {
    let cli = Args::parse();

    match &cli.command {
        Commands::Init => init(),
        Commands::catFile(args) => cat_file(args),
        Commands::HashObject(args) => hash_object(args),
        Commands::lsTree(args) => ls_tree(args),
        Commands::WriteTree => Tree::write_tree(),
        Commands::CommitTree(args) => commit_tree(args),
    }
}

pub fn init() -> Result<()> {
    fs::create_dir(".git")?;
    fs::create_dir(".git/objects")?;
    fs::create_dir(".git/refs")?;
    fs::write(".git/HEAD", "ref: refs/heads/master\n")?;
    println!("Initialized git directory");
    Ok(())
}

pub fn cat_file(args: &catFile) -> Result<()> {
    let object = Object::open(args.sha1.to_string())?;

    match object.object_type {
        ObjectType::Blob => Blob::cat_file(object.content),
        _ => todo!(),
    }
}

pub fn hash_object(args: &HashObject) -> Result<()> {
    let hex_sha1 = Object::hash_object(ObjectType::Blob, &args.path)?;
    println!("{}", hex_sha1);
    Ok(())
}

pub fn ls_tree(args: &lsTree) -> Result<()> {
    let object = Object::open(args.sha1.to_string())?;
    match object.object_type {
        ObjectType::Tree => Tree::ls_tree(object.content),
        _ => Err("Only for Tree Objects".into()),
    }
}

pub fn commit_tree(args: &CommitTree) -> Result<()> {
    Commit::commit_tree(args)
}
