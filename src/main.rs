extern crate bifnom;
use bifnom::parse::{neighbor_parse, triangles_parse, result_parse};
use bifnom::projection::run;
use std::path::Path;

fn main() {
    let base = "";
    for i in 0..249-1 {
        let neighbors = neighbor_parse(&Path::new(&format!("Input_armadillo/nei/neighbor_points_{:05}.txt", i)));
        let tris = triangles_parse(&Path::new(&format!("Input_armadillo/tri/triangles_{:05}.txt", i)));
        let tris_next = triangles_parse(&Path::new(&format!("Input_armadillo/tri/triangles_{:05}.txt", i + 1)));
        let feats = result_parse(&Path::new(todo!()));
        let (prj, qs) = run(&feats[&i], &neighbors, &tris, &tris_next);

    }
}
