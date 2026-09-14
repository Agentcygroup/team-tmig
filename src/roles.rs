use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Role {
    pub name: String,
    #[serde(skip)]
    pub signing: Option<SigningKey>,
    pub verifying: [u8; 32],
}

impl Role {
    pub fn generate(name: &str) -> Self {
        let signing = SigningKey::generate(&mut OsRng);
        let verifying = signing.verifying_key().to_bytes();
        Self {
            name: name.to_string(),
            signing: Some(signing),
            verifying,
        }
    }
    pub fn signs(&self, hash: &[u8; 32]) -> Option<[u8; 64]> {
        let sk = self.signing.as_ref()?;
        Some(sk.sign(hash).to_bytes())
    }
    pub fn verifies(hash: &[u8; 32], sig: &[u8; 64], pubkey: &[u8; 32]) -> bool {
        let vk = match VerifyingKey::from_bytes(pubkey) {
            Ok(v) => v,
            Err(_) => return false,
        };
        let s = Signature::from_bytes(sig);
        vk.verify(hash, &s).is_ok()
    }
}

#[derive(Clone)]
pub struct TeamRoles {
    pub architect: Role,
    pub operator: Role,
    pub auditor: Role,
    pub adversary: Role,
}

impl TeamRoles {
    pub fn generate() -> Self {
        Self {
            architect: Role::generate("architect"),
            operator: Role::generate("operator"),
            auditor: Role::generate("auditor"),
            adversary: Role::generate("adversary"),
        }
    }
}
