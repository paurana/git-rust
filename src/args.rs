use flate2::read::ZlibDecoder;
use std::fs;
use std::fs::File;
use std::io::Read;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn parse(args: Vec<String>) -> Result<()> {
    let arg = &args[1][..];
    match arg {
        "init" => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/master\n").unwrap();
            println!("Initialized git directory");
            Ok(())
        }
        "cat-file" => cat_file(args),
        _ => {
            println!("unknown command: {}", args[1]);
            Ok(())
        }
    }
}

pub fn cat_file(args: Vec<String>) -> Result<()> {
    let option = &args[2][..];
    match option {
        "-p" => {
            let object_name = &args[3];
            let dir_sha = &object_name[..2];
            let file_name = &object_name[2..];

            let file_dir = format!(".git/objects/{}/{}", dir_sha, file_name);
            let f = File::open(file_dir)?;

            let mut z = ZlibDecoder::new(f);
            let mut s = String::new();
            z.read_to_string(&mut s)?;

            // Git first constructs a header which starts by identifying the type of object --- in this case, a blob.
            // To that first part of the header, Git adds a space followed by the size in bytes of the content, and adding a final null byte:
            let s = &s[5..s.len()];
            //start_length = 5 to remove "blob "
            //s.len()-1 to remove the null byte at the end

            for i in 0..s.len() {
                if let Ok(length) = s[..i].parse::<usize>() {
                    if length == s[i+1..].len() {
                        let s = s.replace("\u{0000}", "");
                        let content = &s[i..];
                        print!("{}", content);
                        return Ok(());
                    }
                }
            }

            Err("Invalid Object".into())
        }
        second_arg => {
            println!("{} not supported with cat-file", second_arg);
            Ok(())
        }
    }
}
