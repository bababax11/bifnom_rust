extern crate bifnom;
use bifnom::parse::{read_dir, rust_result_parse};
use bifnom::structs::*;
use cgmath::prelude::*;
use statistical as st;
use proconio::fastout;

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

    for file in files {
        eprintln!("{}", &file);
        let parsed = rust_result_parse(format!("{}/{}", BASE_PATH, file).as_ref());
        let dists: Vec<_> = parsed
            .iter()
            .map(|(feat, prj, _, _)| sub_distance(feat, prj))
            .collect();
        let qs: Vec<_> = parsed
            .iter()
            .map(|(_, _, q, bq)| (q * bq.invert()).magnitude())
            .collect();
        let dist_sum = stat_summaries(&dists);
        let qs_sum = stat_summaries(&qs);
        println!("{} {:?} {:?}", &file, dist_sum, qs_sum);
    }
}
