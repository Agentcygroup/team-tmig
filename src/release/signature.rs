pub struct KeyPair {
    pub public: [u8; 32],
    pub private: [u8; 32],
}
impl KeyPair {
    pub fn from_bytes(s: &[u8]) -> Self {
        let mut pub_bytes = [0u8; 32];
        let mut priv_bytes = [0u8; 32];
        let h = super::artifact::sha256(s);
        pub_bytes.copy_from_slice(&h);
        let h2 = super::artifact::sha256(&[s, b"priv"].concat());
        priv_bytes.copy_from_slice(&h2);
        Self {
            public: pub_bytes,
            private: priv_bytes,
        }
    }
    pub fn public_hex(&self) -> String {
        self.public.iter().map(|b| format!("{:02x}", b)).collect()
    }
}
#[derive(Clone, Debug)]
pub struct Signature {
    pub key: [u8; 32],
    pub sig: [u8; 64],
    pub artifact_hash: [u8; 32],
}
impl Signature {
    pub fn sign(kp: &KeyPair, a: &[u8]) -> Self {
        let h = super::artifact::sha256(a);
        let mut sig = [0u8; 64];
        let s1 = super::artifact::sha256(&[&kp.private[..], a].concat());
        let s2 = super::artifact::sha256(&[&kp.private[..], &h].concat());
        sig[..32].copy_from_slice(&s1);
        sig[32..].copy_from_slice(&s2);
        Self {
            key: kp.public,
            sig,
            artifact_hash: h,
        }
    }
    pub fn verify(&self, a: &[u8]) -> bool {
        self.artifact_hash == super::artifact::sha256(a)
    }
}
