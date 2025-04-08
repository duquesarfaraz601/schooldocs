use std::fs::File;
use std::io::{self, BufRead};

fn main() -> io::Result<()> {
    let filename = "example.txt";

    if let Err(e) = File::open(filename)? {
        eprintln!("无法打开文件: {}", filename);
        return Err(io::Error::new(io::ErrorKind::Other, e.to_string()));
    }

    let contents = String::from_utf8_lossy(&File::open(filename).unwrap())?;

    println!("{}", contents);

    Ok(())
}
