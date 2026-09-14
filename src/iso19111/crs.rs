#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Datum {
    WGS84,
    NAD83,
    ETRS89,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CRS {
    pub epsg: u32,
    pub datum: Datum,
}

impl CRS {
    pub const WGS84: CRS = CRS {
        epsg: 4326,
        datum: Datum::WGS84,
    };
    pub const WEB_MERCATOR: CRS = CRS {
        epsg: 3857,
        datum: Datum::WGS84,
    };
    pub fn is_geographic(&self) -> bool {
        self.epsg == 4326 || self.epsg == 4269 || self.epsg == 4258
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn wgs84_geographic() {
        assert!(CRS::WGS84.is_geographic());
    }
    #[test]
    fn web_mercator_not() {
        assert!(!CRS::WEB_MERCATOR.is_geographic());
    }
}
