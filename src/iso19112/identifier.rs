#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeographicIdentifier {
    pub authority: String,
    pub code: String,
    pub namespace: String,
}

impl GeographicIdentifier {
    pub fn new(authority: &str, namespace: &str, code: &str) -> Self {
        Self {
            authority: authority.into(),
            namespace: namespace.into(),
            code: code.into(),
        }
    }
    pub fn uri(&self) -> String {
        format!("geo:{}:{}:{}", self.authority, self.namespace, self.code)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn uri_format() {
        let g = GeographicIdentifier::new("usps", "zip5", "90024");
        assert_eq!(g.uri(), "geo:usps:zip5:90024");
    }
}
