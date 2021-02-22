use ndarray::prelude::*;
use crate::structs::Quaternion;

/// http://marupeke296.sakura.ne.jp/DXG_No58_RotQuaternionTrans.html
/// https://www.euclideanspace.com/maths/geometry/rotations/conversions/matrixToQuaternion/
/// https://docs.rs/nalgebra/0.24.1/src/nalgebra/geometry/quaternion_construction.rs.html#284-342
pub fn rot_mat_to_q(m: ArrayView2<f32>) -> Quaternion {
    let elem = vec![
        m[[0, 0]] - m[[1, 1]] - m[[2, 2]] + 1.0,
        -m[[0, 0]] + m[[1, 1]] - m[[2, 2]] + 1.0,
        -m[[0, 0]] - m[[1, 1]] + m[[2, 2]] + 1.0,
        m[[0, 0]] + m[[1, 1]] + m[[2, 2]] + 1.0,
    ];
    let biggest_idx = {
        let mut idx = 0;
        for i in 1..4 {
            if elem[i] > elem[idx] {
                idx = i;
            }
        }
        idx
    };
    assert!(elem[biggest_idx] >= 0.0);

    let v = f32::sqrt(elem[biggest_idx]) * 0.5;
    let mult = 0.25 / v;

    let mut ans = [0.0; 4];
    ans[biggest_idx] = v;

    match biggest_idx {
        0 => {
            ans[1] = (m[[0, 1]] + m[[1, 0]]) * mult;
            ans[2] = (m[[2, 0]] + m[[0, 2]]) * mult;
            ans[3] = (m[[1, 2]] - m[[2, 1]]) * mult;
        }
        1 => {
            ans[0] = (m[[0, 1]] + m[[1, 0]]) * mult;
            ans[2] = (m[[1, 2]] + m[[2, 1]]) * mult;
            ans[3] = (m[[2, 1]] - m[[0, 1]]) * mult;
        }
        2 => {
            ans[0] = (m[[2, 0]] + m[[0, 2]]) * mult;
            ans[1] = (m[[1, 2]] + m[[2, 1]]) * mult;
            ans[3] = (m[[0, 1]] - m[[1, 0]]) * mult;
        }
        3 => {
            ans[0] = (m[[1, 2]] - m[[2, 1]]) * mult;
            ans[1] = (m[[2, 0]] - m[[0, 2]]) * mult;
            ans[2] = (m[[0, 1]] - m[[2, 0]]) * mult;
        }
        _ => unreachable!(),
    }
    Quaternion{q: ans}
}
