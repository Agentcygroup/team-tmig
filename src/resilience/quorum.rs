#[derive(Clone, Copy, Debug)]
pub struct Quorum {
    pub members: u32,
    pub required: u32,
}

impl Quorum {
    pub fn new(members: u32) -> Self {
        Self {
            members,
            required: members / 2 + 1,
        }
    }
    pub fn tolerates(&self) -> u32 {
        self.members - self.required
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nine_tolerates_four() {
        assert_eq!(Quorum::new(9).tolerates(), 4);
    }
}
