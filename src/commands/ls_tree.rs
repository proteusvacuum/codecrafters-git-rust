use super::{tree::TreeObjects, utils::decode_blob_as_bytes};
use std::fmt::Write;

pub fn ls_tree(name_only: &bool, object_name: &str) -> String {
    let decoded_blob = decode_blob_as_bytes(object_name);
    // Decoded blob is a string that looks like:
    //  tree <size>\0<mode> <name>\0<20_byte_sha><mode> <name>\0<20_byte_sha>

    // discard tree <size>
    let mut offset: usize = 0;
    while offset < decoded_blob.len() && decoded_blob[offset] != b'\0' {
        offset += 1;
    }
    offset += 1;
    let tree_objects = TreeObjects::from(&decoded_blob[offset..]);
    if *name_only {
        tree_objects
            .objects
            .iter()
            .fold(String::new(), |mut output, tree_object| {
                let _ = writeln!(output, "{}", tree_object.name);
                output
            })
    } else {
        format!("{tree_objects}")
    }
}
