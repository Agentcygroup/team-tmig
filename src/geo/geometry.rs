use crate::geo::point::GeoPoint;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point { pub x: f64, pub y: f64 }

#[derive(Clone, Debug, PartialEq)]
pub struct Polygon { pub exterior: Vec<Point> }

#[derive(Clone, Debug, Default, PartialEq)]
pub struct MultiPolygon { pub polygons: Vec<Polygon> }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BoundingBox { pub min_x: f64, pub min_y: f64, pub max_x: f64, pub max_y: f64 }

impl BoundingBox {
    pub fn of(poly: &Polygon) -> Option<Self> {
        let xs: Vec<f64> = poly.exterior.iter().map(|p| p.x).collect();
        let ys: Vec<f64> = poly.exterior.iter().map(|p| p.y).collect();
        let min_x = xs.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_x = xs.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_y = ys.iter().cloned().fold(f64::INFINITY, f64::min);
        let max_y = ys.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        if min_x.is_finite() && max_x.is_finite() {
            Some(Self { min_x, min_y, max_x, max_y })
        } else {
            None
        }
    }
    pub fn contains(&self, p: Point) -> bool {
        p.x >= self.min_x && p.x <= self.max_x && p.y >= self.min_y && p.y <= self.max_y
    }
}

pub fn point_in_polygon(p: Point, poly: &Polygon) -> bool {
    let n = poly.exterior.len();
    let mut inside = false;
    let mut j = n - 1;
    for i in 0..n {
        let pi = poly.exterior[i];
        let pj = poly.exterior[j];
        if (pi.y > p.y) != (pj.y > p.y) {
            let x = (pj.x - pi.x) * (p.y - pi.y) / (pj.y - pi.y) + pi.x;
            if p.x < x { inside = !inside; }
        }
        j = i;
    }
    inside
}

pub fn bbox_to_geo(b: BoundingBox) -> (GeoPoint, GeoPoint) {
    (GeoPoint { lat: b.min_y, lon: b.min_x }, GeoPoint { lat: b.max_y, lon: b.max_x })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn bbox_of_square() {
        let sq = Polygon { exterior: vec![
            Point { x: 0.0, y: 0.0 }, Point { x: 1.0, y: 0.0 },
            Point { x: 1.0, y: 1.0 }, Point { x: 0.0, y: 1.0 },
        ]};
        let b = BoundingBox::of(&sq).unwrap();
        assert_eq!((b.min_x, b.min_y, b.max_x, b.max_y), (0.0, 0.0, 1.0, 1.0));
    }
    #[test] fn inside_square() {
        let sq = Polygon { exterior: vec![
            Point { x: 0.0, y: 0.0 }, Point { x: 2.0, y: 0.0 },
            Point { x: 2.0, y: 2.0 }, Point { x: 0.0, y: 2.0 },
        ]};
        assert!(point_in_polygon(Point { x: 1.0, y: 1.0 }, &sq));
        assert!(!point_in_polygon(Point { x: 3.0, y: 1.0 }, &sq));
    }
}
