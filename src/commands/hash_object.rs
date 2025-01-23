use std::fs;
use std::io;
use std::io::Read;

use flate2::{read::ZlibEncoder, Compression};

pub fn hash_object(filename: &str) {
    // blob <size>\0

    //print the hash
    //write the file to the hash
    let file_content = fs::read(filename).unwrap();
    let blob_content = get_blob_content(&file_content);
    let hash = hash_content(&blob_content).unwrap();
    print!("{}", hash);
    write_file(&hash, &blob_content);
}

fn get_blob_content(file_content: &Vec<u8>) -> Vec<u8> {
    let mut blob = Vec::<u8>::new();
    blob.extend_from_slice(b"blob ");
    blob.extend(file_content.len().to_ne_bytes());
    blob.extend_from_slice(&file_content);
    blob
}

fn hash_content(content: &Vec<u8>) -> io::Result<String> {
    let mut z = ZlibEncoder::new(content.as_slice(), Compression::best());
    let mut buffer = Vec::new();
    z.read_to_end(&mut buffer)?;
    Ok(String::from_utf8(buffer).unwrap())
}

fn write_file(hash: &str, blob_content: &Vec<u8>) {
    let sha_directory = &hash[0..2];
    let sha_filename = &hash[2..];
    fs::write(
        format!(".git/objects/{}/{}", sha_directory, sha_filename),
        blob_content,
    )
    .unwrap();
}
