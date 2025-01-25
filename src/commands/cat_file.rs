pub fn cat_file(object_name: &str) -> String {
    let decoded_blob = super::utils::decode_blob_as_string(object_name);
    decoded_blob.split("\0").nth(1).unwrap().to_string()
}
