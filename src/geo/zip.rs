#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Zip {
    pub zip5: u32,
    pub plus4: Option<u32>,
}

impl Zip {
    pub fn new(zip5: u32) -> Option<Self> {
        if (1000..=99950).contains(&zip5) {
            Some(Self { zip5, plus4: None })
        } else {
            None
        }
    }

    pub fn with_plus4(mut self, p: u32) -> Option<Self> {
        if p > 9999 {
            return None;
        }
        self.plus4 = Some(p);
        Some(self)
    }

    pub fn sectional_center(&self) -> u32 {
        self.zip5 / 100
    }
    pub fn delivery_unit(&self) -> u32 {
        self.zip5 % 100
    }
}
