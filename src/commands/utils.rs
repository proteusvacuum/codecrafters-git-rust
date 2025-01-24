use flate2::read::ZlibDecoder;
use std::fs;
use std::io;
use std::io::Read;

pub fn decode_blob(object_name: &str) -> String {
    let sha_directory = &object_name[0..2];
    let sha_filename = &object_name[2..];
    let compressed_blob =
        fs::read(format!(".git/objects/{}/{}", sha_directory, sha_filename)).unwrap();
    dbg!(&compressed_blob);
    decode_reader(compressed_blob).unwrap()
}

fn decode_reader(bytes: Vec<u8>) -> io::Result<String> {
    let mut z = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    z.read_to_string(&mut s)?;
    Ok(s)
}
