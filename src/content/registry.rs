use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ContentKind {
    Sig,
    Key,
    Cert,
    Sbom,
    Asm,
    Unknown,
}

impl ContentKind {
    pub fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "sig" | "signature" => ContentKind::Sig,
            "key" => ContentKind::Key,
            "cert" | "crt" => ContentKind::Cert,
            "sbom" => ContentKind::Sbom,
            "asm" | "s" => ContentKind::Asm,
            _ => ContentKind::Unknown,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Content {
    pub name: String,
    pub kind: ContentKind,
    pub bytes: Vec<u8>,
    pub size: u64,
    pub source: String,
}

impl Content {
    pub fn new(name: &str, kind: ContentKind, bytes: &[u8], source: &str) -> Self {
        Self {
            name: name.to_string(),
            kind,
            bytes: bytes.to_vec(),
            size: bytes.len() as u64,
            source: source.to_string(),
        }
    }
}

pub fn sha256(b: &[u8]) -> String {
    Sha256::digest(b)
        .iter()
        .map(|x| format!("{:02x}", x))
        .collect()
}

pub fn all_kinds() -> Vec<ContentKind> {
    vec![
        ContentKind::Sig,
        ContentKind::Key,
        ContentKind::Cert,
        ContentKind::Sbom,
        ContentKind::Asm,
    ]
}

impl std::str::FromStr for ContentKind {
    type Err = std::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(ContentKind::parse(s))
    }
}
