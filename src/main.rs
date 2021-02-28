extern crate bifnom;
use bifnom::parse::{neighbor_parse, read_dir, result_parse, triangles_parse};
use bifnom::projection::run;
use bifnom::structs::*;
use itertools::izip;
use std::fs;
use std::io::prelude::*;
use cgmath::Quaternion;

fn format_vec_strs(
    fs: &[&Feature],
    prj: &[ProjectedPix],
    qs: &[Quaternion<f32>],
    b_qs: &[Quaternion<f32>],
    all_count: usize,
) -> String {
    let none_count = all_count - prj.len();
    let mut s = format!("none_count:{}/{}\n", none_count, all_count);
    assert_eq!(prj.len(), qs.len());
    assert_eq!(qs.len(), b_qs.len());
    for (f, p, q, b) in izip!(fs, prj, qs, b_qs) {
        s += &format!("{}; {}; {}; {}\n", &f, &p, DisplayQuaternion::from(q), DisplayQuaternion::from(b));
    }
    s
}

fn main() {
    const N: usize = 249 - 1;
    const BASE_PATH: &str = "RIFNOM_TAVE015_TVAR005_TANG025_TDIFF12_TTRACK6";
    let mut files: Vec<_> = read_dir(BASE_PATH)
        .unwrap()
        .filter(|x| x.starts_with("result"))
        .collect();
    files.sort();
    let mut tris = triangles_parse(format!("Input_armadillo/tri/triangles_{:05}.txt", 0).as_ref());

    for file in files {
        eprintln!("{}", &file);
        let feats = result_parse(format!("{}/{}", BASE_PATH, file).as_ref());

        use prgrs::Prgrs;
        for i in Prgrs::new(0..N, N) {
            let result_path_str = format!("results/{}_{}_{:03}", BASE_PATH, file, i);
            let mut file = fs::File::create(result_path_str).unwrap();

            let neighbors = neighbor_parse(
                format!("Input_armadillo/nei/neighbor_points_{:05}.txt", i).as_ref(),
            );
            let tris_next =
                triangles_parse(format!("Input_armadillo/tri/triangles_{:05}.txt", i + 1).as_ref());
            let (fs, prj, qs, b_qs) = run(&feats[&i], &neighbors, &tris, &tris_next);

            let results_str = format_vec_strs(&fs, &prj, &qs, &b_qs, feats[&i].len());
            write!(file, "{}\n", results_str).unwrap();

            tris = tris_next;
        }
    }
}
