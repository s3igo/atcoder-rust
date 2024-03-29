use std::collections::HashSet;

use itertools::iproduct;
use proconio::input;

fn main() {
    input!(a: [[usize; 9]; 9]);

    let cmp = (1..=9).collect::<Vec<_>>();
    let cmp = cmp.iter().collect::<HashSet<_>>();

    let cond = a.iter().all(|row| cmp == row.iter().collect())
        && rotate(&a, 1).iter().all(|row| cmp == row.iter().collect())
        && chunks(&a, 3).iter().all(|chunk| cmp == chunk.iter().collect());

    println!("{}", if cond { "Yes" } else { "No" });
}

fn rotate<T: Copy>(matrix: &Vec<Vec<T>>, times: usize) -> Vec<Vec<T>> {
    let (n, m) = (matrix.len(), matrix[0].len());
    assert!(matrix.iter().map(|row| row.len()).all(|x| x == m));
    match times {
        0 => matrix.clone(),
        _ => rotate(&(0..m).map(|i| (0..n).map(|j| matrix[n - 1 - j][i]).collect()).collect(), times - 1),
    }
}

fn chunks<T: Copy>(matrix: &Vec<Vec<T>>, side: usize) -> Vec<Vec<T>> {
    let n = matrix.len();
    assert!(matrix.iter().all(|row| row.len() == n));
    assert!(n % side == 0);
    iproduct!((0..n).step_by(side), (0..n).step_by(side))
        .map(|(i, j)| iproduct!(0..side, 0..side).map(|(k, l)| matrix[i + k][j + l]).collect())
        .collect()
}
