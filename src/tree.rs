use std::convert::TryInto;
use std::fmt::Debug;
use std::fs;
use std::fs::File;
use std::os::unix::fs::PermissionsExt;
use std::{path::Path, path::PathBuf};

use crate::object::{Object, ObjectType};
use crate::utils;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct RefEntry {
    pub mode: u32,
    pub outer_tree: PathBuf,
    pub sha1: [u8; 40],
}

impl Debug for RefEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_sha1 = String::from_utf8_lossy(&self.sha1);
        write!(
            f,
            "{} {} {}",
            self.mode,
            self.outer_tree.display(),
            string_sha1,
        )
    }
}

pub struct Tree {}

impl Tree {
    const POSSIBLE_MODES: [u32; 7] = [100664, 100775, 100644, 100755, 120000, 40000, 160000];

    pub fn ls_tree(data: Vec<u8>) -> Result<()> {
        let header = std::str::from_utf8(&data[..5])?;
        if header != "tree " {
            return Err("Invalid Header, Not a Tree Object".into());
        }
        let mut index = data.iter().position(|x| *x == '\0' as u8).unwrap();

        let mut ref_entries: Vec<RefEntry> = Vec::new();

        let mut counter = index + 1;
        while counter <= data.len() {
            index = data[counter..]
                .iter()
                .position(|x| *x == ' ' as u8)
                .unwrap();
            let mode: u32 = std::str::from_utf8(&data[counter..counter + index])?
                .parse()
                .unwrap();
            counter += index + 1;
            index = data[counter..]
                .iter()
                .position(|x| *x == '\0' as u8)
                .unwrap();
            let outer_tree = PathBuf::from(std::str::from_utf8(&data[counter..counter + index])?);
            counter += index + 1;
            // let hex_sha1: [u8; 20] = data[counter..counter+20].try_into().unwrap();
            let sha1: [u8; 40] = hex::encode(&data[counter..counter + 20])
                .as_bytes()
                .try_into()?;

            ref_entries.push(RefEntry {
                mode,
                outer_tree,
                sha1,
            });
            counter += 20;

            let break_var = data[counter..].iter().position(|x| *x == ' ' as u8);
            if break_var.is_none() {
                break;
            }
        }

        for entries in &ref_entries {
            println!("{}", entries.outer_tree.display());
        }

        Ok(())
    }

    pub fn tree_content<T: AsRef<Path>>(path: T) -> Result<Vec<RefEntry>> {
        let absolute_entries: Vec<PathBuf> = utils::gitignored_sorted_current_dir(path.as_ref())?;

        let mut tree: Vec<RefEntry> = Vec::new();

        for i in absolute_entries {
            let f = File::open(&i)?;
            let metadata = f.metadata()?;
            let permissions = metadata.permissions();
            let mut git_mode: u32 = format!("{:o}", permissions.mode()).parse().unwrap();

            if metadata.is_file() {
                if Tree::POSSIBLE_MODES.contains(&git_mode) {
                    if git_mode == 100664 {
                        git_mode = 100644;
                    } else if git_mode == 100775 {
                        git_mode = 100755;
                    }

                    let byte_vec = fs::read(&i)?;
                    let hex_sha1 = Object::hash_object(ObjectType::Blob, byte_vec)?;

                    let entry = RefEntry {
                        mode: git_mode,
                        outer_tree: i,
                        sha1: hex_sha1
                            .as_bytes()
                            .try_into()
                            .expect("Incorrect sha1 length"),
                    };

                    tree.push(entry);
                }
            } else if metadata.is_dir() {
                git_mode = 40000;

                let ref_entries = Tree::tree_content(&i)?;
                let byte_vec = Tree::ref_entries_to_bytes(ref_entries)?;
                let sha1 = Object::hash_object(ObjectType::Tree, byte_vec)?;

                let entry = RefEntry {
                    mode: git_mode,
                    outer_tree: i,
                    sha1: sha1.as_bytes().try_into().expect("Incorrect sha1 length"),
                };

                tree.push(entry);
            }
        }

        tree.sort_by_key(|x| x.outer_tree.clone());

        //a cool edge case I came across. took me a lot of time to figure this one out. very
        //surprised this is how git has implemented file order
        if tree.len() > 0 {
            for i in 0..tree.len() - 1 {
                let pathbuf = &tree[i].outer_tree;
                if let None = pathbuf.extension() {
                    if pathbuf.is_dir() {
                        if let Some(extension) = &tree[i + 1].outer_tree.extension() {
                            let mut new_pathname = String::new();
                            let string = &tree[i].outer_tree.display().to_string();
                            new_pathname.push_str(&string);
                            new_pathname.push_str(".");
                            new_pathname.push_str(extension.to_str().unwrap());
                            if tree[i + 1].outer_tree.display().to_string() == new_pathname {
                                tree.swap(i, i + 1);
                            }
                        }
                    }
                }
            }
        }

        Ok(tree)
    }

    pub fn ref_entries_to_bytes(vec: Vec<RefEntry>) -> Result<Vec<u8>> {
        let mut bytes: Vec<u8> = Vec::new();
        for object in vec {
            bytes.extend(object.mode.to_string().as_bytes());
            bytes.push(' ' as u8);

            let mut path = object.outer_tree;
            if let Some(index) = path.to_str().unwrap().rfind("/") {
                let pathbuf_string = path.display().to_string();
                let prefix = &pathbuf_string[..index + 1];
                path = path.strip_prefix(prefix).unwrap().to_path_buf();
            }

            bytes.extend(
                path.into_os_string()
                    .into_string()
                    .expect("Path has Invalid Unicode Data")
                    .as_bytes(),
            );
            bytes.push('\0' as u8);
            let decoded_hex = hex::decode(object.sha1)?;
            bytes.extend(decoded_hex);
        }

        Ok(bytes)
    }

    pub fn write_tree() -> Result<()> {
        let ref_entries = Tree::tree_content(".")?;
        let byte_vec = Tree::ref_entries_to_bytes(ref_entries)?;
        let sha1 = Object::hash_object(ObjectType::Tree, byte_vec)?;

        println!("{}", sha1);

        Ok(())
    }
}
