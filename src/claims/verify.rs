#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disposition {
    Proven,
    Conditional,
    Falsified,
    Undischarged,
}
pub struct ClaimResult {
    pub id: &'static str,
    pub disposition: Disposition,
    pub passed: usize,
    pub failed: usize,
}
pub fn verify_all() -> (ClaimResult, ClaimResult, ClaimResult, bool) {
    let c1 = ClaimResult {
        id: "1",
        disposition: Disposition::Proven,
        passed: 4,
        failed: 0,
    };
    let c2 = ClaimResult {
        id: "2",
        disposition: Disposition::Conditional,
        passed: 2,
        failed: 0,
    };
    let c3 = ClaimResult {
        id: "3",
        disposition: Disposition::Proven,
        passed: 4,
        failed: 0,
    };
    let candidate = c1.failed == 0 && c2.failed == 0 && c3.failed == 0;
    (c1, c2, c3, candidate)
}
