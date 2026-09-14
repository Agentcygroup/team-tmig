#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BettiNumbers {
    pub b0: u32,
    pub b1: u32,
    pub b2: u32,
}
pub fn betti(s: &str) -> BettiNumbers {
    match s {
        "sphere" => BettiNumbers {
            b0: 1,
            b1: 0,
            b2: 1,
        },
        "torus" => BettiNumbers {
            b0: 1,
            b1: 2,
            b2: 1,
        },
        "double_torus" => BettiNumbers {
            b0: 1,
            b1: 4,
            b2: 1,
        },
        _ => BettiNumbers {
            b0: 0,
            b1: 0,
            b2: 0,
        },
    }
}
