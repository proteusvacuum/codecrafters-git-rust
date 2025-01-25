use std::fs;
use std::io::{self, Write as _};
use std::path::Path;

use super::utils;

#[derive(Debug, Clone)]
pub struct TreeObject {
    pub mode: String,
    pub name: String,
    pub sha: String,
}

pub fn write_tree() -> String {
    // Iterate over the files/directories in the working directory
    //     If the entry is a file, create a blob object and record its SHA hash (`hash_object`)
    //     If the entry is a directory, recursively create a tree object and record its SHA hash
    //     Once you have all the entries and their SHA hashes, write the tree object to the .git/objects directory

    fn visit_dirs(dir: &Path) -> io::Result<String> {
        let mut tree_objects: Vec<TreeObject> = Vec::new();
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.starts_with("./.git") {
                    continue;
                }
                let mode = {
                    if path.is_dir() {
                        "40000".to_string()
                    } else {
                        "100644".to_string()
                    }
                };
                if path.is_dir() {
                    tree_objects.push(TreeObject {
                        mode,
                        name: path.file_name().unwrap().to_string_lossy().to_string(),
                        sha: visit_dirs(&path)?,
                    });
                } else {
                    tree_objects.push(TreeObject {
                        mode,
                        name: path.file_name().unwrap().to_string_lossy().to_string(),
                        sha: super::hash_object(path.to_str().unwrap()),
                    });
                }
            }
        }
        let hash = write_tree_objects(&tree_objects);
        Ok(hash)
    }

    visit_dirs(Path::new(".")).unwrap()
}

fn write_tree_objects(tree: &[TreeObject]) -> String {
    let mut files: Vec<u8> = Vec::new();
    let mut sorted_tree_objects = tree.to_vec();
    sorted_tree_objects.sort_by(|a, b| a.name.cmp(&b.name));
    for item in sorted_tree_objects {
        write!(&mut files, "{} {}\0", item.mode, item.name).unwrap();
        let mut sha_bytes = [0u8; 20];
        hex::decode_to_slice(&item.sha, &mut sha_bytes).unwrap();
        files.extend_from_slice(&sha_bytes)
        //list each file with <mode> <name>\0<20_byte_sha>
    }
    let mut blob: Vec<u8> = Vec::new();
    write!(&mut blob, "tree {}\0", files.len()).unwrap();
    blob.append(&mut files);
    let compressed_tree = utils::compress_content(&blob).unwrap();
    let hash = utils::get_hash(&blob);
    utils::write_blob(&hash, &compressed_tree);
    hash
}
