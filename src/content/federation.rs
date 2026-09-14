//! Stubs so bins compile. Replace bodies with real semantics later.

use crate::content::index::ContentIndex;
use crate::content::registry::{Content, ContentKind};

#[derive(Debug, Clone, Default)]
pub struct Federation {
    pub members: Vec<String>,
}

impl Federation {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn omega() -> Self {
        Self::default()
    }
    pub fn add(&mut self, m: impl Into<String>) {
        self.members.push(m.into());
    }
    pub fn len(&self) -> usize {
        self.members.len()
    }
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Render {
    pub kind: Option<ContentKind>,
    pub body: String,
}

impl Render {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn of(c: &Content) -> Self {
        Self {
            kind: Some(c.kind),
            body: String::from_utf8_lossy(&c.bytes).into_owned(),
        }
    }

    pub fn tsv(idx: &ContentIndex) -> String {
        let mut out = String::from("kind\tcount\n");
        for (k, v) in idx.by_kind.iter() {
            out.push_str(&format!("{:?}\t{}\n", k, v.len()));
        }
        out
    }

    pub fn json(idx: &ContentIndex) -> String {
        // hand-rolled, no serde dependency required
        let mut out = String::from("{\n");
        out.push_str(&format!("  \"total_bytes\": {},\n", idx.total_bytes));
        out.push_str(&format!("  \"kinds\": {},\n", idx.kinds()));
        out.push_str("  \"by_kind\": {\n");
        let n = idx.by_kind.len();
        for (i, (k, v)) in idx.by_kind.iter().enumerate() {
            let comma = if i + 1 < n { "," } else { "" };
            out.push_str(&format!("    \"{:?}\": {}{}\n", k, v.len(), comma));
        }
        out.push_str("  }\n}\n");
        out
    }
}
