pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct Tree {}

impl Tree {
    const POSSIBLE_MODES: [u32; 5] = [100644, 100755, 120000, 40000, 160000];

    pub fn ls_tree(data: String) -> Result<()> {
        let mut files = Vec::new();

        let mut vec_indices = Vec::new();
        for chars in data.char_indices() {
            vec_indices.push(chars.0);
        }

        //fuck i should probably document this
        let mut i = 0;
        while i + 6 < vec_indices.len() {
            if let Ok(mode) = data[vec_indices[i]..vec_indices[i + 6]].parse::<u32>() {
                if Tree::POSSIBLE_MODES.contains(&mode) {
                    let index = i + 7;
                    if let Some(index_null) = &data[vec_indices[index]..].find('\0') {
                        let filename = &data[vec_indices[index]..vec_indices[index + index_null]];
                        files.push(filename);
                    }
                    i += 7;
                } else {
                    i += 1;
                }
            } else if let Ok(mode) = data[vec_indices[i]..vec_indices[i + 5]].parse::<u32>() {
                let index = i + 6;
                if Tree::POSSIBLE_MODES.contains(&mode) {
                    if let Some(index_null) = &data[vec_indices[index]..].find('\0') {
                        let filename = &data[vec_indices[index]..vec_indices[index + index_null]];
                        files.push(filename);
                    }
                    i += 6;
                } else {
                    i += 1;
                }
            } else {
                i += 1;
            }
        }

        files.sort();
        for file in files {
            println!("{}", file);
        }

        Ok(())
    }
}
