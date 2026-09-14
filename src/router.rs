use crate::controlplane::{ActionProof, Attestation, ControlPlane};
use crate::hitl::consequence::Consequence;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Provider {
    Ollama,
    MlxLm,
    KimiForCoding,
    OpenAiCodex,
    Human,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct RouteTarget {
    pub provider: Provider,
    pub model: &'static str,
    pub quality: u32,
    pub cost: u32,
    pub latency: u32,
}

pub const TARGETS: &[RouteTarget] = &[
    RouteTarget {
        provider: Provider::Ollama,
        model: "llama3:8b",
        quality: 55,
        cost: 0,
        latency: 3,
    },
    RouteTarget {
        provider: Provider::MlxLm,
        model: "Qwen2.5-Coder",
        quality: 60,
        cost: 0,
        latency: 4,
    },
    RouteTarget {
        provider: Provider::Ollama,
        model: "llama3.3:70b",
        quality: 75,
        cost: 0,
        latency: 20,
    },
    RouteTarget {
        provider: Provider::KimiForCoding,
        model: "kimi-for-coding",
        quality: 80,
        cost: 2,
        latency: 3,
    },
    RouteTarget {
        provider: Provider::OpenAiCodex,
        model: "gpt-5.5",
        quality: 95,
        cost: 10,
        latency: 5,
    },
    RouteTarget {
        provider: Provider::Human,
        model: "human",
        quality: 100,
        cost: u32::MAX,
        latency: u32::MAX,
    },
];

pub fn is_dominated(t: &RouteTarget, others: &[RouteTarget]) -> bool {
    for o in others {
        if std::ptr::eq(t, o) {
            continue;
        }
        let q = o.quality >= t.quality;
        let c = o.cost <= t.cost;
        let l = o.latency <= t.latency;
        let s = o.quality > t.quality || o.cost < t.cost || o.latency < t.latency;
        if q && c && l && s {
            return true;
        }
    }
    false
}

pub fn pareto_set() -> Vec<RouteTarget> {
    TARGETS
        .iter()
        .filter(|t| !is_dominated(t, TARGETS))
        .copied()
        .collect()
}

pub fn route_for(c: Consequence) -> RouteTarget {
    match c {
        Consequence::None => TARGETS[0],
        Consequence::Low => TARGETS[1],
        Consequence::Moderate => TARGETS[2],
        Consequence::High => TARGETS[3],
        Consequence::Legal => TARGETS[4],
        Consequence::Irreversible => TARGETS[5],
    }
}

pub fn route(
    plane: &ControlPlane,
    proof: &ActionProof,
) -> Result<(RouteTarget, Attestation), String> {
    let att = plane.handle(proof).map_err(|e| format!("{:?}", e))?;
    let target = route_for(att.consequence);
    Ok((target, att))
}
