#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Target {
    GitHubReleases,
    CratesIo,
    DocsRs,
    Homebrew,
    Docker,
    Nix,
}

pub struct Distributor {
    pub targets: Vec<Target>,
}

impl Distributor {
    pub fn new() -> Self {
        Self {
            targets: Vec::new(),
        }
    }
    pub fn add(&mut self, t: Target) {
        self.targets.push(t);
    }
    pub fn count(&self) -> usize {
        self.targets.len()
    }
    pub fn is_multi_target(&self) -> bool {
        self.targets.len() >= 5
    }
}
impl Default for Distributor {
    fn default() -> Self {
        Self::new()
    }
}
