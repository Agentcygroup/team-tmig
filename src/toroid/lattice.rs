#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LatticePoint(pub i64, pub i64);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Lattice {
    pub m: i64,
    pub n: i64,
}

impl Lattice {
    pub fn new(m: i64, n: i64) -> Option<Self> {
        if m > 0 && n > 0 { Some(Self { m, n }) } else { None }
    }

    pub fn wrap(&self, p: LatticePoint) -> LatticePoint {
        LatticePoint(p.0.rem_euclid(self.m), p.1.rem_euclid(self.n))
    }

    pub fn neighbors(&self, p: LatticePoint) -> [LatticePoint; 4] {
        let w = self.wrap(p);
        [
            self.wrap(LatticePoint(w.0 + 1, w.1)),
            self.wrap(LatticePoint(w.0 - 1, w.1)),
            self.wrap(LatticePoint(w.0, w.1 + 1)),
            self.wrap(LatticePoint(w.0, w.1 - 1)),
        ]
    }

    pub fn cardinality(&self) -> i64 {
        self.m * self.n
    }

    pub fn euler_characteristic(&self) -> i64 {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn wrap_negatives() {
        let l = Lattice::new(3, 3).unwrap();
        assert_eq!(l.wrap(LatticePoint(-1, -1)).0, 2);
    }
    #[test] fn cardinality() {
        let l = Lattice::new(4, 5).unwrap();
        assert_eq!(l.cardinality(), 20);
    }
    #[test] fn euler_is_zero() {
        let l = Lattice::new(3, 3).unwrap();
        assert_eq!(l.euler_characteristic(), 0);
    }
}
