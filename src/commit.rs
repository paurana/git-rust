use crate::utils::Utils;

use std::{
    convert::TryInto,
    time::{SystemTime, UNIX_EPOCH},
};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct Commit {}

pub struct CommitEntry {
    tree: [u8; 40],
    parent: [u8; 40], //only one parent will be provided
    author: Author,
    committer: Committer,
    message: String, //only one message will be provided
}

struct Author {
    name: String,
    email: String,
    time: u64,
    offset: String,
}

struct Committer {
    name: String,
    email: String,
    time: u64,
    offset: String,
}

impl Commit {
    pub fn commit_tree(args: Vec<String>) -> Result<()> {
        let author = Author {
            name: String::from("Aayush author"),
            email: String::from("aayushauthor@gmail.com"),
            time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            offset: String::from("+530"),
        };

        let committer = Committer {
            name: String::from("Aayush author"),
            email: String::from("aayushauthor@gmail.com"),
            time: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            offset: String::from("+530"),
        };

        let tree: [u8; 40] = args[2].as_bytes().try_into()?;
        let parent: [u8; 40] = args[4].as_bytes().try_into()?;
        let message: String = args[6].to_string();

        let commit_entry = CommitEntry {
            tree,
            parent,
            author,
            committer,
            message,
        };
        
        let commit_entry_in_bytes  = Commit::commit_entry_to_bytes(commit_entry);

        let mut byte_content: Vec<u8> = Vec::new();
        byte_content.extend("commit ".as_bytes());
        byte_content.extend(commit_entry_in_bytes.len().to_string().as_bytes());
        byte_content.push('\0' as u8);
        byte_content.extend(commit_entry_in_bytes);
        let hex_sha1 = Utils::hex_sha1(&byte_content);

        println!("{hex_sha1}");

        Ok(())

    }

    pub fn commit_entry_to_bytes(entry: CommitEntry) -> Vec<u8> {
        let mut vec: Vec<u8> = Vec::new();

        vec.extend("tree ".as_bytes());
        vec.extend(entry.tree);
        vec.push('\n' as u8);

        vec.extend("parent ".as_bytes());
        vec.extend(entry.parent);
        vec.push('\n' as u8);

        vec.extend("author ".as_bytes());
        vec.extend(entry.author.name.as_bytes());
        vec.push(' ' as u8);
        vec.extend(entry.author.email.as_bytes());
        vec.extend(entry.author.time.to_string().as_bytes());
        vec.push(' ' as u8);
        vec.extend(entry.author.offset.as_bytes());
        vec.push('\n' as u8);

        vec.extend("committer ".as_bytes());
        vec.extend(entry.committer.name.as_bytes());
        vec.push(' ' as u8);
        vec.extend(entry.committer.email.as_bytes());
        vec.extend(entry.committer.time.to_string().as_bytes());
        vec.push(' ' as u8);
        vec.extend(entry.committer.offset.as_bytes());
        vec.push('\n' as u8);

        vec.push('\n' as u8);

        vec.extend(entry.message.as_bytes());
        
        vec
    }
}
