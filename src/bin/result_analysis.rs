extern crate bifnom;
use bifnom::parse::{read_dir, rust_result_parse};
use bifnom::structs::*;

fn main() {
    const BASE_PATH: &str = "results";
    let mut files: Vec<_> = read_dir(BASE_PATH).unwrap().collect();
    files.sort();
    
    for file in files {
        eprintln!("{}", &file);
        let parsed = rust_result_parse(file.as_ref());
        
    }
    
}
