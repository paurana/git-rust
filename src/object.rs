use crate::tree::Tree;
use crate::utils::Utils;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder as WriteEncoder;
use flate2::Compression;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::str;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct Object {
    pub object_type: ObjectType,
    pub content: Vec<u8>,
}

pub enum ObjectType {
    Blob,
    Tree,
    Commit,
}

impl Display for ObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            ObjectType::Blob => write!(f, "blob"),
            ObjectType::Tree => write!(f, "tree"),
            ObjectType::Commit => write!(f, "commit"),
        }
    }
}

impl Object {
    pub fn open(object_sha: String) -> Result<Object> {
        // let object_name = &args[3];
        let dir_sha = &object_sha[..2];
        let file_name = &object_sha[2..];

        let file_dir = format!(".git/objects/{}/{}", dir_sha, file_name);
        let f = File::open(file_dir)?;

        let mut z = ZlibDecoder::new(f);
        let mut buffer = Vec::new();
        z.read_to_end(&mut buffer)?;
        // println!("{:?}", buffer);

        let index = buffer
            .iter()
            .position(|x| *x == ' ' as u8)
            .expect("Error identifying Object Type");

        let git_object = str::from_utf8(&buffer[..index])?;

        match git_object {
            "blob" => Ok(Object {
                object_type: ObjectType::Blob,
                content: buffer,
            }),
            "tree" => Ok(Object {
                object_type: ObjectType::Tree,
                content: buffer,
            }),
            "commit" => Ok(Object {
                object_type: ObjectType::Commit,
                content: buffer,
            }),
            _ => Err("Unidentified Git Object".into()),
        }
    }

    pub fn hash_object<T: AsRef<Path>>(object: ObjectType, filename: T) -> Result<String> {
        let hex_sha1;
        let content;

        match object {
            ObjectType::Tree => {
                let ref_entries = Tree::tree_content(filename)?;
                let byte_vec = Tree::ref_entries_to_bytes(ref_entries)?;

                let mut byte_content: Vec<u8> = Vec::new();
                byte_content.extend("tree ".as_bytes());
                byte_content.extend(byte_vec.len().to_string().as_bytes());
                byte_content.push('\0' as u8);
                byte_content.extend(byte_vec);
                hex_sha1 = Utils::hex_sha1(&byte_content);
                content = byte_content;
            }
            ObjectType::Commit => {
                return Err("Function not implemented for Commit Objects".into());
            }
            ObjectType::Blob => {
                let fs = fs::read_to_string(filename.as_ref())?;
                content = format!("{} {}\u{0000}{}", object, fs.len(), fs)
                    .as_bytes()
                    .to_vec();
                hex_sha1 = Utils::hex_sha1(&content);
            }
        }

        let mut e = WriteEncoder::new(Vec::new(), Compression::default());
        e.write_all(&content)?;
        let buffer = e.finish()?;

        let file_dir = format!(".git/objects/{}", &hex_sha1[..2]);
        if !Path::new(&file_dir).exists() {
            fs::create_dir(file_dir)?;
        }

        let file_path = format!(".git/objects/{}/{}", &hex_sha1[..2], &hex_sha1[2..40]);
        if !Path::new(&file_path).exists() {
            let mut f = File::create(file_path)?;
            f.write(&buffer)?;
        }

        Ok(hex_sha1)
    }
}
