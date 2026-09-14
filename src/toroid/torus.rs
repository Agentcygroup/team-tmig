use crate::toroid::wrap::{wrap_f64, wrap_i64};
use crate::toroid::geodesic::torus_distance;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TorusCoord {
    pub u: f64,
    pub v: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Torus {
    pub r: f64,
    pub r_minor: f64,
}

impl Torus {
    pub fn new(r: f64, r_minor: f64) -> Option<Self> {
        if r > 0.0 && r_minor > 0.0 { Some(Self { r, r_minor }) } else { None }
    }

    pub fn wrap(&self, c: TorusCoord) -> TorusCoord {
        TorusCoord {
            u: wrap_f64(c.u, 2.0 * std::f64::consts::PI),
            v: wrap_f64(c.v, 2.0 * std::f64::consts::PI),
        }
    }

    pub fn distance(&self, a: TorusCoord, b: TorusCoord) -> f64 {
        torus_distance(a, b, self.r, self.r_minor)
    }

    pub fn area(&self) -> f64 {
        4.0 * std::f64::consts::PI.powi(2) * self.r * self.r_minor
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn wrap_puts_in_range() {
        let t = Torus::new(1.0, 0.3).unwrap();
        let c = t.wrap(TorusCoord { u: -0.1, v: 7.0 });
        assert!(c.u >= 0.0 && c.u < 2.0 * std::f64::consts::PI);
        assert!(c.v >= 0.0 && c.v < 2.0 * std::f64::consts::PI);
    }
    #[test] fn area_positive() {
        let t = Torus::new(2.0, 1.0).unwrap();
        assert!(t.area() > 0.0);
    }
}
