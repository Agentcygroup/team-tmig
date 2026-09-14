use crate::payload::registry::Registry;
use crate::payload::instantiate::instantiate;

pub fn verify_all(r: &Registry, seed: &[u8]) -> (usize, usize) {
    let mut pass = 0;
    let mut fail = 0;
    for s in &r.specs {
        let b = instantiate(s, seed);
        if !b.is_empty() { pass += 1 } else { fail += 1 }
    }
    (pass, fail)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn all_pass() {
        let r = Registry::omega();
        let (p, f) = verify_all(&r, b"seed");
        assert_eq!(f, 0);
        assert!(p >= 90);
    }
}
