use ndarray::prelude::*;
use once_cell::sync::Lazy;

pub const EPS: f32 = 1e-7;

pub const WIDTH: usize = 640;
pub const HEIGHT: usize = 320;

pub static INV_V: Lazy<Array2<f32>> = Lazy::new(|| {
    array![
        [-0.707107, 0.000000, 0.707107, 0.000000],
        [0.122788, 0.984808, 0.122788, 0.000001],
        [-0.696364, 0.173648, -0.696364, -30.000003],
        [0.000000, 0.000000, 0.000000, 1.000000],
    ]
});

pub static PRJ: Lazy<Array2<f32>> = Lazy::new(|| {
    array![
        [16.107893, 0.000000, 0.000000, 0.000000],
        [0.000000, 28.636253, 0.000000, 0.000000],
        [0.000000, 0.000000, -1.020202, -2.020202],
        [0.000000, 0.000000, -1.000000, 0.000000],
    ]
});

pub static PRJ00: Lazy<f32> = Lazy::new(|| PRJ[[0, 0]]);
pub static PRJ11: Lazy<f32> = Lazy::new(|| PRJ[[1, 1]]);
