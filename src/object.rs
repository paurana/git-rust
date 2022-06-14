use flate2::read::ZlibDecoder;
use std::fs::File;
use std::io::Read;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct Object {
    pub object_type: ObjectType,
    pub decoded_string: String,
}

pub enum ObjectType {
    Blob,
    Tree,
    Commit,
}

impl Object {
    pub fn open(args: Vec<String>) -> Result<Object> {
        let object_name = &args[3];
        let dir_sha = &object_name[..2];
        let file_name = &object_name[2..];

        let file_dir = format!(".git/objects/{}/{}", dir_sha, file_name);
        let f = File::open(file_dir)?;

        let mut z = ZlibDecoder::new(f);
        let mut s = String::new();
        z.read_to_string(&mut s)?;

        let git_object = s.split(' ').next().expect("Error identifying Object Type");
        match git_object {
            "blob" => Ok(Object {
                object_type: ObjectType::Blob,
                decoded_string: s.to_string(),
            }),
            "tree" => Ok(Object {
                object_type: ObjectType::Tree,
                decoded_string: s.to_string(),
            }),
            "commit" => Ok(Object {
                object_type: ObjectType::Commit,
                decoded_string: s.to_string(),
            }),
            _ => Err("Unidentified Git Object".into()),
        }
    }

    pub fn open_tree(args: Vec<String>) -> Result<Object> {
        let object_name = &args[3];
        let dir_sha = &object_name[..2];
        let file_name = &object_name[2..];

        let file_dir = format!(".git/objects/{}/{}", dir_sha, file_name);
        let f = File::open(file_dir)?;

        let mut z = ZlibDecoder::new(f);
        let mut buffer = vec![0; 32 * 1024];
        z.read(&mut buffer)?;

        let string = String::from_utf8_lossy(&buffer[..]);
        let string = string.trim_matches(char::from(0)).to_string();

        // Git first constructs a header which starts by identifying the type of object --- in this case, a tree.
        // To that first part of the header, Git adds a space followed by the size in bytes of the content, and adding a final null byte:
        let string = &string[5..string.len()];
        //start_length = 5 to remove "tree "

        let mut content = String::new();
        if let Some(index) = string.find("\0") {
            if let Ok(_length) = string[..index].parse::<usize>() {
                content = string[index..].to_string();
            }
        }

        Ok(Object {
            object_type: ObjectType::Tree,
            decoded_string: content,
        })
    }
}
