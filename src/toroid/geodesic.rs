use crate::toroid::torus::TorusCoord;
use crate::toroid::wrap::wrap_f64;

pub fn torus_distance(a: TorusCoord, b: TorusCoord, r: f64, r_minor: f64) -> f64 {
    let two_pi = 2.0 * std::f64::consts::PI;
    let du = wrap_f64(a.u - b.u, two_pi);
    let dv = wrap_f64(a.v - b.v, two_pi);
    let du_short = du.min(two_pi - du);
    let dv_short = dv.min(two_pi - dv);
    let arc = ((r + r_minor * dv_short.cos()) * du_short).powi(2)
        + (r_minor * dv_short).powi(2);
    arc.sqrt()
}

pub fn torus_bearing(a: TorusCoord, b: TorusCoord) -> f64 {
    let two_pi = 2.0 * std::f64::consts::PI;
    let du = wrap_f64(b.u - a.u, two_pi);
    let dv = wrap_f64(b.v - a.v, two_pi);
    let du_short = if du > std::f64::consts::PI { du - two_pi } else { du };
    let dv_short = if dv > std::f64::consts::PI { dv - two_pi } else { dv };
    dv_short.atan2(du_short)
}

pub fn shortest_path(a: TorusCoord, b: TorusCoord, steps: usize) -> Vec<TorusCoord> {
    if steps == 0 { return vec![a]; }
    let mut out = Vec::with_capacity(steps + 1);
    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        let two_pi = 2.0 * std::f64::consts::PI;
        let du = wrap_f64(b.u - a.u + std::f64::consts::PI, two_pi) - std::f64::consts::PI;
        let dv = wrap_f64(b.v - a.v + std::f64::consts::PI, two_pi) - std::f64::consts::PI;
        out.push(TorusCoord {
            u: wrap_f64(a.u + t * du, two_pi),
            v: wrap_f64(a.v + t * dv, two_pi),
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn distance_is_short_path() {
        let a = TorusCoord { u: 0.1, v: 0.1 };
        let b = TorusCoord { u: 0.2, v: 0.1 };
        let d = torus_distance(a, b, 1.0, 0.3);
        assert!(d < 1.0);
    }
    #[test] fn distance_wraps() {
        let a = TorusCoord { u: 0.01, v: 0.0 };
        let b = TorusCoord { u: 6.27, v: 0.0 };
        let d = torus_distance(a, b, 1.0, 0.3);
        assert!(d < 0.05);
    }
    #[test] fn path_ends_at_b() {
        let a = TorusCoord { u: 0.0, v: 0.0 };
        let b = TorusCoord { u: 1.0, v: 1.0 };
        let p = shortest_path(a, b, 10);
        assert_eq!(p.len(), 11);
        assert!((p[0].u - a.u).abs() < 1e-9);
        assert!((p[10].u - b.u).abs() < 1e-9);
    }
}
