extern crate bifnom;
use bifnom::parse::{neighbor_parse, triangles_parse, result_parse};
use bifnom::projection::run;
// use bifnom::structs::DisplayQuaternion;
use std::path::Path;
use ndarray::prelude::*;

fn main() {
    let base = "";
    for i in 0..249-1 {
        let neighbors = neighbor_parse(&Path::new(&format!("Input_armadillo/nei/neighbor_points_{:05}.txt", i)));
        let tris = triangles_parse(&Path::new(&format!("Input_armadillo/tri/triangles_{:05}.txt", i)));
        let tris_next = triangles_parse(&Path::new(&format!("Input_armadillo/tri/triangles_{:05}.txt", i + 1)));
        let feats = result_parse(&Path::new("RIFNOM_TAVE015_TVAR005_TANG025_TDIFF12_TTRACK6/result_RIFNOM_RAD2_ANG10_R5.txt"));
        let (prj, qs) = run(&feats[&i], &neighbors, &tris, &tris_next);
        println!("{:?}", Array::from(prj));
        println!("{:?}",  Array::from(qs));
    }
}
