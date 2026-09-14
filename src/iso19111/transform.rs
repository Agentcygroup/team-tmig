#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeoCoord {
    pub lat: f64,
    pub lon: f64,
}

const EARTH_RADIUS: f64 = 6378137.0;
const ORIGIN_SHIFT: f64 = std::f64::consts::PI * EARTH_RADIUS;

pub fn wgs84_to_web_mercator(c: GeoCoord) -> (f64, f64) {
    let x = c.lon * ORIGIN_SHIFT / 180.0;
    let lat_rad = c.lat.to_radians();
    let y = EARTH_RADIUS * (std::f64::consts::PI / 4.0 + lat_rad / 2.0).tan().ln();
    (x, y)
}

pub fn web_mercator_to_wgs84(x: f64, y: f64) -> GeoCoord {
    let lon = x * 180.0 / ORIGIN_SHIFT;
    let lat_rad = 2.0 * (y / EARTH_RADIUS).exp().atan() - std::f64::consts::PI / 2.0;
    GeoCoord {
        lat: lat_rad.to_degrees(),
        lon,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn origin() {
        let c = GeoCoord { lat: 0.0, lon: 0.0 };
        let (x, y) = wgs84_to_web_mercator(c);
        assert!(x.abs() < 1e-6);
        assert!(y.abs() < 1e-6);
    }
    #[test]
    fn roundtrip() {
        let c = GeoCoord {
            lat: 40.7128,
            lon: -74.0060,
        };
        let (x, y) = wgs84_to_web_mercator(c);
        let b = web_mercator_to_wgs84(x, y);
        assert!((b.lat - c.lat).abs() < 1e-6);
        assert!((b.lon - c.lon).abs() < 1e-6);
    }
}
