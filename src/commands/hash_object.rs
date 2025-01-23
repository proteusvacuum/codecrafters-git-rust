use std::fs;
use std::io;
use std::io::Write;

use flate2::{write::ZlibEncoder, Compression};
use sha1::{Digest, Sha1};

pub fn hash_object(filename: &str) {
    // blob <size>\0

    //print the hash
    //write the file to the hash
    let file_content = fs::read(filename).unwrap();
    let blob_content = get_blob_content(&file_content);
    let compressed_content = compress_content(&blob_content).unwrap();
    let hash = get_hash(&blob_content);
    write_file(&hash, &compressed_content);
    println!("{}", hash)
}

fn get_blob_content(file_content: &Vec<u8>) -> Vec<u8> {
    let mut blob = Vec::<u8>::new();
    blob.extend_from_slice(b"blob ");
    blob.extend(file_content.len().to_string().as_bytes());
    blob.extend(b"\0");
    blob.extend_from_slice(&file_content);
    blob
}

fn compress_content(content: &Vec<u8>) -> io::Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(content)?;
    Ok(encoder.finish().unwrap())
}

fn get_hash(content: &Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    format!("{:x}", hasher.finalize())
}

fn write_file(hash: &str, blob_content: &Vec<u8>) {
    let sha_directory = &hash[0..2];
    let sha_filename = &hash[2..];
    fs::create_dir_all(format!(".git/objects/{}", sha_directory)).unwrap();
    fs::write(
        format!(".git/objects/{}/{}", sha_directory, sha_filename),
        blob_content,
    )
    .unwrap();
}
