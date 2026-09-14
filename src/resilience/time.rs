#[derive(Clone, Copy, Debug)]
pub struct TimeWitness {
    pub rfc3339: u64,
    pub monotonic: u64,
}

impl TimeWitness {
    pub fn now() -> Self {
        let t = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        Self {
            rfc3339: t.as_secs(),
            monotonic: t.as_millis() as u64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn monotone() {
        let a = TimeWitness::now();
        let b = TimeWitness::now();
        assert!(b.monotonic >= a.monotonic);
    }
}
