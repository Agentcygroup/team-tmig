use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Json,
    Toml,
    Binary,
    Text,
}

#[derive(Clone, Debug)]
pub struct Schema {
    pub name: String,
    pub format: Format,
    pub version: u32,
}

#[derive(Default)]
pub struct SchemaRegistry {
    by_name: BTreeMap<String, Schema>,
}

impl SchemaRegistry {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn register(&mut self, s: Schema) {
        self.by_name.insert(s.name.clone(), s);
    }
    pub fn get(&self, name: &str) -> Option<&Schema> {
        self.by_name.get(name)
    }
    pub fn len(&self) -> usize {
        self.by_name.len()
    }
    pub fn is_empty(&self) -> bool {
        self.by_name.is_empty()
    }
}

pub fn all_schemas() -> Vec<Schema> {
    vec![
        Schema {
            name: "hitl-ledger".into(),
            format: Format::Json,
            version: 1,
        },
        Schema {
            name: "cargo-manifest".into(),
            format: Format::Toml,
            version: 1,
        },
        Schema {
            name: "audit-log".into(),
            format: Format::Json,
            version: 1,
        },
        Schema {
            name: "clause-set".into(),
            format: Format::Text,
            version: 1,
        },
    ]
}
