use flate2::read::ZlibDecoder;
use flate2::{write::ZlibEncoder, Compression};

use std::fmt::Write as _;
use std::fs;
use std::io::Read;
use std::io::{self, Write as _};

use sha1::{Digest, Sha1};

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

pub fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().fold(String::new(), |mut output, byte| {
        let _ = write!(output, "{byte:02x}");
        output
    })
}

pub fn write_blob(hash: &str, blob_content: &Vec<u8>) {
    let sha_directory = &hash[0..2];
    let sha_filename = &hash[2..];
    fs::create_dir_all(format!(".git/objects/{}", sha_directory)).unwrap();
    fs::write(
        format!(".git/objects/{}/{}", sha_directory, sha_filename),
        blob_content,
    )
    .unwrap_or(()); // This might fail if the file already exists, in which case we just ignore it...
}

pub fn get_hash(content: &Vec<u8>) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    format!("{:x}", hasher.finalize())
}

pub fn compress_content(content: &[u8]) -> io::Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(content)?;
    Ok(encoder.finish().unwrap())
}
