use std::path::Path;
use std::fs::File;
use serde_json::Value;

pub fn load_json_file(file: &str) -> Value {
    let path = Path::new(file);
    let file = File::open(path).expect("file not found");
    serde_json::from_reader(file).expect("unable to parse json in file")
}