#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeoBBox {
    pub min_lat: f64,
    pub min_lon: f64,
    pub max_lat: f64,
    pub max_lon: f64,
}

impl GeoBBox {
    pub fn contains(&self, lat: f64, lon: f64) -> bool {
        lat >= self.min_lat && lat <= self.max_lat
            && lon >= self.min_lon && lon <= self.max_lon
    }
    pub fn intersects(&self, other: &Self) -> bool {
        !(self.max_lat < other.min_lat || self.min_lat > other.max_lat
          || self.max_lon < other.min_lon || self.min_lon > other.max_lon)
    }
    pub fn union(&self, other: &Self) -> Self {
        Self {
            min_lat: self.min_lat.min(other.min_lat),
            min_lon: self.min_lon.min(other.min_lon),
            max_lat: self.max_lat.max(other.max_lat),
            max_lon: self.max_lon.max(other.max_lon),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn contains_point() {
        let b = GeoBBox { min_lat: 40.0, min_lon: -75.0, max_lat: 41.0, max_lon: -74.0 };
        assert!(b.contains(40.5, -74.5));
        assert!(!b.contains(42.0, -74.5));
    }
    #[test] fn intersects_self() {
        let b = GeoBBox { min_lat: 0.0, min_lon: 0.0, max_lat: 1.0, max_lon: 1.0 };
        assert!(b.intersects(&b));
    }
}
