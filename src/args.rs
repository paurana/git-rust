use crate::blob::Blob;
use crate::object::{Object, ObjectType};
use crate::tree::Tree;
use std::fs;
#[allow(unused_imports)]
#[allow(dead_code)]

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

//todo: use clap for parsing cli args instead of matching &str

pub fn parse(args: Vec<String>) -> Result<()> {
    assert!(args.len() > 1);
    let arg = &args[1][..];
    match arg {
        "init" => init(),
        "cat-file" => cat_file(args),
        "hash-object" => hash_object(args),
        "ls-tree" => ls_tree(args),
        "write-tree" => Tree::write_tree(),
        _ => {
            println!("unknown command: {}", args[1]);
            Ok(())
        }
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

pub fn cat_file(args: Vec<String>) -> Result<()> {
    let option = &args[2][..];
    match option {
        "-p" => {
            let object = Object::open(args)?;

            match object.object_type {
                ObjectType::Blob => Blob::cat_file(object.content),
                _ => todo!(),
            }
        }
        second_arg => {
            println!("{} not supported with cat-file", second_arg);
            Ok(())
        }
    }
}

pub fn hash_object(args: Vec<String>) -> Result<()> {
    let option = &args[2][..];
    match option {
        "-w" => {
            let hex_sha1 = Object::hash_object(ObjectType::Blob, &args[3])?;
            println!("{}", hex_sha1);
            Ok(())
        }
        second_arg => {
            println!("{} not supported with hash-object", second_arg);
            Ok(())
        }
    }
}

pub fn ls_tree(args: Vec<String>) -> Result<()> {
    let option = &args[2][..];
    match option {
        "--name-only" => {
            let object = Object::open(args)?;
            match object.object_type {
                ObjectType::Tree => Tree::ls_tree(object.content),
                _ => Err("Only for Tree Objects".into()),
            }
        }
        second_arg => {
            println!("{} not supported with hash-object", second_arg);
            Ok(())
        }
    }
}
