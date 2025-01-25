use std::fs;

use super::utils;

pub fn hash_object(filename: &str) -> String {
    // blob <size>\0
    let file_content = fs::read(filename).unwrap();
    let blob_content = get_blob_content(&file_content);
    let compressed_content = utils::compress_content(&blob_content).unwrap();
    let hash = utils::get_hash(&blob_content);
    utils::write_blob(&hash, &compressed_content);
    hash
}

fn get_blob_content(file_content: &[u8]) -> Vec<u8> {
    let mut blob = Vec::<u8>::new();
    blob.extend_from_slice(b"blob ");
    blob.extend(file_content.len().to_string().as_bytes());
    blob.extend(b"\0");
    blob.extend_from_slice(file_content);
    blob
}
