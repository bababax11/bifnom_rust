use crate::consts::*;
use crate::structs::*;
use cgmath::{prelude::*, Matrix3, Quaternion};
use std::collections::HashMap;

/// P(x, y), A(x0, y0), B(x1, y1), C(x2, y2) として
/// AP = sAB + tACとなるような(s, t)を返す 
fn triangle_coo(xy: &[f32; 2], xy0: &[f32; 2], xy1: &[f32; 2], xy2: &[f32; 2]) -> FeatCoo {
    let [x, y] = *xy;
    let [x0, y0] = *xy0;
    let [x1, y1] = *xy1;
    let [x2, y2] = *xy2;
    let x = x - x0;
    let y = y - y0;
    let x1 = x1 - x0;
    let y1 = y1 - y0;
    let x2 = x2 - x0;
    let y2 = y2 - y0;

    let area2_inv = 1. / (x1 * y2 - y1 * x2 + EPS);
    let s = area2_inv * (y2 * x - x2 * y);
    let t = area2_inv * (-y1 * x + x1 * y);
    FeatCoo { st: [s, t] }
}

#[inline]
fn coo(feat: &Feature, tri: &TriangleCoo) -> FeatCoo {
    let [pos_ax, pos_ay] = feat.pos_a;
    triangle_coo(
        &[pos_ax as f32, pos_ay as f32],
        &tri.prj_a,
        &tri.prj_b,
        &tri.prj_c,
    )
}

fn cal_px_from_st(f_coo: &FeatCoo, tri: &TriangleCoo) -> ProjectedPix {
    let [s, t] = f_coo.st;
    let x = (1. - s - t) * tri.prj_a[0] + s * tri.prj_b[0] + t * tri.prj_c[0];
    let y = (1. - s - t) * tri.prj_a[1] + s * tri.prj_b[1] + t * tri.prj_c[1];
    ProjectedPix { xy: [x, y] }
}

fn rotate_mat(before: &Triangle, after: &Triangle) -> Option<Quaternion<f32>> {
    let bf_arr = Matrix3::from([before.a.clone(), before.b.clone(), before.c.clone()]).transpose();
    let af_arr = Matrix3::from([after.a.clone(), after.b.clone(), after.c.clone()]).transpose();
    Some((bf_arr * af_arr.invert()?).into())
}

pub fn run<'a>(
    feats: &'a Vec<Feature>,
    neighbor: &HashMap<[usize; 2], usize>,
    triangles: &Vec<Triangle>,
    triangles_next: &Vec<Triangle>, // 1個未来の三角形
) -> (
    Vec<&'a Feature>,
    Vec<ProjectedPix>,
    Vec<Quaternion<f32>>,
    Vec<Quaternion<f32>>,
) {
    let tri_coos: Vec<_> = triangles_next
        .iter()
        .map(|tri| TriangleCoo::from(tri))
        .collect();
    let mut fs = vec![];
    let mut pxs = vec![];
    let mut qs = vec![];
    let mut b_qs = vec![];
    for feat in feats {
        if let Some(&place) = neighbor.get(&feat.pos_a) {
            let tri = &triangles[place];
            let tri_next = &triangles_next[place];
            let _q = rotate_mat(tri, tri_next);
            if let Some(q) = _q {
                fs.push(feat);
                qs.push(q);

                assert_eq!(tri_next.t, place);
                let tri_coo = &tri_coos[place];
                let f_coo = coo(feat, tri_coo);
                let px = cal_px_from_st(&f_coo, tri_coo);
                pxs.push(px);

                b_qs.push(feat.rotate_q().unwrap());
            }
        }
    }
    (fs, pxs, qs, b_qs)
}
