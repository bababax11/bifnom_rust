#[derive(Clone, Debug, PartialEq)]
pub struct Triangle {
  pub t: usize,
  pub a: [f32; 3],
  pub b: [f32; 3],
  pub c: [f32; 3],
}
#[derive(Clone, Debug, PartialEq)]
pub struct Feature {
  pub pos_a: [usize; 2], /// 画像Aでの特徴点の位置
  pub pos_b: [usize; 2], /// 画像Bでの特徴点の位置
  pub a_l: [f32; 3],
  pub a_m: [f32; 3],
  pub b_l: [f32; 3],
  pub b_m: [f32; 3],
}

impl Feature {
  fn cross(l: &[f32], m: &[f32]) -> [f32; 3] {
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

