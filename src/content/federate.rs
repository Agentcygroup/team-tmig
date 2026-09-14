use std::collections::BTreeMap;

pub struct Federation { pub sources: BTreeMap<String, String> }

impl Federation {
    pub fn new() -> Self { Self { sources: BTreeMap::new() } }
    pub fn add(&mut self, n: &str, u: &str) { self.sources.insert(n.into(), u.into()); }
    pub fn len(&self) -> usize { self.sources.len() }
    pub fn is_empty(&self) -> bool { self.sources.is_empty() }
    pub fn omega() -> Self {
        let mut f = Self::new();
        for (n, u) in [
            ("iana", "https://www.iana.org/assignments/"),
            ("nist", "https://csrc.nist.gov/publications"),
            ("itu", "https://www.itu.int/rec/"),
            ("iso", "https://www.iso.org/obp"),
            ("iec", "https://webstore.iec.ch/"),
            ("ieee", "https://standards.ieee.org/"),
            ("ietf", "https://www.rfc-editor.org/rfc/"),
            ("w3c", "https://www.w3.org/TR/"),
            ("unicode", "https://www.unicode.org/Public/"),
            ("usps", "https://postalpro.usps.com/"),
            ("census", "https://www2.census.gov/geo/tiger/"),
            ("nanpa", "https://nationalnanpa.com/reports/"),
            ("sec", "https://www.sec.gov/Archives/edgar/"),
            ("worldbank", "https://api.worldbank.org/v2/"),
            ("imf", "https://dataservices.imf.org/"),
            ("arxiv", "https://arxiv.org/"),
            ("zenodo", "https://zenodo.org/api/"),
            ("osv", "https://api.osv.dev/v1/"),
            ("cyclonedx", "https://cyclonedx.org/"),
            ("spdx", "https://spdx.org/"),
            ("slsa", "https://slsa.dev/"),
            ("in-toto", "https://in-toto.io/"),
            ("sigstore", "https://sigstore.dev/"),
            ("oci", "https://github.com/opencontainers/"),
            ("cncf", "https://github.com/cncf/"),
            ("openssf", "https://github.com/ossf/"),
            ("cisa", "https://www.cisa.gov/"),
            ("nvd", "https://nvd.nist.gov/feeds/"),
            ("cve", "https://cve.mitre.org/"),
            ("epss", "https://api.first.org/data/v1/epss"),
        ] { f.add(n, u); }
        f
    }
}
impl Default for Federation { fn default() -> Self { Self::new() } }
