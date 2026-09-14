pub fn power_iteration(a: &[Vec<f64>], iters: usize) -> (Vec<f64>, f64) {
    let n = a.len();
    if n == 0 {
        return (vec![], 0.0);
    }
    if a.iter().any(|r| r.len() != n) {
        return (vec![], 0.0);
    }
    let mut v = vec![1.0 / (n as f64).sqrt(); n];
    let mut lambda = 0.0;
    for _ in 0..iters {
        let mut w = vec![0.0; n];
        for i in 0..n {
            for j in 0..n {
                w[i] += a[i][j] * v[j];
            }
        }
        let norm: f64 = w.iter().map(|x| x * x).sum::<f64>().sqrt();
        if norm < 1e-15 {
            break;
        }
        for x in &mut w {
            *x /= norm;
        }
        lambda = w.iter().zip(&v).map(|(x, y)| x * y).sum();
        v = w;
    }
    (v, lambda)
}
