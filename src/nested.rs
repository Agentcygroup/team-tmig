pub const MAX_DEPTH: u8 = 3;

#[derive(Clone, Debug)]
pub struct Observation {
    pub id: [u8; 32],
    pub depth: u8,
}

#[derive(Clone, Debug)]
pub struct Reflection {
    pub id: [u8; 32],
    pub depth: u8,
}

#[derive(Clone, Debug)]
pub enum NestError {
    DepthExceeded,
}

pub struct Tower {
    pub obs: Vec<Observation>,
    pub refl: Vec<Reflection>,
    pub max_depth: u8,
}

impl Tower {
    pub fn new() -> Self {
        Self {
            obs: Vec::new(),
            refl: Vec::new(),
            max_depth: MAX_DEPTH,
        }
    }
    pub fn observe(&mut self, depth: u8) -> Result<(), NestError> {
        if depth > self.max_depth {
            return Err(NestError::DepthExceeded);
        }
        self.obs.push(Observation { id: [0; 32], depth });
        Ok(())
    }
    pub fn reflect(&mut self, depth: u8) -> Result<(), NestError> {
        if depth == 0 || depth > self.max_depth {
            return Err(NestError::DepthExceeded);
        }
        self.refl.push(Reflection { id: [0; 32], depth });
        Ok(())
    }
}

impl Default for Tower {
    fn default() -> Self {
        Self::new()
    }
}
