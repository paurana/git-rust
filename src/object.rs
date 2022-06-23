use crate::utils;
use flate2::read::ZlibDecoder;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
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

    pub fn hash_object<T: AsRef<[u8]>>(object_type: ObjectType, byte_vec: T) -> Result<String> {
        let hex_sha1;
        let content;

        match object_type {
            ObjectType::Tree => {
                let mut byte_content: Vec<u8> = Vec::new();
                byte_content.extend("tree ".as_bytes());
                byte_content.extend(byte_vec.as_ref().len().to_string().as_bytes());
                byte_content.push('\0' as u8);
                byte_content.extend(byte_vec.as_ref());
                hex_sha1 = utils::hex_sha1(&byte_content);
                content = byte_content;
            }
            ObjectType::Commit => {
                let mut byte_content: Vec<u8> = Vec::new();
                byte_content.extend("commit ".as_bytes());
                byte_content.extend(byte_vec.as_ref().len().to_string().as_bytes());
                byte_content.push('\0' as u8);
                byte_content.extend(byte_vec.as_ref());
                hex_sha1 = utils::hex_sha1(&byte_content);
                content = byte_content;
            }
            ObjectType::Blob => {
                let fs = byte_vec.as_ref();
                content = format!(
                    "{} {}\u{0000}{}",
                    object_type,
                    fs.len(),
                    str::from_utf8(fs)?
                )
                .as_bytes()
                .to_vec();
                hex_sha1 = utils::hex_sha1(&content);
            }
        }

        utils::save_object(hex_sha1.to_string(), content)?;

        Ok(hex_sha1)
    }
}
