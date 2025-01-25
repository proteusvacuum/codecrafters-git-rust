use super::utils;
use chrono;
use std::io::Write;

pub fn commit_tree(tree_sha: &str, parent_sha: &str, message: &str) -> String {
    let mut content: Vec<u8> = Vec::new();
    let now = chrono::Local::now();
    let date_secs = now.timestamp();
    let timezone = now.format("%z").to_string();

    write!(&mut content, "tree {tree_sha}\nparent {parent_sha}\nauthor Author <name@example.com> {date_secs} {timezone}\ncommitter Committer <name@example.com> {date_secs} {timezone}\n\n{message}\n").unwrap();
    let blob_content = get_blob_content(&content);
    let compressed_content = utils::compress_content(&blob_content).unwrap();
    let hash = utils::get_hash(&blob_content);
    utils::write_blob(&hash, &compressed_content);

    hash
}

fn get_blob_content(file_content: &[u8]) -> Vec<u8> {
    let mut blob = Vec::<u8>::new();
    blob.extend_from_slice(b"commit ");
    blob.extend(file_content.len().to_string().as_bytes());
    blob.extend(b"\0");
    blob.extend_from_slice(file_content);
    blob
}

// commit {size}\0{content}
// tree {tree_sha}
// parent {parent1_sha}
// parent {parent2_sha}
// author {author_name} <{author_email}> {author_date_seconds} {author_date_timezone}
// committer {committer_name} <{committer_email}> {committer_date_seconds} {committer_date_timezone}

// {commit message}

// commit 252\0tree 1d637e1670f4270c7657f123100e8828bde1c68f\nparent 873d0dbcf70fa7f484bdc7057d0f34c027dff965\nauthor Farid Rener <proteusvacuum@gmail.com> 1737837898 -0500\ncommitter Farid Rener <proteusvacuum@gmail.com> 1737837898 -0500\n\ncodecrafters submit [skip ci]\n"
