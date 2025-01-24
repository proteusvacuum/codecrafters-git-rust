use flate2::read::ZlibDecoder;
use std::fs;
use std::io::Read;

pub fn decode_blob_as_string(object_name: &str) -> String {
    let sha_directory = &object_name[0..2];
    let sha_filename = &object_name[2..];
    let compressed_blob =
        fs::read(format!(".git/objects/{}/{}", sha_directory, sha_filename)).unwrap();
    let mut z = ZlibDecoder::new(&compressed_blob[..]);
    let mut s = String::new();
    z.read_to_string(&mut s).unwrap();
    s
}

pub fn decode_blob_as_bytes(object_name: &str) -> Vec<u8> {
    let sha_directory = &object_name[0..2];
    let sha_filename = &object_name[2..];
    let compressed_blob =
        fs::read(format!(".git/objects/{}/{}", sha_directory, sha_filename)).unwrap();
    let mut z = ZlibDecoder::new(&compressed_blob[..]);
    let mut buffer = vec![];
    z.read_to_end(&mut buffer).unwrap();
    buffer
}
