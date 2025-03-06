// make directory
// get packfile (only sideband 1)
// parse packfile
// check out latest commit

// The git server encodes the response in packet-line format. The response consists of multiple sidebands (or streams), described in gitprotocol-pack.txt. For this challenge, you only need to deal with sideband 1, the packfile data, and ignores other sidebands like progress information.

//     You'll need to parse the packfile data and unpack the blobs/commits/trees to .git/objects. You might want to reuse the plumbing commands you've built. The official git clone doesn't unpack. Instead, it keeps the packfile as .git/objects/pack/pack-{sha}.pack and uses an index file .git/objects/pack/pack-{sha}.idx to store the objects. For this challenge, it'd be easer if you unpack the objects. Otherwise, you'd need to figure out how to generate the index for the packfile.
//     After unpacking, you'll need to check out the latest commit to the working directory. From the commit, you'll find the tree. From the tree, you'd find more subtrees and blobs. You'll need to check them all out recursively.

use std::env;
use std::fs;
use std::io::{Read, Write};

use flate2::read::ZlibDecoder;
use reqwest::header::HeaderMap;

use crate::commands::utils::{materialize_object, Blob, ObjectType};

use super::init;

pub fn clone(repo_url: &str, dir: &str) {
    create_dir(dir);
    init();
    get_packfile(repo_url);
}

fn create_dir(dir: &str) {
    fs::create_dir(dir).unwrap();
    env::set_current_dir(dir).unwrap();
}

fn get_packfile(repo_url: &str) {
    let head_ref = discover_head_ref(repo_url).expect("An error occurred while fetching the head");
    let data = get_pack(repo_url, &head_ref).expect("An error occurred while getting the pack");
    make_data_from_pack(data, &head_ref).unwrap();
}

fn discover_head_ref(repo_url: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Use git protocol version 1 (i.e. without headers), as it is simpler.
    let ref_url = format!("{repo_url}/info/refs?service=git-upload-pack");
    let body = reqwest::blocking::get(ref_url)?.text()?;
    let mut head_ref = String::new();
    for line in body.split("\n") {
        if let Some(head) = line.find("HEAD") {
            head_ref = line[head - 41..head - 1].to_string();
            break;
        }
    }
    Ok(head_ref)
}

fn get_pack(repo_url: &str, head_ref: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let ref_url = format!("{repo_url}/git-upload-pack");
    let client = reqwest::blocking::Client::builder()
        // .proxy(reqwest::Proxy::https("http://localhost:8080")?)
        // .danger_accept_invalid_certs(true)
        .build()?;
    let mut headers = HeaderMap::new();
    headers.insert(
        "content-type",
        "application/x-git-upload-pack-request".parse().unwrap(),
    );
    headers.insert(
        "accept",
        "application/x-git-upload-pack-result".parse().unwrap(),
    );

    let mut body: Vec<u8> = Vec::new();
    write!(
        &mut body,
        "004cwant {head_ref} side-bank-64k no-progress\n00000009done\n"
    )
    .unwrap();
    let mut res = client.post(ref_url).body(body).headers(headers).send()?;
    let mut data: Vec<u8> = Vec::new();
    res.read_to_end(&mut data)?;

    Ok(data)
}

fn make_data_from_pack(data: Vec<u8>, head_ref: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut start = usize::from_str_radix(std::str::from_utf8(&data[..4]).unwrap(), 16).unwrap();
    dbg!(start);

    // 4-byte signature:
    // The signature is: {'P', 'A', 'C', 'K'}
    assert!(std::str::from_utf8(&data[start..start + 4])
        .unwrap()
        .eq("PACK")); // PACK
    start += 4;

    // 4-byte version number (network byte order):
    // Git currently accepts version number 2 or 3 but
    //     generates version 2 only.
    let version_number = u32::from_be_bytes(data[start..start + 4].try_into().unwrap());
    assert!(version_number == 2); // 0002
    start += 4;

    // 4-byte number of objects contained in the pack (network byte order)
    let num_objects = u32::from_be_bytes(data[start..start + 4].try_into()?) as usize;
    // dbg!(num_objects);
    start += 4;
    for _ in 0..num_objects {
        start += make_blob_from_data(&data[start..]);
    }
    materialize_object(head_ref, ".");

    Ok(())
}

fn make_blob_from_data(data: &[u8]) -> usize {
    let mut offset = 0;
    let obj_type: ObjectType = data[offset].into();
    while data[offset] & 0b1000_0000 != 0 {
        offset += 1;
    }
    offset += 1;
    if obj_type == ObjectType::RefDelta {
        offset += 20;
    }
    let mut z = ZlibDecoder::new(&data[offset..]);
    let mut buffer = vec![];
    z.read_to_end(&mut buffer).unwrap();
    let total_read = z.total_in() as usize;
    offset += total_read;
    match obj_type {
        ObjectType::Commit | ObjectType::Tree | ObjectType::Blob | ObjectType::Tag => {
            let blob = Blob::new(obj_type.clone(), &buffer);
            blob.write();
            offset
        }
        ObjectType::OfsDelta | ObjectType::RefDelta => {
            // Do nothing, this is annoying to implement.
            println!("This git implementation does not support deltas...");
            0
        }
    }
}
