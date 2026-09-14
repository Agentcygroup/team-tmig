use crate::controlplane::ActionProof;
use crate::hitl::consequence::Consequence;
use serde::{Deserialize, Serialize};

pub type Hash256 = [u8; 32];

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FlockPolarity {
    White,
    Black,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DunbarLayer {
    Org,
    Repo,
    Service,
    Subsystem,
    Module,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Attestation {
    pub action_hash: Hash256,
    pub doctrine_hash: Hash256,
    pub thesis_ok: bool,
    pub scope_ok: bool,
    pub consequence: Consequence,
    pub polarity: FlockPolarity,
    pub layer: DunbarLayer,
}

impl Attestation {
    pub fn from(p: &ActionProof, doctrine_hash: Hash256) -> Self {
        let c = p.consequence.unwrap_or(Consequence::None);
        let (pol, layer) = route_for(c);
        Self {
            action_hash: p.content_hash,
            doctrine_hash,
            thesis_ok: true,
            scope_ok: true,
            consequence: c,
            polarity: pol,
            layer,
        }
    }
}

pub fn route_for(c: Consequence) -> (FlockPolarity, DunbarLayer) {
    match c {
        Consequence::None => (FlockPolarity::White, DunbarLayer::Org),
        Consequence::Low => (FlockPolarity::White, DunbarLayer::Repo),
        Consequence::Moderate => (FlockPolarity::White, DunbarLayer::Service),
        Consequence::High => (FlockPolarity::White, DunbarLayer::Subsystem),
        Consequence::Legal => (FlockPolarity::White, DunbarLayer::Module),
        Consequence::Irreversible => (FlockPolarity::Black, DunbarLayer::Module),
    }
}
