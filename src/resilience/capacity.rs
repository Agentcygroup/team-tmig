#[derive(Clone, Copy, Debug)]
pub struct Capacity {
    pub replicas: u32,
    pub shards: u32,
    pub geographic_zones: u32,
    pub failure_domains: u32,
}

impl Capacity {
    pub fn omega() -> Self {
        Self {
            replicas: 9,
            shards: 256,
            geographic_zones: 7,
            failure_domains: 5,
        }
    }
    pub fn redundancy_factor(&self) -> u32 {
        self.replicas
    }
    pub fn can_survive(&self, failures: u32) -> bool {
        failures < self.replicas
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn omega_survives_eight() {
        let c = Capacity::omega();
        assert!(c.can_survive(8));
        assert!(!c.can_survive(9));
    }
}
