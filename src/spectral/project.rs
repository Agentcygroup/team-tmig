pub fn project_2d(points: &[Vec<f64>]) -> Vec<(f64, f64)> {
    if points.is_empty() { return vec![]; }
    let n = points[0].len();
    let mut mean = vec![0.0; n];
    for p in points { for (i, v) in p.iter().enumerate() { mean[i] += v; } }
    for m in &mut mean { *m /= points.len() as f64; }
    let mut centered: Vec<Vec<f64>> = points.iter()
        .map(|p| p.iter().zip(&mean).map(|(a, b)| a - b).collect())
        .collect();
    centered.sort_by(|a, b| a.partial_cmp(b).unwrap());
    centered.iter().map(|p| (p.get(0).copied().unwrap_or(0.0), p.get(1).copied().unwrap_or(0.0))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn empty_input() {
        assert!(project_2d(&[]).is_empty());
    }
    #[test] fn same_length_as_input() {
        let pts = vec![vec![1.0, 2.0], vec![3.0, 4.0], vec![5.0, 6.0]];
        assert_eq!(project_2d(&pts).len(), 3);
    }
}
