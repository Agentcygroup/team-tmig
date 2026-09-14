pub fn rbf_kernel(x: &[f64], y: &[f64], sigma: f64) -> f64 {
    let d2: f64 = x.iter().zip(y).map(|(a, b)| (a - b).powi(2)).sum();
    (-d2 / (2.0 * sigma * sigma)).exp()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn self_kernel_is_one() {
        let x = vec![1.0, 2.0, 3.0];
        assert!((rbf_kernel(&x, &x, 1.0) - 1.0).abs() < 1e-12);
    }
    #[test] fn far_kernel_near_zero() {
        let x = vec![0.0];
        let y = vec![1000.0];
        assert!(rbf_kernel(&x, &y, 1.0) < 1e-6);
    }
}
