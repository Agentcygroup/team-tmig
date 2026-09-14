//! Routing: (consequence, agent, layer) -> FellowKind.

use crate::fellowship::fellow::FellowKind;
use crate::fellowship::unit::Unit;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum RouteError {
    UnassignedConsequence,
    UnassignedAgent,
    UnassignedLayer,
}

pub fn route(unit: Unit) -> Result<FellowKind, RouteError> {
    let c = unit
        .consequence()
        .ok_or(RouteError::UnassignedConsequence)?;
    let a = unit.agent().ok_or(RouteError::UnassignedAgent)?;
    let l = unit.layer().ok_or(RouteError::UnassignedLayer)?;
    let _ = c;
    Ok(FellowKind { kind: a, layer: l })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fellowship::unit::Layer;
    use crate::skills::AgentKind;

    #[test]
    fn route_architect_ledger() {
        let u = Unit(
            2,
            Unit::agent_idx(AgentKind::Architect),
            Unit::layer_idx(Layer::Ledger),
        );
        let k = route(u).unwrap();
        assert_eq!(k.kind, AgentKind::Architect);
        assert_eq!(k.layer, Layer::Ledger);
    }

    #[test]
    fn route_fails_unassigned() {
        let u = Unit(255, 0, 0);
        assert!(route(u).is_err());
    }
}
