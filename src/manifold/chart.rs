#[derive(Clone, Debug, PartialEq)]
pub struct Chart {
    pub name: String,
    pub dim: usize,
    pub u_min: f64,
    pub u_max: f64,
    pub v_min: f64,
    pub v_max: f64,
}

impl Chart {
    pub fn torus_standard(name: &str) -> Self {
        let two_pi = 2.0 * std::f64::consts::PI;
        Self { name: name.into(), dim: 2, u_min: 0.0, u_max: two_pi, v_min: 0.0, v_max: two_pi }
    }
    pub fn contains(&self, u: f64, v: f64) -> bool {
        u >= self.u_min && u <= self.u_max && v >= self.v_min && v <= self.v_max
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn standard_chart_contains() {
        let c = Chart::torus_standard("main");
        assert!(c.contains(1.0, 1.0));
        assert!(!c.contains(-1.0, 1.0));
    }
}
