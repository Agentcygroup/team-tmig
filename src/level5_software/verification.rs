#[derive(Clone, Debug)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
}

#[derive(Default, Debug)]
pub struct TestSuite {
    pub results: Vec<TestResult>,
}

impl TestSuite {
    pub fn new() -> Self { Self::default() }
    pub fn record(&mut self, name: impl Into<String>, passed: bool) {
        self.results.push(TestResult { name: name.into(), passed });
    }
    pub fn passed(&self) -> usize { self.results.iter().filter(|r| r.passed).count() }
    pub fn failed(&self) -> usize { self.results.iter().filter(|r| !r.passed).count() }
    pub fn total(&self) -> usize { self.results.len() }
}
