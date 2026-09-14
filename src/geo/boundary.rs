#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Boundary11 {
    pub state_fips: u8,
    pub county_fips: u16,
    pub tract: u32,
}

impl Boundary11 {
    pub fn new(s: u8, c: u16, t: u32) -> Option<Self> {
        if (1..=99).contains(&s) && c <= 999 && t <= 999999 {
            Some(Self {
                state_fips: s,
                county_fips: c,
                tract: t,
            })
        } else {
            None
        }
    }

    /// Inverse of `as_string`.  Format: 2-digit state, 3-digit county, 6-digit tract = 11 chars.
    pub fn parse(s: &str) -> Option<Self> {
        if s.len() != 11 || !s.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let state: u8 = s[0..2].parse().ok()?;
        let county: u16 = s[2..5].parse().ok()?;
        let tract: u32 = s[5..11].parse().ok()?;
        Boundary11::new(state, county, tract)
    }

    pub fn as_string(&self) -> String {
        format!(
            "{:02}{:03}{:06}",
            self.state_fips, self.county_fips, self.tract
        )
    }
}
