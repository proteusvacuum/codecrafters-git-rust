use clap::builder::TryMapValueParser;
use flate2::read::ZlibDecoder;
use flate2::{write::ZlibEncoder, Compression};

use std::fmt::{Display, Write as _};
use std::fs::{self, create_dir};
use std::io::Read;
use std::io::{self, Write as _};

use sha1::{Digest, Sha1};

use crate::commands::cat_file;
use crate::commands::ls_tree::{self, tree_objects_from_blob};

pub fn decode_blob_as_string(object_name: &str) -> String {
    let sha_directory = &object_name[0..2];
    let sha_filename = &object_name[2..];
    let compressed_blob = fs::read(format!("./.git/objects/{}/{}", sha_directory, sha_filename))
        .expect(format!("{object_name} not found").as_str());
    let mut z = ZlibDecoder::new(&compressed_blob[..]);
    let mut s = String::new();
    z.read_to_string(&mut s).unwrap();
    s
}

pub fn decode_blob_as_bytes(object_name: &str) -> Vec<u8> {
    let sha_directory = &object_name[0..2];
    let sha_filename = &object_name[2..];
    let compressed_blob = fs::read(format!(".git/objects/{}/{}", sha_directory, sha_filename))
        .expect(format!("Couldn't find {object_name}").as_str());
    let mut z = ZlibDecoder::new(&compressed_blob[..]);
    let mut buffer = vec![];
    z.read_to_end(&mut buffer).unwrap();
    buffer
}

pub fn decode_bytes_to_string(compressed_blob: &[u8]) -> String {
    let mut z = ZlibDecoder::new(&compressed_blob[..]);
    let mut s = String::new();
    z.read_to_string(&mut s).unwrap();
    s
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

#[derive(Debug, PartialEq, Eq)]
pub enum BlobType {
    Commit,
    Tree,
    Blob,
}

impl From<u8> for BlobType {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Commit,
            2 => Self::Tree,
            3 => Self::Blob,
            _ => panic!(),
        }
    }
}

impl From<char> for BlobType {
    fn from(value: char) -> Self {
        match value {
            'c' => Self::Commit,
            't' => Self::Tree,
            'b' => Self::Blob,
            _ => panic!(),
        }
    }
}

impl Display for BlobType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Blob {
    blob_type: BlobType,
    blob: Vec<u8>,
}

impl Blob {
    pub fn new(blob_type: u8, blob: &Vec<u8>) -> Self {
        Self {
            blob_type: blob_type.into(),
            blob: blob.to_vec(),
        }
    }

    pub fn new_with_blob_type(blob_type: BlobType, blob: &[u8]) -> Self {
        Self {
            blob_type,
            blob: blob.to_vec(),
        }
    }

    pub fn write(&self) {
        let mut content: Vec<u8> = format!("{} {}\0", self.blob_type, self.blob.len())
            .into_bytes()
            .to_vec();
        content.extend(self.blob.clone());
        let hash = get_hash(&content);
        dbg!(&hash);
        write_blob(&hash, &compress_content(&content).unwrap());
    }

    pub fn materialize(&self, current_dir: &str) {
        match self.blob_type {
            BlobType::Commit => {
                let hash = std::str::from_utf8(&self.blob[5..45]).unwrap();
                materialize_object(hash, current_dir);
            }
            BlobType::Tree => {
                // read the tree
                let tree_objects = tree_objects_from_blob(&self.blob);
                for tree_object in tree_objects {
                    match tree_object.mode.chars().nth(0).unwrap() {
                        '1' => {
                            let path = format!("{}/{}", current_dir, tree_object.name);
                            let content = cat_file(&tree_object.sha);
                            fs::write(&path, content).unwrap();
                        }
                        '4' => {
                            let new_dir = format!("{}/{}", current_dir, tree_object.name);
                            create_dir(&new_dir).unwrap();
                            materialize_object(&tree_object.sha, &new_dir);
                        }
                        _ => {
                            todo!();
                        }
                    }
                }
            }
            BlobType::Blob => todo!(),
        }
    }
}

pub fn materialize_object(hash: &str, current_dir: &str) {
    let bytes = decode_blob_as_bytes(hash);
    let blob_type: BlobType = (bytes[0] as char).into();
    let start = bytes
        .iter()
        .position(|b| *b == b'\0')
        .expect("No null character found!");

    let blob = Blob::new_with_blob_type(blob_type, &bytes[start + 1..]);
    blob.materialize(current_dir)
}
