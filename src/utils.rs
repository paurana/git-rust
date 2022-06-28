use sha1::{Digest, Sha1};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::{fs, path::Path, path::PathBuf};

use flate2::write::ZlibEncoder as WriteEncoder;
use flate2::Compression;
use std::io::Write;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn hex_sha1<T: AsRef<[u8]>>(data: T) -> String {
    let sha1 = Sha1::digest(data.as_ref());
    let hex_sha1 = hex::encode(sha1);

    hex_sha1
}

pub fn save_object<T: AsRef<[u8]>>(hex_sha1: &str, data: T) -> Result<()> {
    let mut e = WriteEncoder::new(Vec::new(), Compression::default());
    e.write_all(data.as_ref())?;
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

    Ok(())
}

pub fn sorted_current_dir<T: AsRef<Path>>(dir_path: T) -> Result<Vec<PathBuf>> {
    use std::io::Result;

    //we get Vec<PathBuf>
    let mut entries = fs::read_dir(dir_path.as_ref())?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>>>()?;

    entries.sort();
    Ok(entries)
}

pub fn gitignored_sorted_current_dir<T: AsRef<Path>>(dir_path: T) -> Result<Vec<PathBuf>> {
    let mut entries = sorted_current_dir(dir_path.as_ref())?;

    let mut gitignore_content = Vec::new();

    let mut path = PathBuf::new();
    match dir_path.as_ref().to_str() {
        Some(".") => {
            path.push(".gitignore");
            gitignore_content.push(String::from("./.git"));
        }
        Some(dir) => {
            path.push(dir);
            let gitpath = format!("{}/.git", dir);
            gitignore_content.push(gitpath);
            if path.ends_with("/") {
                path.push(".gitignore");
            } else {
                path.push("/.gitignore");
            }
        }
        None => {
            return Err("Path Error".into());
        }
    }

    //not sure how git actually implements this, but I'm assuming that
    //.git is ignored in subdirectories as well. Also considering any
    //.gitignores these subfolders might have
    if Path::new(&path).exists() {
        let gitignore = File::open(path)?;
        let reader = BufReader::new(gitignore);

        for line in reader.lines() {
            let mut name = line?;
            if name.starts_with("/") {
                name = name.strip_prefix("/").unwrap().to_string();
            }
            let mut new_path = String::new();
            let str = dir_path.as_ref().to_str().unwrap();
            if str.ends_with("/") {
                new_path.push_str(str);
                new_path.push_str(&name);
            } else {
                new_path.push_str(str);
                new_path.push_str("/");
                new_path.push_str(&name);
            }
            gitignore_content.push(new_path);
        }
    }

    entries.retain(|x| {
        let mut boolean = true;
        for ignored in &gitignore_content {
            if *x == PathBuf::from(ignored) {
                boolean = false;
                break;
            }
        }
        boolean
    });

    Ok(entries)
}
