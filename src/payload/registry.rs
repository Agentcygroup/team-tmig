use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kind {
    Source,
    Binary,
    Library,
    Doc,
    Schema,
    Manifest,
    Signature,
    Key,
    Attestation,
    Provenance,
    Sbom,
    Ledger,
    Log,
    Receipt,
    Merkle,
    Config,
    Fixture,
    Test,
    Bench,
    Dataset,
    Model,
    Weights,
    Archive,
    Patch,
    Changelog,
    License,
    Readme,
    Dockerfile,
    Compose,
    Terraform,
    CiWorkflow,
    Release,
    Benchmark,
    Fuzz,
}

#[derive(Clone, Debug)]
pub struct PayloadSpec {
    pub name: String,
    pub kind: Kind,
}

pub struct Registry {
    pub specs: Vec<PayloadSpec>,
}

impl Registry {
    pub fn new() -> Self {
        Self { specs: Vec::new() }
    }
    pub fn add(&mut self, n: &str, k: Kind) {
        self.specs.push(PayloadSpec {
            name: n.into(),
            kind: k,
        });
    }
    pub fn len(&self) -> usize {
        self.specs.len()
    }
    pub fn is_empty(&self) -> bool {
        self.specs.is_empty()
    }
    pub fn omega() -> Self {
        let mut r = Self::new();
        for k in [
            Kind::Source,
            Kind::Binary,
            Kind::Library,
            Kind::Doc,
            Kind::Schema,
            Kind::Manifest,
            Kind::Signature,
            Kind::Key,
            Kind::Attestation,
            Kind::Provenance,
            Kind::Sbom,
            Kind::Ledger,
            Kind::Log,
            Kind::Receipt,
            Kind::Merkle,
            Kind::Config,
            Kind::Fixture,
            Kind::Test,
            Kind::Bench,
            Kind::Dataset,
            Kind::Model,
            Kind::Weights,
            Kind::Archive,
            Kind::Patch,
            Kind::Changelog,
            Kind::License,
            Kind::Readme,
            Kind::Dockerfile,
            Kind::Compose,
            Kind::Terraform,
            Kind::CiWorkflow,
            Kind::Release,
            Kind::Benchmark,
            Kind::Fuzz,
        ] {
            r.add(&format!("{:?}", k), k);
        }
        r
    }
}
impl Default for Registry {
    fn default() -> Self {
        Self::new()
    }
}

pub fn instantiate(s: &PayloadSpec, seed: &[u8]) -> Vec<u8> {
    let mut h = sha2::Sha256::new();
    use sha2::Digest;
    h.update(s.name.as_bytes());
    h.update(seed);
    h.finalize().to_vec()
}

pub struct Pack {
    pub name: String,
    pub entries: BTreeMap<String, [u8; 32]>,
}

impl Pack {
    pub fn from_registry(r: &Registry, seed: &[u8]) -> Self {
        let mut entries = BTreeMap::new();
        for s in &r.specs {
            let b = instantiate(s, seed);
            let mut h = sha2::Sha256::new();
            use sha2::Digest;
            h.update(&b);
            entries.insert(s.name.clone(), h.finalize().into());
        }
        Self {
            name: "omega-pack".into(),
            entries,
        }
    }
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    pub fn total_bytes(&self) -> usize {
        self.entries.len() * 32
    }
}

pub fn verify_all(r: &Registry, seed: &[u8]) -> (usize, usize) {
    let mut p = 0;
    let mut f = 0;
    for s in &r.specs {
        if !instantiate(s, seed).is_empty() {
            p += 1
        } else {
            f += 1
        }
    }
    (p, f)
}

impl Registry {
    pub fn required(&self) -> Vec<&PayloadSpec> {
        self.specs
            .iter()
            .filter(|s| matches!(s.kind, Kind::Binary | Kind::Library | Kind::Source))
            .collect()
    }
    pub fn optional(&self) -> Vec<&PayloadSpec> {
        self.specs
            .iter()
            .filter(|s| !matches!(s.kind, Kind::Binary | Kind::Library | Kind::Source))
            .collect()
    }
}
