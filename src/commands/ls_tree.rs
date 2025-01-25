use super::utils::decode_blob_as_bytes;
use super::utils::hex_encode;

use std::fmt::Write;

#[derive(Debug)]
pub struct TreeObject {
    pub mode: String,
    pub name: String,
    pub sha: String,
}

pub fn ls_tree(name_only: &bool, object_name: &str) -> String {
    let decoded_blob = decode_blob_as_bytes(object_name);
    // Decoded blob is a string that looks like:
    //  tree <size>\0<mode> <name>\0<20_byte_sha><mode> <name>\0<20_byte_sha>

    let mut tree_objects: Vec<TreeObject> = Vec::new();

    // Iterate over the decoded blob.
    // We can't just split on \0 as the 20_byte_sha might actually contain the null character.
    let mut offset: usize = 0;

    // discard tree <size>
    while offset < decoded_blob.len() && decoded_blob[offset] != b'\0' {
        offset += 1;
    }
    offset += 1;

    while offset < decoded_blob.len() {
        let mode_start = offset;
        // Move to the first space
        while offset < decoded_blob.len() && decoded_blob[offset] != b' ' {
            offset += 1;
        }
        let mode = &decoded_blob[mode_start..offset];
        offset += 1;

        // Move to the first null terminator
        let name_start = offset;
        while offset < decoded_blob.len() && decoded_blob[offset] != b'\0' {
            offset += 1;
        }
        let name = &decoded_blob[name_start..offset];
        offset += 1;

        // Parse the 20-byte SHA
        let sha = if offset + 20 <= decoded_blob.len() {
            let sha = hex_encode(&decoded_blob[offset..offset + 20]);
            offset += 20;
            sha
        } else {
            String::new() // Should probably panic?
        };

        tree_objects.push(TreeObject {
            mode: String::from_utf8_lossy(mode).to_string(),
            name: String::from_utf8_lossy(name).to_string(),
            sha,
        })
    }
    let mut output = String::new();
    for tree_object in tree_objects {
        if *name_only {
            writeln!(&mut output, "{}", tree_object.name).unwrap();
        } else {
            writeln!(
                &mut output,
                "{:0>6} {} {}\t{}",
                tree_object.mode,
                if tree_object.mode == "40000" {
                    "tree"
                } else {
                    "blob"
                },
                tree_object.sha,
                tree_object.name,
            )
            .unwrap();
        }
    }
    output
}
