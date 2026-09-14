#[derive(Clone, Debug, PartialEq)]
pub struct Cycle {
    pub vertices: Vec<u32>,
    pub closed: bool,
}

impl Cycle {
    pub fn new(vs: Vec<u32>) -> Self {
        let closed = vs.first() == vs.last() && vs.len() > 1;
        Self { vertices: vs, closed }
    }
    pub fn length(&self) -> usize { self.vertices.len() }
    pub fn is_closed(&self) -> bool { self.closed }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn closed_cycle() {
        let c = Cycle::new(vec![0, 1, 2, 0]);
        assert!(c.is_closed());
        assert_eq!(c.length(), 4);
    }
    #[test] fn open_cycle() {
        let c = Cycle::new(vec![0, 1, 2]);
        assert!(!c.is_closed());
    }
}
