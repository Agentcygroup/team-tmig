use crate::fiber::projection::Projection;

#[derive(Clone, Debug)]
pub struct Bundle {
    pub name: String,
    pub base: String,
    pub fiber: String,
    pub total: String,
    pub proj: Projection,
}

impl Bundle {
    pub fn torus_bundle() -> Self {
        Self {
            name: "torus".into(),
            base: "S1".into(),
            fiber: "S1".into(),
            total: "S1 x S1".into(),
            proj: Projection { from: "S1 x S1".into(), to: "S1".into(), index: 0 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn torus_bundle() {
        let b = Bundle::torus_bundle();
        assert_eq!(b.base, "S1");
        assert_eq!(b.fiber, "S1");
    }
}
