#[derive(Clone, Debug)]
pub enum MetricValue { Integer(i64), Float(f64), Bytes(u64), Ratio { num: i64, den: i64 } }
impl MetricValue {
    pub fn as_f64(&self) -> f64 {
        match self {
            Self::Integer(i) => *i as f64, Self::Float(f) => *f,
            Self::Bytes(b) => *b as f64,
            Self::Ratio { num, den } => if *den == 0 { 0.0 } else { *num as f64 / *den as f64 },
        }
    }
}
