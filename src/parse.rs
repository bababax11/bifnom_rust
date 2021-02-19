use std::fs::{File};
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use regex::Regex;


use crate::structs::*;

macro_rules! input {
  (source = $s:expr, $($r:tt)*) => {
      let mut iter = $s.split_whitespace();
      input_inner!{iter, $($r)*}
  };
  ($($r:tt)*) => {
      let mut s = {
          use std::io::Read;
          let mut s = String::new();
          std::io::stdin().read_to_string(&mut s).unwrap();
          s
      };
      let mut iter = s.split_whitespace();
      input_inner!{iter, $($r)*}
  };
}

macro_rules! input_inner {
  ($iter:expr) => {};
  ($iter:expr, ) => {};

  ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
      let $var = read_value!($iter, $t);
      input_inner!{$iter $($r)*}
  };

  ($iter:expr, mut $var:ident : $t:tt $($r:tt)*) => {
      let mut $var = read_value!($iter, $t);
      input_inner!{$iter $($r)*}
  };
}

macro_rules! read_value {
  ($iter:expr, ( $($t:tt),* )) => {
      ( $(read_value!($iter, $t)),* )
  };

  ($iter:expr, [ $t:tt ; $len:expr ]) => {
      (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
  };

  ($iter:expr, chars) => {
      read_value!($iter, String).chars().collect::<Vec<char>>()
  };

  ($iter:expr, usize1) => {
      read_value!($iter, usize) - 1
  };

  ($iter:expr, $t:ty) => {
      $iter.next().unwrap().parse::<$t>().expect("Parse error")
  };
}


// A simple implementation of `% cat path`
// `% cat path`のシンプルな実装
pub fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

pub fn neighbor_parse(path: &Path) -> HashMap<(usize, usize), usize> {
  let re_line = Regex::new(r"\d+").unwrap();
  let mut results = HashMap::with_capacity(35000);

  for s in cat(&path).unwrap().split("\r\n").filter(|s| s.len() >= 3) {
    let mut cs = re_line.captures_iter(&s).map(|w| w[0].parse::<usize>().unwrap());;
    let (x, y, t) = (cs.next().unwrap(), cs.next().unwrap(), cs.next().unwrap());
    results.insert((x, y), t);
  }
  results
}

pub fn triangles_parse(path: &Path) -> Vec<Triangle> {
  let mut results = Vec::with_capacity(23000);

  for s in cat(&path).unwrap().split("\r\n").filter(|s| s.len() >= 3) {
    input!(source = s, 
      _t: String,
      t: usize,
      _a: String,
      a: [f32; 3],
      _b: String,
      b: [f32; 3],
      _c: String,
      c: [f32; 3]
    );
    results.push(Triangle{
      t, a: [a[0], a[1], a[2]], b: [b[0], b[1], b[2]], c: [c[0], c[1], c[2]]
    });
  }
  results
}

pub fn parse_result(path: &Path) -> HashMap<usize, Vec<Feature>> {
  let mut results = HashMap::<usize, Vec<Feature>>::with_capacity(10000);
  for s in cat(&path).unwrap().split("\r\n").filter(|s| s.len() >= 3) {
    input!(source = s,
      photo_no: usize,
      _is_f: i8,
      pos_a: [usize; 2],
      pos_b: [usize; 2],
      a_l: [f32; 3],
      a_m: [f32; 3],
      b_l: [f32; 3],
      b_m: [f32; 3]
    );
    results.entry(photo_no).or_insert(vec![]).push(Feature{
      pos_a: [pos_a[0], pos_a[1]],
      pos_b: [pos_b[0], pos_b[1]],
      a_l: [a_l[0], a_l[1], a_l[2]],
      a_m: [a_m[0], a_m[1], a_m[2]],
      b_l: [b_l[0], b_l[1], b_l[2]],
      b_m: [b_m[0], b_m[1], b_m[2]],
    });
  }
  results
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn neighbor_parse_test() {
    let hm = neighbor_parse(&Path::new("Input_armadillo/nei/neighbor_points_00000.txt"));
    dbg!(hm.keys().collect::<std::collections::BTreeSet<_>>());
  }

  #[test]
  fn triangles_parse_test() {
    let tris = triangles_parse(&Path::new("Input_armadillo/tri/triangles_00000.txt"));
    dbg!(tris);
  }
}

