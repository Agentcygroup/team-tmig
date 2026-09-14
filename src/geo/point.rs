#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeoPoint {
    pub lat: f64,
    pub lon: f64,
}

const R_EARTH_M: f64 = 6_371_008.8;

pub fn haversine_m(a: GeoPoint, b: GeoPoint) -> f64 {
    let lat1 = a.lat.to_radians();
    let lat2 = b.lat.to_radians();
    let dlat = (b.lat - a.lat).to_radians();
    let dlon = (b.lon - a.lon).to_radians();
    let h = (dlat / 2.0).sin().powi(2)
        + lat1.cos() * lat2.cos() * (dlon / 2.0).sin().powi(2);
    2.0 * R_EARTH_M * h.sqrt().asin()
}

pub fn bearing_deg(a: GeoPoint, b: GeoPoint) -> f64 {
    let lat1 = a.lat.to_radians();
    let lat2 = b.lat.to_radians();
    let dlon = (b.lon - a.lon).to_radians();
    let y = dlon.sin() * lat2.cos();
    let x = lat1.cos() * lat2.sin() - lat1.sin() * lat2.cos() * dlon.cos();
    let deg = y.atan2(x).to_degrees();
    (deg + 360.0) % 360.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn new_york_to_london() {
        let ny = GeoPoint { lat: 40.7128, lon: -74.0060 };
        let ld = GeoPoint { lat: 51.5074, lon: -0.1278 };
        let d = haversine_m(ny, ld);
        assert!((d - 5_570_000.0).abs() < 20_000.0);
    }
    #[test] fn bearing_ny_to_london() {
        let ny = GeoPoint { lat: 40.7128, lon: -74.0060 };
        let ld = GeoPoint { lat: 51.5074, lon: -0.1278 };
        let b = bearing_deg(ny, ld);
        assert!(b > 40.0 && b < 60.0);
    }
}
