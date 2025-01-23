use flate2::read::ZlibDecoder;
use std::fs;
use std::io;
use std::io::Read;

pub fn cat_file(blob_sha: &str) {
    let sha_directory = &blob_sha[0..2];
    let sha_filename = &blob_sha[2..];
    let compressed_blob =
        fs::read(format!(".git/objects/{}/{}", sha_directory, sha_filename)).unwrap();
    let decoded_blob = decode_reader(compressed_blob).unwrap();
    print!("{}", get_blob_contents(&decoded_blob))
}

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
    let mut z = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    z.read_to_string(&mut s)?;
    Ok(s)
}

fn get_blob_contents(blob: &str) -> String {
    blob.split("\0").nth(1).unwrap().to_string()
}
