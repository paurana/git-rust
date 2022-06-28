#![allow(non_camel_case_types)]

use std::{
    convert::TryInto,
    fmt::Debug,
};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct Packfile {
    header: PackHeader,
    chunks: Vec<u8>,
    checksum: [u8; 20],
}

impl Debug for Packfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} {:?}", self.header, self.chunks, self.checksum)
    }
}

enum _OBJECT_TYPE {
    OBJ_COMMIT = 1,
    OBJ_TREE = 2,
    OBJ_BLOB = 3,
    OBJ_TAG = 4,
    OBJ_OFS_DELTA = 6,
    OBJ_REF_DELTA = 7,
}

struct PackHeader {
    pack: [u8; 4],
    version: [u8; 4],
    object_number: [u8; 4],
}

impl Debug for PackHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?} {:?} {:?}",
            self.pack, self.version, self.object_number
        )
    }
}

impl Packfile {
    pub fn new<T: AsRef<[u8]>>(bytes: T) -> Result<Self> {
        let header = PackHeader {
            pack: bytes.as_ref()[..4].try_into().unwrap(),
            version: bytes.as_ref()[4..8].try_into().unwrap(),
            object_number: bytes.as_ref()[8..12].try_into().unwrap(),
        };

        let pack_header: [u8; 4] = "PACK".as_bytes().try_into()?;
        if header.pack != pack_header {
            return Err("Invalid Pack Header".into());
        }

        let byte_length = bytes.as_ref().len();
        Ok(Packfile {
            header,
            chunks: bytes.as_ref()[12..byte_length - 20].to_vec(),
            checksum: bytes.as_ref()[byte_length - 20..byte_length]
                .try_into()
                .unwrap(),
        })
    }

    pub fn parse_pack(&self) {
        todo!()
    }
}
