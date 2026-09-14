#[derive(Clone, Debug)]
pub struct Witness {
    pub name: String,
    pub observed_hash: [u8; 32],
    pub signature: [u8; 64],
}

impl Witness {
    pub fn new(name: &str, observed_hash: [u8; 32]) -> Self {
        Self {
            name: name.into(),
            observed_hash,
            signature: [0; 64],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn construct() {
        let w = Witness::new("x", [0; 32]);
        assert_eq!(w.name, "x");
    }
}
