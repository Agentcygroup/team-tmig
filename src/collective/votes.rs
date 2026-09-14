pub const PRIOR_WEIGHT: f32 = 0.5;
pub fn accuracy_weight(n: usize, k: usize) -> f32 {
    if n == 0 { return PRIOR_WEIGHT; }
    (k as f32 + 1.0) / (n as f32 + 2.0)
}
