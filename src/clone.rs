use reqwest::StatusCode;

use crate::packfile::Packfile;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct Clone {
    client: reqwest::Client,
    url: String,
}

impl Clone {
    pub fn new(mut url: String) -> Self {
        if url.ends_with("/") {
            url.pop();
            url.push_str(".git");
        } else {
            url.push_str(".git");
        }

        Clone {
            url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn clone(&self) -> Result<()> {
        Clone::request(&self).await?;
        Ok(())
    }

    async fn request(&self) -> Result<()> {
        let get_uri = format!("{}/info/refs?service=git-upload-pack", self.url);
        let post_uri = format!("{}/git-upload-pack", self.url);

        let resp = self.client.get(&get_uri).send().await?;
        if resp.status() != StatusCode::OK && resp.status() != StatusCode::NOT_MODIFIED {
            return Err("Could not connect with the repository".into());
        }

        let parsed_resp = parsed_response(resp.text().await?); //parsed_resp is a Vec<String>, every element ends at "\n"
        println!("{:?}\n", parsed_resp);

        let hash_vec = post_content(parsed_resp);

        // println!("{:?}", hash_vec);

        let mut pack_vector = Vec::new();
        for hash in hash_vec {
            let res = self
                .client
                .post(&post_uri)
                .body(hash.clone())
                .send()
                .await?;
            // println!("{:?}", res.bytes().await?);
            pack_vector.push(res.bytes().await?);
        }

        for pack in pack_vector {
            let byte_slice = pack.as_ref();
            let mut iter = byte_slice.splitn(2, |x| *x == '\n' as u8);
            let _ack = iter.next().expect("ack reply missing");
            let pack_bytes = iter.next().expect("pack reply missing");
            let pack_file = Packfile::new(pack_bytes)?;
            pack_file.parse_pack();
        }

        Ok(())
    }
}

fn parsed_response(resp: String) -> Vec<String> {
    let mut parsed_resp: Vec<String> = Vec::new();
    let mut vec_char = String::new();
    for chars in resp.chars() {
        vec_char.push(chars);
        if chars == '\n' {
            parsed_resp.push(vec_char);
            vec_char = String::new();
        }
    }

    parsed_resp
}

fn post_content(resp: Vec<String>) -> Vec<String> {
    let mut hash_vec = Vec::new();
    for line in resp {
        if line.contains("refs") && !line.contains("HEAD") {
            println!("{}", line);
            let mut split_iter = line.split(" ");
            let hash = &split_iter
                .next()
                .expect("Could not parse ref hash from response")[4..];
            let mut want_string = String::new();
            want_string.push_str("0032want ");
            want_string.push_str(&hash);
            want_string.push_str("\n");
            want_string.push_str("00000009done\n");
            hash_vec.push(want_string);
        }
    }

    hash_vec
}
