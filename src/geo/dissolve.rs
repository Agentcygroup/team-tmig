use crate::geo::boundary::Boundary11;
use std::collections::{BTreeSet, BTreeMap};

pub fn dissolve(boundaries: &[Boundary11]) -> BTreeSet<String> {
    boundaries.iter().map(Boundary11::as_string).collect()
}

pub fn union_of(boundaries: &[Boundary11], state: u8) -> Vec<Boundary11> {
    boundaries.iter().filter(|b| b.state_fips == state).copied().collect()
}

pub fn dissolve_by_county(boundaries: &[Boundary11]) -> BTreeMap<(u8, u16), Vec<Boundary11>> {
    let mut m: BTreeMap<(u8, u16), Vec<Boundary11>> = BTreeMap::new();
    for b in boundaries {
        m.entry((b.state_fips, b.county_fips)).or_default().push(*b);
    }
    m
}

pub fn dissolve_by_tract(boundaries: &[Boundary11]) -> BTreeMap<u32, Vec<Boundary11>> {
    let mut m: BTreeMap<u32, Vec<Boundary11>> = BTreeMap::new();
    for b in boundaries {
        m.entry(b.tract).or_default().push(*b);
    }
    m
}

pub fn dissolve_by_state(boundaries: &[Boundary11]) -> BTreeMap<u8, Vec<Boundary11>> {
    let mut m: BTreeMap<u8, Vec<Boundary11>> = BTreeMap::new();
    for b in boundaries {
        m.entry(b.state_fips).or_default().push(*b);
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn dedupes() {
        let a = Boundary11::new(6, 37, 207502).unwrap();
        assert_eq!(dissolve(&[a, a, a]).len(), 1);
    }
    #[test] fn by_county() {
        let a = Boundary11::new(6, 37, 207502).unwrap();
        let b = Boundary11::new(6, 37, 207503).unwrap();
        let c = Boundary11::new(6, 38, 100).unwrap();
        let m = dissolve_by_county(&[a, b, c]);
        assert_eq!(m.get(&(6, 37)).unwrap().len(), 2);
        assert_eq!(m.get(&(6, 38)).unwrap().len(), 1);
    }
    #[test] fn by_state() {
        let a = Boundary11::new(6, 37, 207502).unwrap();
        let b = Boundary11::new(36, 61, 100).unwrap();
        let m = dissolve_by_state(&[a, b]);
        assert_eq!(m.get(&6).unwrap().len(), 1);
        assert_eq!(m.get(&36).unwrap().len(), 1);
    }
}
