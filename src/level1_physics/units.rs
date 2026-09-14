#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Seconds(pub f64);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Meters(pub f64);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Kilograms(pub f64);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bits(pub f64);

pub fn seconds(t: f64) -> Seconds { Seconds(t) }
pub fn meters(d: f64) -> Meters { Meters(d) }
pub fn kg(m: f64) -> Kilograms { Kilograms(m) }
pub fn bits(b: f64) -> Bits { Bits(b) }
