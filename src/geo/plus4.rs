use crate::geo::zip::Zip;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Plus4Kind {
    BlockFace,
    Building,
    Firm,
    HighRise,
    POBox,
    Rural,
    Unknown,
}

pub fn classify_plus4(plus4: u32) -> Plus4Kind {
    match plus4 {
        0001..=1999 => Plus4Kind::BlockFace,
        2000..=2999 => Plus4Kind::Building,
        3000..=3999 => Plus4Kind::Firm,
        4000..=4999 => Plus4Kind::HighRise,
        5000..=5999 => Plus4Kind::POBox,
        6000..=8999 => Plus4Kind::Rural,
        9000..=9999 => Plus4Kind::Unknown,
        _ => Plus4Kind::Unknown,
    }
}

pub fn valid(z: Zip) -> bool {
    z.plus4.is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn block_face() {
        assert_eq!(classify_plus4(0100), Plus4Kind::BlockFace);
    }
    #[test] fn building() {
        assert_eq!(classify_plus4(2345), Plus4Kind::Building);
    }
}
