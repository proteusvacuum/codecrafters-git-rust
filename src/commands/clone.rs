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

use bytes::Bytes;
use flate2::read::ZlibDecoder;
use reqwest::header::HeaderMap;

use crate::commands::utils::{materialize_object, Blob};

use super::init;

pub fn clone(repo_url: &str, dir: &str) -> String {
    create_dir(dir);
    init();
    get_packfile(repo_url);
    String::from("")
}

fn create_dir(dir: &str) {
    fs::create_dir(dir).unwrap();
    env::set_current_dir(&dir).unwrap();
}

fn get_packfile(repo_url: &str) {
    let head_ref = discover_head_ref(repo_url).unwrap();
    get_pack(repo_url, &head_ref).unwrap();
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

fn get_pack(repo_url: &str, head_ref: &str) -> Result<String, Box<dyn std::error::Error>> {
    let ref_url = format!("{repo_url}/git-upload-pack");
    // let ref_url = String::from("https://blahblahblah.free.beeceptor.com");
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
    // headers.insert("git-protocol", "version=2".parse().unwrap());

    let mut body: Vec<u8> = Vec::new();
    write!(
        &mut body,
        "004cwant {head_ref} side-bank-64k no-progress\n00000009done\n"
    )
    .unwrap();
    // write!(&mut body, "0011command=fetch0014agent=git/2.43.00016object-format=sha10001000dthin-pack000dofs-delta0032want {head_ref}\n0032want {head_ref}\n0009done\n0000\n").unwrap();
    dbg!(String::from_utf8_lossy(&body.clone()));
    let mut res = client.post(ref_url).body(body).headers(headers).send()?;
    let mut data: Vec<u8> = Vec::new();
    res.read_to_end(&mut data).unwrap();
    let mut start = usize::from_str_radix(std::str::from_utf8(&data[..4]).unwrap(), 16).unwrap();
    dbg!(start);

    // 4-byte signature:
    // The signature is: {'P', 'A', 'C', 'K'}
    dbg!(std::str::from_utf8(&data[start..start + 4]).unwrap()); // PACK
    start += 4;

    // 4-byte version number (network byte order):
    // Git currently accepts version number 2 or 3 but
    //     generates version 2 only.

    let version_number = u32::from_be_bytes(data[start..start + 4].try_into().unwrap());
    // dbg!(version_number); // 0002
    start += 4;

    // 4-byte number of objects contained in the pack (network byte order)
    let num_objects = u32::from_be_bytes(data[start..start + 4].try_into()?) as usize;
    // dbg!(num_objects);
    start += 4;

    for object_num in 0..num_objects {
        let obj_type = (&data[start] & 0b0111_0000) >> 4;
        dbg!(obj_type);

        let mut obj_len = (&data[start] & 0b1111) as usize;
        let mut shift_count = 4;
        while data[start] & 0b1000_0000 != 0 {
            start += 1;
            obj_len = obj_len + (((data[start] & 0b0111_1111) as usize) << shift_count);
            shift_count += 8;
        }
        start += 1;
        // dbg!(obj_len);
        if obj_type == 7 {
            start += 20;
        }
        let mut z = ZlibDecoder::new(&data[start..]);
        let mut buffer = vec![];
        z.read_to_end(&mut buffer).unwrap();
        let total_read = z.total_in() as usize;
        start += total_read;
        if obj_type < 5 {
            let blob = Blob::new(obj_type, &buffer);
            blob.write();
        } else if obj_type == 7 {
            // We don't care about this for now.
        }
    }

    materialize_object(head_ref, ".");

    Ok(String::from("foo"))
}
