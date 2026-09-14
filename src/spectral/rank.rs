#![allow(clippy::needless_range_loop)]
use crate::spectral::matrix::Matrix;
pub fn rank(m: &Matrix, eps: f64) -> usize {
    let mut a = m.data.clone();
    let (r, c) = m.shape();
    let mut rank = 0;
    let mut col = 0;
    while rank < r && col < c {
        let mut piv = rank;
        let mut best = a[piv][col].abs();
        for i in rank + 1..r {
            if a[i][col].abs() > best {
                best = a[i][col].abs();
                piv = i;
            }
        }
        if best < eps {
            col += 1;
            continue;
        }
        a.swap(rank, piv);
        let p = a[rank][col];
        for i in 0..r {
            if i == rank {
                continue;
            }
            let f = a[i][col] / p;
            for j in col..c {
                a[i][j] -= f * a[rank][j];
            }
        }
        rank += 1;
        col += 1;
    }
    rank
}
