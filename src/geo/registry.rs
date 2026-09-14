use crate::geo::{Zip, AreaCode, Boundary11};
use std::collections::BTreeSet;

#[derive(Default)]
pub struct GeoRegistry {
    pub zips: BTreeSet<u32>,
    pub area_codes: BTreeSet<u16>,
    pub boundaries: BTreeSet<String>,
}

impl GeoRegistry {
    pub fn new() -> Self { Self::default() }

    pub fn add_zip(&mut self, z: Zip) -> bool {
        self.zips.insert(z.zip5)
    }
    pub fn add_area(&mut self, a: AreaCode) -> bool {
        self.area_codes.insert(a.0)
    }
    pub fn add_boundary(&mut self, b: Boundary11) -> bool {
        self.boundaries.insert(b.as_string())
    }

    pub fn counts(&self) -> (usize, usize, usize) {
        (self.zips.len(), self.area_codes.len(), self.boundaries.len())
    }

    pub fn covers(&self, z: u32, a: u16, b: &str) -> bool {
        self.zips.contains(&z) && self.area_codes.contains(&a) && self.boundaries.contains(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn add_and_count() {
        let mut r = GeoRegistry::new();
        r.add_zip(Zip::new(90024).unwrap());
        r.add_area(AreaCode::new(310).unwrap());
        r.add_boundary(Boundary11::new(6, 37, 207502).unwrap());
        assert_eq!(r.counts(), (1, 1, 1));
    }
    #[test] fn covers_all_three() {
        let mut r = GeoRegistry::new();
        r.add_zip(Zip::new(90024).unwrap());
        r.add_area(AreaCode::new(310).unwrap());
        r.add_boundary(Boundary11::new(6, 37, 207502).unwrap());
        assert!(r.covers(90024, 310, "06037207502"));
        assert!(!r.covers(10001, 310, "06037207502"));
    }
}
