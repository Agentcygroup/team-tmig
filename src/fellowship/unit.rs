//! A 3-byte routing unit. 24 bits, 16 million possible values.
//!
//! Byte 0: consequence class index (0..5, or 255 for "unknown")
//! Byte 1: agent kind index       (0..7, or 255 for "unassigned")
//! Byte 2: layer index            (0..6, or 255 for "unassigned")

use crate::hitl::consequence::Consequence;
use crate::skills::AgentKind;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Unit(pub u8, pub u8, pub u8);

pub const UNASSIGNED: u8 = 255;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Layer {
    Substrate,
    Ledger,
    Primitives,
    Plane,
    Skills,
    Catalog,
    Doctrine,
}

impl Unit {
    pub fn new(consequence_idx: u8, agent_idx: u8, layer_idx: u8) -> Self {
        Unit(consequence_idx, agent_idx, layer_idx)
    }

    pub fn consequence(&self) -> Option<Consequence> {
        match self.0 {
            0 => Some(Consequence::None),
            1 => Some(Consequence::Low),
            2 => Some(Consequence::Moderate),
            3 => Some(Consequence::High),
            4 => Some(Consequence::Legal),
            5 => Some(Consequence::Irreversible),
            _ => None,
        }
    }

    pub fn agent(&self) -> Option<AgentKind> {
        match self.1 {
            0 => Some(AgentKind::Architect),
            1 => Some(AgentKind::Operator),
            2 => Some(AgentKind::Auditor),
            3 => Some(AgentKind::Adversary),
            4 => Some(AgentKind::Swarm),
            5 => Some(AgentKind::Jira),
            6 => Some(AgentKind::Plane),
            7 => Some(AgentKind::FileLayer),
            _ => None,
        }
    }

    pub fn layer(&self) -> Option<Layer> {
        match self.2 {
            0 => Some(Layer::Substrate),
            1 => Some(Layer::Ledger),
            2 => Some(Layer::Primitives),
            3 => Some(Layer::Plane),
            4 => Some(Layer::Skills),
            5 => Some(Layer::Catalog),
            6 => Some(Layer::Doctrine),
            _ => None,
        }
    }

    pub fn as_u24(&self) -> u32 {
        ((self.0 as u32) << 16) | ((self.1 as u32) << 8) | (self.2 as u32)
    }

    pub fn from_u24(v: u32) -> Self {
        Unit(
            ((v >> 16) & 0xFF) as u8,
            ((v >> 8) & 0xFF) as u8,
            (v & 0xFF) as u8,
        )
    }

    pub fn consequence_idx(c: Consequence) -> u8 {
        match c {
            Consequence::None => 0,
            Consequence::Low => 1,
            Consequence::Moderate => 2,
            Consequence::High => 3,
            Consequence::Legal => 4,
            Consequence::Irreversible => 5,
        }
    }

    pub fn agent_idx(a: AgentKind) -> u8 {
        match a {
            AgentKind::Architect => 0,
            AgentKind::Operator => 1,
            AgentKind::Auditor => 2,
            AgentKind::Adversary => 3,
            AgentKind::Swarm => 4,
            AgentKind::Jira => 5,
            AgentKind::Plane => 6,
            AgentKind::FileLayer => 7,
        }
    }

    pub fn layer_idx(l: Layer) -> u8 {
        match l {
            Layer::Substrate => 0,
            Layer::Ledger => 1,
            Layer::Primitives => 2,
            Layer::Plane => 3,
            Layer::Skills => 4,
            Layer::Catalog => 5,
            Layer::Doctrine => 6,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_is_3_bytes() {
        assert_eq!(std::mem::size_of::<Unit>(), 3);
    }

    #[test]
    fn round_trip_u24() {
        let u = Unit(2, 5, 6);
        assert_eq!(Unit::from_u24(u.as_u24()), u);
    }

    #[test]
    fn product_is_288() {
        assert_eq!(6 * 8 * 6, 288);
    }

    #[test]
    fn unassigned_is_none() {
        let u = Unit(UNASSIGNED, UNASSIGNED, UNASSIGNED);
        assert!(u.consequence().is_none());
        assert!(u.agent().is_none());
        assert!(u.layer().is_none());
    }
}
