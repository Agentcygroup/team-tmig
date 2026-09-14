#[derive(Clone, Debug)]
pub struct Component {
    pub name: String,
    pub version: String,
    pub license: String,
}

#[derive(Clone, Debug)]
pub struct Sbom {
    pub format: String,
    pub components: Vec<Component>,
}

impl Sbom {
    pub fn cyclonedx(ns: Vec<String>) -> Self {
        Self {
            format: "CycloneDX-1.5".into(),
            components: ns
                .into_iter()
                .map(|n| Component {
                    name: n,
                    version: "*".into(),
                    license: "unknown".into(),
                })
                .collect(),
        }
    }
    pub fn to_json(&self) -> String {
        let items: Vec<String> = self
            .components
            .iter()
            .map(|c| format!("{{\"name\":\"{}\",\"version\":\"{}\"}}", c.name, c.version))
            .collect();
        format!(
            "{{\"format\":\"{}\",\"components\":[{}]}}",
            self.format,
            items.join(",")
        )
    }
    pub fn count(&self) -> usize {
        self.components.len()
    }
}
