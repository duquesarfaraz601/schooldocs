use std::fs;
use std::path::Path;

fn main() {
    let path = Path::new("example.txt");
    let mut file = fs::File::create(path).unwrap();
    file.write_all(b"Hello, world!").unwrap();
}
