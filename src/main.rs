extern crate bifnom;
use bifnom::parse::{neighbor_parse, result_parse, triangles_parse};
use bifnom::projection::run;
use bifnom::structs::*;
use itertools::izip;
use std::io::prelude::*;
use std::path::Path;
use std::{fs, io};

fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<Vec<String>> {
    Ok(fs::read_dir(path)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_type().ok()?.is_file() {
                Some(entry.file_name().to_string_lossy().into_owned())
            } else {
                None
            }
        })
        .collect())
}

fn format_vec_strs(prj: &[ProjectedPix], qs: &[Quaternion], b_qs: &[Quaternion]) -> String {
    let mut s = String::new();
    assert_eq!(prj.len(), qs.len());
    assert_eq!(qs.len(), b_qs.len());
    for (p, q, b) in izip!(prj, qs, b_qs) {
        s += &format!("{}; {}; {}", &p, &q, &b);
    }
    s
}

fn main() {
    const N: usize = 249 - 1;
    const BASE_PATH: &str = "RIFNOM_TAVE015_TVAR005_TANG025_TDIFF12_TTRACK6";
    let dirs = read_dir(BASE_PATH).unwrap();
    let mut tris = triangles_parse(concat!("Input_armadillo/tri/triangles_{:05}.txt", 0).as_ref());

    for dir in dirs {
        let feats = result_parse(format!("{}/{}", BASE_PATH, dir).as_ref());

        use prgrs::Prgrs;
        for i in Prgrs::new(0..N, N) {
            let result_path_str = format!("results/{}_{}", BASE_PATH, i);
            let mut file = fs::File::create(result_path_str).unwrap();

            let neighbors =
                neighbor_parse(format!("Input_armadillo/tri/triangles_{:05}.txt", i).as_ref());
            let tris_next =
                triangles_parse(format!("Input_armadillo/tri/triangles_{:05}.txt", i + 1).as_ref());
            let (prj, qs) = run(&feats[&i], &neighbors, &tris, &tris_next);
            let b_qs: Vec<_> = feats[&i].iter().map(|x| x.rotate_q()).collect();

            let results_str = format_vec_strs(&prj, &qs, &b_qs);
            writeln!(file, "{}", results_str);

            tris = tris_next;
        }
    }
}
