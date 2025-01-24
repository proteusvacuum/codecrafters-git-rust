use crate::commands::utils::decode_blob;

pub fn ls_tree(name_only: &bool, object_name: &str) {
    let decoded_blob = decode_blob(object_name);
    //  tree <size>\0
    // <mode> <name>\0<20_byte_sha>
    // <mode> <name>\0<20_byte_sha>
    dbg!(decoded_blob);
}
// We recommend implementing the full ls-tree output too since that'll require that you parse all data in the tree object, not just filenames.
