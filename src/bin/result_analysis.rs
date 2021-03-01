extern crate bifnom;
use bifnom::parse::{read_dir, rust_result_parse};
use bifnom::structs::*;
use cgmath::prelude::*;
use proconio::fastout;
use statistical as st;

#[inline]
fn euclid_dist(a: &[f32; 2], b: &[f32; 2]) -> f32 {
    f32::sqrt((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2))
}

#[inline]
fn sub_distance(feat: &Feature, prj: &ProjectedPix) -> f32 {
    let [bx, by] = feat.pos_b;
    euclid_dist(&[bx as f32, by as f32], &prj.xy)
}

fn stat_summaries<T: num_traits::Float>(v: &[T]) -> (T, T, T) {
    let mean = st::mean(v);
    let std = st::population_standard_deviation(v, Some(mean));
    let med = st::median(v);
    (mean, std, med)
}

#[fastout]
fn main() {
    const BASE_PATH: &str = "results";
    let mut files: Vec<_> = read_dir(BASE_PATH).unwrap().collect();
    files.sort();

    for src_path in files {
        eprintln!("{}", &src_path);
        let parsed = rust_result_parse(format!("{}/{}", BASE_PATH, src_path).as_ref());
        let dists_and_qs: Vec<_> = parsed
            .iter()
            .map(|(feat, prj, q, bq)| {
                (
                    sub_distance(feat, prj),
                    (q.normalize() * &bq.invert().normalize()).s.acos() * 2f32,
                )
            })
            .filter(|(d, _)| *d < 5.)
            .collect();
        let dists: Vec<_> = dists_and_qs.iter().map(|(d, _)| *d).collect();
        let qs: Vec<_> = dists_and_qs
            .iter()
            .map(|(_, angle)| *angle)
            .filter(|x| x.is_finite())
            .collect();
        let dist_sum = stat_summaries(&dists);
        let qs_sum = stat_summaries(&qs);
        println!("{} {:?} {:?}", &src_path, dist_sum, qs_sum);
    }
}
