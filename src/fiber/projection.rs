#[derive(Clone, Debug)]
pub struct Projection {
    pub from: String,
    pub to: String,
    pub index: usize,
}
impl Projection {
    pub fn apply(&self, c: &[f64]) -> Option<f64> {
        c.get(self.index).copied()
    }
}
