use crate::consts::*;
use ndarray::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Triangle {
    pub t: usize,
    pub a: [f32; 3],
    pub b: [f32; 3],
    pub c: [f32; 3],
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}, {:?}, {:?}
        ", &self.a, &self.b, &self.c)
    }
}

pub struct TriangleCoo {
    pub prj_a: [f32; 2],
    pub prj_b: [f32; 2],
    pub prj_c: [f32; 2],
}

impl fmt::Display for TriangleCoo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}, {:?}, {:?}",
         &self.prj_a, &self.prj_b, &self.prj_c)
    }
}

fn prj2(xyz: &[f32; 3]) -> [f32; 2] {
    let [x, y, z] = *xyz;
    let a = array![x, y, z, 1.0];
    let a = INV_V.dot(&a);
    [
        (a[0] / (-a[2] + EPS) * *PRJ00 + 1.) * WIDTH as f32,
        (a[1] / (a[2] + EPS) * *PRJ11 + 1.) * HEIGHT as f32,
    ]
}

impl From<&Triangle> for TriangleCoo {
    #[inline]
    fn from(triangle: &Triangle) -> TriangleCoo {
        TriangleCoo {
            prj_a: prj2(&triangle.a),
            prj_b: prj2(&triangle.b),
            prj_c: prj2(&triangle.c),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Feature {
    /// 画像Aでの特徴点の位置
    pub pos_a: [usize; 2],
    /// 画像Bでの特徴点の位置
    pub pos_b: [usize; 2],
    pub a_l: [f32; 3],
    pub a_m: [f32; 3],
    pub b_l: [f32; 3],
    pub b_m: [f32; 3],
}

impl fmt::Display for Feature {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?}", &self.pos_a, &self.pos_b, &self.a_l, &self.a_m, &self.a_n(),
            &self.b_l, &self.b_m, &self.b_n())
    }
}

impl Feature {
    #[inline]
    fn cross(l: &[f32; 3], m: &[f32; 3]) -> [f32; 3] {
        [
            l[1] * m[2] - l[2] * m[1],
            l[2] * m[0] - l[0] * m[2],
            l[0] * m[1] - l[1] * m[0],
        ]
    }
    pub fn a_n(&self) -> [f32; 3] {
        Self::cross(&self.a_l, &self.a_m)
    }
    pub fn b_n(&self) -> [f32; 3] {
        Self::cross(&self.b_l, &self.b_m)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FeatCoo {
    pub st: [f32; 2],
}

#[derive(Clone, Debug, PartialEq)]
pub struct ProjectedPix {
    pub xy: [f32; 2],
}

#[derive(Clone, Debug, PartialEq)]
pub struct Quaternion {
    pub q: [f32; 4]
}

impl fmt::Display for FeatCoo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self.st)
    }
}

impl fmt::Display for ProjectedPix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self.xy)
    }
}

use std::fmt;
impl fmt::Display for Quaternion {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", &self.q)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn quaternion_test() {
        let q = Quaternion {
            q: [0.; 4]
        };
        println!("{}", q);
    }
}
