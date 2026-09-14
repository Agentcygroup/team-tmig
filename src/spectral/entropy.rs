pub fn shannon_entropy(p: &[f64]) -> f64 {
    let total: f64 = p.iter().sum();
    if total <= 0.0 {
        return 0.0;
    }
    p.iter()
        .filter(|&&x| x > 0.0)
        .map(|&x| {
            let q = x / total;
            -q * q.log2()
        })
        .sum()
}
