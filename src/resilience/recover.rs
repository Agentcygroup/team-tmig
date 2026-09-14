#[derive(Clone, Copy, Debug)]
pub struct Recovery {
    pub rto_seconds: u32,
    pub rpo_seconds: u32,
    pub auto: bool,
}

impl Recovery {
    pub fn omega() -> Self {
        Self {
            rto_seconds: 60,
            rpo_seconds: 0,
            auto: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rto_under_minute() {
        assert!(Recovery::omega().rto_seconds <= 60);
    }
}
