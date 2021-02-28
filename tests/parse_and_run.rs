extern crate bifnom;
use bifnom::parse::{neighbor_parse, result_parse, triangles_parse};
use bifnom::projection::run;
use ndarray::prelude::*;
use std::path::Path;

#[test]
fn parse_and_run() {
    let neighbors = neighbor_parse(&Path::new("Input_armadillo/nei/neighbor_points_00000.txt"));
    let tris = triangles_parse(&Path::new("Input_armadillo/tri/triangles_00000.txt"));
    let tris_next = triangles_parse(&Path::new("Input_armadillo/tri/triangles_00001.txt"));
    let feats = result_parse(&Path::new(
        "RIFNOM_TAVE015_TVAR005_TANG025_TDIFF12_TTRACK6/result_RIFNOM_RAD2_ANG10_R5.txt",
    ));
    let (fs, prj, qs, b_qs) = run(&feats[&0], &neighbors, &tris, &tris_next);
    println!("{}", Array::from(fs));
    println!("{}", Array::from(prj));
    println!("{}", Array::from(qs));
    println!("{}", Array::from(b_qs));
}
