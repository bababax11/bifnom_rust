extern crate bifnom;
use bifnom::parse::{read_dir, rust_result_parse};
use bifnom::structs::*;

#[inline]
fn euclid_dist(a: &[f32; 2], b: &[f32; 2]) -> f32 {
    f32::sqrt((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2))
}

#[inline]
fn sub_distance(feat: &Feature, prj: &ProjectedPix) -> f32 {
    let [bx, by] = feat.pos_b;
    euclid_dist(&[bx as f32, by as f32], &prj.xy)
}

fn main() {
    const BASE_PATH: &str = "results";
    let mut files: Vec<_> = read_dir(BASE_PATH).unwrap().collect();
    files.sort();
    
    for file in files {
        eprintln!("{}", &file);
        let parsed = rust_result_parse(file.as_ref());
        parsed.iter().map(|(feat, prj, q, bq)| {
            let dist = sub_distance(feat, prj);

            (dist)
        });
        
    }
    
}
