extern crate bifnom;
use bifnom::parse::{neighbor_parse, result_parse, triangles_parse};
use bifnom::projection::run;
use bifnom::structs::*;
use itertools::izip;
use std::io::prelude::*;
use std::path::Path;
use std::{fs, io};

fn read_dir<P: AsRef<Path>>(path: P) -> io::Result<impl Iterator<Item = String>> {
    Ok(fs::read_dir(path)?.filter_map(|entry| {
        let entry = entry.ok()?;
        if entry.file_type().ok()?.is_file() {
            Some(entry.file_name().to_string_lossy().into_owned())
        } else {
            None
        }
    }))
}

fn format_vec_strs(prj: &[ProjectedPix], qs: &[Quaternion], b_qs: &[Quaternion], none_count: usize, all_count: usize) -> String {
    let mut s = format!("none_count:{}/{}\n", none_count, all_count);
    assert_eq!(prj.len(), qs.len());
    assert_eq!(qs.len(), b_qs.len());
    for (p, q, b) in izip!(prj, qs, b_qs) {
        s += &format!("{}; {}; {}\n", &p, &q, &b);
    }
    s
}

fn main() {
    const N: usize = 249 - 1;
    const BASE_PATH: &str = "RIFNOM_TAVE015_TVAR005_TANG025_TDIFF12_TTRACK6";
    let mut dirs: Vec<_> = read_dir(BASE_PATH)
        .unwrap()
        .filter(|x| x.starts_with("result"))
        .collect();
    dirs.sort();
    let mut tris = triangles_parse(format!("Input_armadillo/tri/triangles_{:05}.txt", 0).as_ref());

    for dir in dirs {
        eprintln!("{}", &dir);
        let feats = result_parse(format!("{}/{}", BASE_PATH, dir).as_ref());

        use prgrs::Prgrs;
        for i in Prgrs::new(0..N, N) {
            let result_path_str = format!("results/{}_{}", BASE_PATH, i);
            let mut file = fs::File::create(result_path_str).unwrap();

            let neighbors = neighbor_parse(
                format!("Input_armadillo/nei/neighbor_points_{:05}.txt", i).as_ref(),
            );
            let tris_next =
                triangles_parse(format!("Input_armadillo/tri/triangles_{:05}.txt", i + 1).as_ref());
            let (prj, qs, b_qs, none_count) = run(&feats[&i], &neighbors, &tris, &tris_next);

            let results_str = format_vec_strs(&prj, &qs, &b_qs, none_count, feats.len());
            write!(file, "{}\n", results_str).unwrap();

            tris = tris_next;
        }
    }
}
