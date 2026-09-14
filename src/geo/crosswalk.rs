use crate::geo::zip::Zip;
use crate::geo::boundary::Boundary11;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct CrosswalkEntry {
    pub zip: u32,
    pub boundary: Boundary11,
    pub residential_ratio: f32,
    pub business_ratio: f32,
    pub other_ratio: f32,
}

#[derive(Default)]
pub struct ZipTractCrosswalk {
    by_zip: BTreeMap<u32, Vec<CrosswalkEntry>>,
    by_boundary: BTreeMap<String, Vec<u32>>,
}

impl ZipTractCrosswalk {
    pub fn new() -> Self { Self::default() }

    pub fn insert(&mut self, e: CrosswalkEntry) {
        self.by_zip.entry(e.zip).or_default().push(e.clone());
        self.by_boundary
            .entry(e.boundary.as_string())
            .or_default()
            .push(e.zip);
    }

    pub fn tracts_for_zip(&self, zip: u32) -> &[CrosswalkEntry] {
        self.by_zip.get(&zip).map(Vec::as_slice).unwrap_or(&[])
    }

    pub fn zips_for_tract(&self, tract: &str) -> &[u32] {
        self.by_boundary.get(tract).map(Vec::as_slice).unwrap_or(&[])
    }

    pub fn dominant_tract(&self, zip: u32) -> Option<&CrosswalkEntry> {
        self.tracts_for_zip(zip)
            .iter()
            .max_by(|a, b| a.residential_ratio.partial_cmp(&b.residential_ratio).unwrap())
    }

    pub fn validate(&self, z: Zip, b: Boundary11) -> bool {
        self.by_zip
            .get(&z.zip5)
            .map(|v| v.iter().any(|e| e.boundary == b))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn insert_and_query() {
        let mut cw = ZipTractCrosswalk::new();
        let b = Boundary11::new(6, 37, 207502).unwrap();
        cw.insert(CrosswalkEntry {
            zip: 90024, boundary: b, residential_ratio: 0.8,
            business_ratio: 0.15, other_ratio: 0.05,
        });
        assert_eq!(cw.tracts_for_zip(90024).len(), 1);
        assert_eq!(cw.zips_for_tract("06037207502"), &[90024]);
        assert!(cw.validate(Zip::new(90024).unwrap(), b));
        assert!(!cw.validate(Zip::new(10001).unwrap(), b));
    }
}
