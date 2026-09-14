use std::collections::BTreeMap;

#[derive(Default)]
pub struct VersionRegistry {
    map: BTreeMap<String, Vec<String>>,
}

impl VersionRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn publish(&mut self, name: &str, version: &str) {
        self.map
            .entry(name.to_string())
            .or_default()
            .push(version.to_string());
    }
    pub fn latest(&self, name: &str) -> Option<&str> {
        self.map
            .get(name)
            .and_then(|v| v.last())
            .map(String::as_str)
    }
    pub fn all(&self, name: &str) -> Option<&[String]> {
        self.map.get(name).map(Vec::as_slice)
    }
}
