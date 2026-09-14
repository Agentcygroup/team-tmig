#[derive(Clone, Debug, PartialEq)]
pub struct Chain {
    pub coefficients: Vec<i64>,
}

impl Chain {
    pub fn new(n: usize) -> Self { Self { coefficients: vec![0; n] } }
    pub fn add(&self, other: &Self) -> Option<Self> {
        if self.coefficients.len() != other.coefficients.len() { return None; }
        Some(Self {
            coefficients: self.coefficients.iter().zip(&other.coefficients).map(|(a, b)| a + b).collect(),
        })
    }
    pub fn is_zero(&self) -> bool { self.coefficients.iter().all(|&c| c == 0) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn zero_chain() {
        assert!(Chain::new(5).is_zero());
    }
    #[test] fn add_shapes() {
        let a = Chain::new(3);
        let b = Chain::new(4);
        assert!(a.add(&b).is_none());
    }
}
