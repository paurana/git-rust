use std::str;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct Blob {}

impl Blob {
    pub fn cat_file(content: Vec<u8>) -> Result<()> {
        let content = str::from_utf8(&content).expect("stream did not contain valid utf8");
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
}
