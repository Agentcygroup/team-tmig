//! Fellowship admission: bounded by Dunbar layer radius.

use crate::fellowship::fellow::Fellow;
use crate::fellowship::unit::Layer;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FellowshipError {
    LayerFull { layer: Layer, capacity: usize },
    DuplicateId,
}

pub const LAYER_RADIUS: [(Layer, usize); 7] = [
    (Layer::Substrate, 5),
    (Layer::Ledger, 15),
    (Layer::Primitives, 50),
    (Layer::Plane, 150),
    (Layer::Skills, 500),
    (Layer::Catalog, 1500),
    (Layer::Doctrine, 1500),
];

pub fn invite(members: &mut Vec<Fellow>, f: Fellow) -> Result<(), FellowshipError> {
    let cap = LAYER_RADIUS
        .iter()
        .find(|(l, _)| *l == f.layer)
        .map(|(_, n)| *n)
        .unwrap_or(1500);
    if members.iter().any(|m| m.id == f.id) {
        return Err(FellowshipError::DuplicateId);
    }
    let at_layer = members.iter().filter(|m| m.layer == f.layer).count();
    if at_layer >= cap {
        return Err(FellowshipError::LayerFull {
            layer: f.layer,
            capacity: cap,
        });
    }
    members.push(f);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::skills::AgentKind;

    fn f(name: &str, layer: Layer) -> Fellow {
        Fellow::new(name, AgentKind::Architect, [0u8; 32], layer)
    }

    #[test]
    fn substrate_is_capped() {
        let mut v = Vec::new();
        for i in 0..5 {
            invite(&mut v, f(&format!("f{}", i), Layer::Substrate)).unwrap();
        }
        let r = invite(&mut v, f("f5", Layer::Substrate));
        assert!(matches!(r, Err(FellowshipError::LayerFull { .. })));
    }

    #[test]
    fn duplicate_rejected() {
        let mut v = Vec::new();
        invite(&mut v, f("a", Layer::Ledger)).unwrap();
        let r = invite(&mut v, f("a", Layer::Ledger));
        assert_eq!(r, Err(FellowshipError::DuplicateId));
    }
}
