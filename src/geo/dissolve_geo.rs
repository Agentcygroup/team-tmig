use geo::{Polygon, MultiPolygon, BooleanOps};

pub fn dissolve(polys: Vec<Polygon<f64>>) -> MultiPolygon<f64> {
    let mut acc = MultiPolygon::new(Vec::new());
    for p in polys {
        let mp: MultiPolygon<f64> = p.into();
        acc = acc.union(&mp);
    }
    acc
}

pub fn dissolve_count(polys: Vec<Polygon<f64>>) -> usize {
    dissolve(polys).0.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo::polygon;
    #[test] fn one_poly() {
        let p = polygon![(x: 0.0, y: 0.0), (x: 1.0, y: 0.0), (x: 1.0, y: 1.0), (x: 0.0, y: 1.0)];
        assert_eq!(dissolve_count(vec![p]), 1);
    }
    #[test] fn overlapping() {
        let a = polygon![(x: 0.0, y: 0.0), (x: 2.0, y: 0.0), (x: 2.0, y: 2.0), (x: 0.0, y: 2.0)];
        let b = polygon![(x: 1.0, y: 1.0), (x: 3.0, y: 1.0), (x: 3.0, y: 3.0), (x: 1.0, y: 3.0)];
        assert_eq!(dissolve_count(vec![a, b]), 1);
    }
    #[test] fn disjoint() {
        let a = polygon![(x: 0.0, y: 0.0), (x: 1.0, y: 0.0), (x: 1.0, y: 1.0), (x: 0.0, y: 1.0)];
        let b = polygon![(x: 5.0, y: 5.0), (x: 6.0, y: 5.0), (x: 6.0, y: 6.0), (x: 5.0, y: 6.0)];
        assert_eq!(dissolve_count(vec![a, b]), 2);
    }
}
