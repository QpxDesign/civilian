use std::fs;
use std::io::Read;

pub fn file_to_string(path: &str) -> String {
    let mut file = fs::File::open(path).unwrap();
    let mut contents: String = "".to_string();
    let _ = file.read_to_string(&mut contents);
    return contents;
}
