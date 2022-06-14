use crate::blob::Blob;
use crate::object::{Object, ObjectType};
use crate::tree::Tree;
use std::fs;
#[allow(unused_imports)]
#[allow(dead_code)]

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn parse(args: Vec<String>) -> Result<()> {
    assert!(args.len() > 1);
    let arg = &args[1][..];
    match arg {
        "init" => init(),
        "cat-file" => cat_file(args),
        "hash-object" => hash_object(args),
        "ls-tree" => ls_tree(args),
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
                ObjectType::Blob => Blob::cat_file(object.decoded_string),
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
        "-w" => Blob::hash_object(args),
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
            let object = Object::open_tree(args)?;
            match object.object_type {
                ObjectType::Tree => Tree::ls_tree(object.decoded_string),
                _ => Err("Only for Tree Objects".into()),
            }
        }
        second_arg => {
            println!("{} not supported with hash-object", second_arg);
            Ok(())
        }
    }
}
