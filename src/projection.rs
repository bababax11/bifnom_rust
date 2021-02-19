use crate::consts::*;
use crate::structs::*;
use ndarray::prelude::*;
use std::collections::HashMap;

pub fn prj2(xyz: [f32; 3]) -> [f32; 2] {
    let [x, y, z] = xyz;
    let a = array![x, y, z, 1.0];
    let a = INV_V.dot(&a);
    [
        (a[0] / (-a[2] + EPS) * *PRJ00 + 1.) * WIDTH as f32,
        (a[1] / (a[2] + EPS) * *PRJ11 + 1.) * HEIGHT as f32,
    ]
}

pub fn run(
    results: &Vec<Feature>,
    neighbor: &HashMap<[usize; 2], usize>,
    triangles: &Vec<Triangle>,
) {
    for feat in results {
        if let Some(&place) = neighbor.get(&feat.pos_a) {
            let tri = &triangles[place];
            
        }
    }
}
