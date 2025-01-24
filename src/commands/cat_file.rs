pub fn cat_file(object_name: &str) {
    let decoded_blob = super::utils::decode_blob_as_string(object_name);
    print!("{}", get_blob_contents(&decoded_blob))
}

fn get_blob_contents(blob: &str) -> String {
    blob.split("\0").nth(1).unwrap().to_string()
}
