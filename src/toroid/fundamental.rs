#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FundamentalDomain {
    pub u_min: f64,
    pub u_max: f64,
    pub v_min: f64,
    pub v_max: f64,
}

pub fn fundamental_domain() -> FundamentalDomain {
    let two_pi = 2.0 * std::f64::consts::PI;
    FundamentalDomain { u_min: 0.0, u_max: two_pi, v_min: 0.0, v_max: two_pi }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn has_positive_measure() {
        let f = fundamental_domain();
        assert!(f.u_max > f.u_min);
        assert!(f.v_max > f.v_min);
    }
}
