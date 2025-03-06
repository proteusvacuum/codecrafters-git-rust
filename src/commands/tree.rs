use crate::commands::utils::hex_encode;
use std::fmt::Display;

#[derive(Debug)]
pub enum TreeMode {
    Tree,
    Blob,
}

impl From<&[u8]> for TreeMode {
    fn from(value: &[u8]) -> Self {
        if String::from_utf8_lossy(value) == "40000" {
            return TreeMode::Tree;
        }
        TreeMode::Blob
    }
}

impl Display for TreeMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TreeMode::Tree => write!(f, "tree"),
            TreeMode::Blob => write!(f, "blob"),
        }
    }
}

impl TreeMode {
    fn get_code(&self) -> &str {
        match self {
            TreeMode::Tree => "040000",
            TreeMode::Blob => "100644",
        }
    }
}

#[derive(Debug)]
pub struct TreeObject {
    pub mode: TreeMode,
    pub name: String,
    pub sha: String,
    pub total_in: usize,
}

impl Display for TreeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:0>6} {} {}\t{}",
            self.mode.get_code(),
            self.mode,
            self.sha,
            self.name,
        )
    }
}

impl From<&[u8]> for TreeObject {
    fn from(value: &[u8]) -> Self {
        // Iterate over the decoded blob.
        // We can't just split on \0 as the 20_byte_sha might actually contain the null character.

        let mode_start = 0;
        let mut offset = 0;
        // Move to the first space
        while offset < value.len() && value[offset] != b' ' {
            offset += 1;
        }
        let mode = &value[mode_start..offset];
        offset += 1;

        // Move to the first null terminator
        let name_start = offset;
        while offset < value.len() && value[offset] != b'\0' {
            offset += 1;
        }
        let name = &value[name_start..offset];
        offset += 1;

        // Parse the 20-byte SHA
        let sha = if offset + 20 <= value.len() {
            let sha = hex_encode(&value[offset..offset + 20]);
            offset += 20;
            sha
        } else {
            String::new() // Should probably panic?
        };
        Self {
            mode: mode.into(),
            name: String::from_utf8_lossy(name).to_string(),
            sha,
            total_in: offset,
        }
    }
}

pub struct TreeObjects {
    pub objects: Vec<TreeObject>,
}

impl Display for TreeObjects {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for tree_object in &self.objects {
            writeln!(f, "{tree_object}")?
        }
        Ok(())
    }
}

impl From<&[u8]> for TreeObjects {
    fn from(value: &[u8]) -> Self {
        let mut offset: usize = 0;

        // Use an iterator instead of calling `.push` on an empty vector
        let objects = std::iter::from_fn(|| {
            if offset >= value.len() {
                return None;
            }
            let tree_object = TreeObject::from(&value[offset..]);
            offset += tree_object.total_in;
            Some(tree_object)
        })
        .collect();
        TreeObjects { objects }
    }
}
