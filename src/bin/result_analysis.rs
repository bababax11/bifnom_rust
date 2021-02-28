extern crate bifnom;
use bifnom::parse::read_dir;
use bifnom::structs::*;

fn main() {
    const BASE_PATH: &str = "results";
    let mut files: Vec<_> = read_dir(BASE_PATH).unwrap().collect();
    files.sort();
    dbg!(files);
}
