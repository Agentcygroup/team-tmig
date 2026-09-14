#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AreaRegion {
    Northeast,
    Midwest,
    South,
    West,
    Pacific,
    Territory,
    TollFree,
}

pub fn region_of(code: u16) -> AreaRegion {
    match code {
        800 | 833 | 844 | 855 | 866 | 877 | 888 => AreaRegion::TollFree,
        201..=289 => AreaRegion::Northeast,
        304..=386 => AreaRegion::South,
        401..=419 => AreaRegion::Northeast,
        501..=587 => AreaRegion::Midwest,
        601..=689 => AreaRegion::South,
        701..=799 => AreaRegion::Midwest,
        801..=899 => AreaRegion::West,
        900..=999 => AreaRegion::Pacific,
        _ => AreaRegion::Territory,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn toll_free() {
        assert_eq!(region_of(800), AreaRegion::TollFree);
    }
    #[test] fn northeast() {
        assert_eq!(region_of(212), AreaRegion::Northeast);
    }
}
