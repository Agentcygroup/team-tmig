#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GM_Point {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GM_Curve {
    pub points: Vec<GM_Point>,
    pub closed: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GM_Surface {
    pub exterior: Vec<GM_Point>,
    pub holes: Vec<Vec<GM_Point>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GM_Solid {
    pub shells: Vec<GM_Surface>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TopoDim {
    Point,
    Curve,
    Surface,
    Solid,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn point_dim() {
        assert_eq!(TopoDim::Point, TopoDim::Point);
    }
    #[test]
    fn curve_construction() {
        let c = GM_Curve {
            points: vec![
                GM_Point {
                    x: 0.0,
                    y: 0.0,
                    z: None,
                },
                GM_Point {
                    x: 1.0,
                    y: 0.0,
                    z: None,
                },
            ],
            closed: false,
        };
        assert_eq!(c.points.len(), 2);
    }
}
