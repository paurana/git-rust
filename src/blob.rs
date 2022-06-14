use flate2::write::ZlibEncoder as WriteEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct Blob {}

impl Blob {
    pub fn cat_file(content: String) -> Result<()> {
        // Git first constructs a header which starts by identifying the type of object --- in this case, a blob.
        // To that first part of the header, Git adds a space followed by the size in bytes of the content, and adding a final null byte:
        let s = &content[5..content.len()];
        //start_length = 5 to remove "blob "

        for i in 0..s.len() {
            if let Ok(length) = s[..i].parse::<usize>() {
                if length == s[i + 1..].len() {
                    let s = s.replace("\u{0000}", "");
                    let content = &s[i..];
                    print!("{}", content);
                    return Ok(());
                }
            }
        }

        Err("Invalid Object".into())
    }

    pub fn hash_object(args: Vec<String>) -> Result<()> {
        let filename = &args[3];
        let fs = fs::read_to_string(filename)?;

        let blob_content = format!("blob {}\u{0000}{}", fs.len(), fs);
        let sha1 = <Sha1 as Digest>::digest(blob_content.as_bytes());
        let hex_sha1 = hex::encode(sha1);

        println!("{}", hex_sha1);

        // let mut z = ZlibEncoder::new(blob_content.as_bytes(), Compression::default());
        // println!("{:?}", z);
        // let mut buffer = Vec::new();
        // z.read(&mut buffer)?;
        // println!("{:?}", buffer);

        let mut e = WriteEncoder::new(Vec::new(), Compression::default());
        e.write_all(&blob_content.as_bytes())?;
        let buffer = e.finish()?;

        let file_dir = format!(".git/objects/{}", &hex_sha1[..2]);
        if !Path::new(&file_dir).exists() {
            fs::create_dir(file_dir)?;
        }

        let file_path = format!(".git/objects/{}/{}", &hex_sha1[..2], &hex_sha1[2..40]);
        let mut f = File::create(file_path)?;
        f.write(&buffer)?;

        Ok(())
    }
}
