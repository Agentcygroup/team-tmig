#[derive(Clone, Debug)]
pub enum Shape { Bytes, Text, Json, Toml, Yaml, Binary, Merkle, Chain, KeyPair }

#[derive(Clone, Debug)]
pub struct Field { pub name: String, pub shape: Shape, pub required: bool }

#[derive(Clone, Debug)]
pub struct Spec {
    pub name: String,
    pub shape: Shape,
    pub fields: Vec<Field>,
}

impl Spec {
    pub fn empty(name: &str, shape: Shape) -> Self {
        Self { name: name.into(), shape, fields: Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn construct() {
        let s = Spec::empty("x", Shape::Json);
        assert_eq!(s.name, "x");
    }
}
