use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Scope {
    pub roots: Vec<String>,
    pub denylist: Vec<String>,
}

impl Scope {
    pub fn from_doctrine(_t: &str) -> Self {
        Self {
            roots: vec!["src/".into(), "tests/".into(), "docs/".into()],
            denylist: vec![".ssh".into(), "secret".into(), "token".into()],
        }
    }
    pub fn contains(&self, path: &str) -> Result<(), String> {
        for d in &self.denylist {
            if path.contains(d) {
                return Err(format!("denied: {}", d));
            }
        }
        for r in &self.roots {
            if path.starts_with(r) {
                return Ok(());
            }
        }
        Err(format!("oos: {}", path))
    }
}
