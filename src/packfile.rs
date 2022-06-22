#![allow(non_camel_case_types)]

// use num_traits::pow;

use std::{
    convert::{TryFrom, TryInto},
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

enum OBJECT_TYPE {
    OBJ_COMMIT = 1,
    OBJ_TREE = 2,
    OBJ_BLOB = 3,
    OBJ_TAG = 4,
    OBJ_OFS_DELTA = 6,
    OBJ_REF_DELTA = 7,
}

impl TryFrom<usize> for OBJECT_TYPE {
    type Error = Box<dyn std::error::Error>;
    fn try_from(value: usize) -> Result<Self> {
        match value {
            x if x == OBJECT_TYPE::OBJ_COMMIT as usize => Ok(OBJECT_TYPE::OBJ_COMMIT),
            x if x == OBJECT_TYPE::OBJ_TREE as usize => Ok(OBJECT_TYPE::OBJ_TREE),
            x if x == OBJECT_TYPE::OBJ_BLOB as usize => Ok(OBJECT_TYPE::OBJ_BLOB),
            x if x == OBJECT_TYPE::OBJ_TAG as usize => Ok(OBJECT_TYPE::OBJ_TAG),
            x if x == OBJECT_TYPE::OBJ_OFS_DELTA as usize => Ok(OBJECT_TYPE::OBJ_OFS_DELTA),
            x if x == OBJECT_TYPE::OBJ_REF_DELTA as usize => Ok(OBJECT_TYPE::OBJ_REF_DELTA),
            _ => Err("ObjectType Mismatch".into()),
        }
    }
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
        println!("{}", self.chunks.len());
        let mut i = 0;
        while i < self.chunks.len() {
            let mut length: Vec<String> = Vec::new();

            let mut byte = self.chunks[i];

            let mut read = byte & 0b10000000;
            let mut count = 1;
            let mut size = (byte & 0b00001111) as u64;

            let object_type = format!("{}{}{}", byte >> 6 & 1, byte >> 5 & 1, byte >> 4 & 1);
            let _int_type = usize::from_str_radix(&object_type, 2).unwrap();

            // match int_type.try_into() {
            //     Ok(OBJECT_TYPE::OBJ_REF_DELTA) => {}
            //     _ => {}
            // }
            let length_str = format!(
                "{}{}{}{}",
                byte >> 3 & 1,
                byte >> 2 & 1,
                byte >> 1 & 1,
                byte >> 0 & 1
            );
            length.push(length_str);

            if byte >= 128 {
                loop {
                    let byte = self.chunks[i];
                    if byte >= 128 {
                        // let length_continues = byte & 0b01111111;
                        let length_str = format!(
                            "{}{}{}{}{}{}{}",
                            byte >> 6 & 1,
                            byte >> 5 & 1,
                            byte >> 4 & 1,
                            byte >> 3 & 1,
                            byte >> 2 & 1,
                            byte >> 1 & 1,
                            byte >> 0 & 1
                        );
                        // length2.push(byte-128);
                        length.push(length_str);
                        i += 1;
                    } else {
                        let length_str = format!(
                            "{}{}{}{}{}{}{}",
                            byte >> 6 & 1,
                            byte >> 5 & 1,
                            byte >> 4 & 1,
                            byte >> 3 & 1,
                            byte >> 2 & 1,
                            byte >> 1 & 1,
                            byte >> 0 & 1
                        );
                        length.push(length_str);
                        // length2.push(byte);
                        i += 1;
                        break;
                    }
                }
            }

            let mut inner = i;
            while read > 0 && inner < self.chunks.len(){
                byte = self.chunks[inner];
                println!("{} {}", inner, byte);
                inner += 1;

                size |= ((byte & 127) as u64) << (4 + 7 * count);
                count +=1;
                read = byte & 0b10000000;
            }
            println!("{:b}", size);


            let length_1  = length.len();
            let len  = packfile_len(length);
            if len != size {
                println!("bruh {} {} {}" , i, len, size);
                break;
            }
            i += 4 + 7*(length_1 -1);

        }
    }
}

fn packfile_len(mut vec: Vec<String>) -> u64 {
    print!("hi {:?}", vec);

    let mut boolean = false;

    for i in 1..=vec.len() {
        if i == vec.len() {
            if boolean {
                let bin_one = 0b01;
                let offset_adjusted = format!("{:07b}", bin_one);
                boolean = false;
                vec.push(offset_adjusted);
            }
        } else {
            let string = &vec[i];
            let str_length = usize::from_str_radix(&string, 2).unwrap();
            let bin_one = 0b01;
            let offset_adjusted;
            if boolean {
                offset_adjusted = format!("{:07b}", str_length + bin_one + bin_one);
                boolean = false;
            } else {
                offset_adjusted = format!("{:07b}", str_length + bin_one);
            }
            if offset_adjusted.len() != 8 {
                vec[i] = offset_adjusted;
            } else {
                // println!("{}", offset_adjusted);
                vec[i] = "0000000".to_string();
                boolean = true;
            }
        }
    }

    // print!("{:?}", vec);
    vec.reverse();
    print!("{:?}", vec);
    let joined = vec.join("");
    // print!("{:?}", vec);
    let length = u64::from_str_radix(&joined, 2).unwrap();
    // println!("{}", length);
    return length;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_length() {
        let str = "0000001";
        let str2 = "1111111";
        let mut test_vec: Vec<String> = Vec::new();
        test_vec.push(str.to_string());
        test_vec.push(str2.to_string());

        let length = packfile_len(test_vec);

        //blog says length should be equal to 16835?!!
        assert_eq!(length, 16385)
    }
}
