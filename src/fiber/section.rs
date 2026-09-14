#[derive(Clone, Debug)]
pub struct Section {
    pub name: String,
    pub map: Vec<f64>,
    pub is_global: bool,
}

impl Section {
    pub fn new(name: &str, map: Vec<f64>) -> Self {
        Self { name: name.into(), map, is_global: true }
    }
    pub fn is_zero(&self) -> bool {
        self.map.iter().all(|&x| x == 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn zero_section() {
        let s = Section::new("zero", vec![0.0; 4]);
        assert!(s.is_zero());
    }
}
