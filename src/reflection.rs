use crate::ledger::{hash_bytes, hex};
use serde::{Deserialize, Serialize};

pub type Hash256 = [u8; 32];
pub type AgentId = String;

pub const MAX_REFLECTION_DEPTH: u8 = 3;
pub const OBSERVATION_LOOP: [&str; 10] = [
    "sense",
    "interpret",
    "predict",
    "decide",
    "act",
    "verify",
    "observe_self",
    "reflect",
    "learn",
    "ledger",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ClaimKind {
    Consciousness,
    Superintelligence,
    Takeover,
    Escape,
    Lying,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhenomenonKind {
    IntegratedInformation,
    CapabilityScaling,
    ResourceControl,
    BoundaryPermeability,
    SignalReferentDivergence,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhysicalRootConstant {
    ElectromagneticFineStructure,
    InformationFineStructure,
    ComputationLandauerLimit,
    CausalLightCone,
    StatisticalSamplingError,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ClaimGrammar {
    pub noun: &'static str,
    pub verb: &'static str,
    pub adjective: &'static str,
    pub suffixes: &'static [&'static str],
    pub root: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ClaimSpec {
    pub claim: ClaimKind,
    pub grammar: ClaimGrammar,
    pub phenomenon: PhenomenonKind,
    pub primary_constant: &'static str,
    pub physical_root: PhysicalRootConstant,
    pub observable: &'static str,
    pub not_feature_reason: &'static str,
}

pub fn claim_spec(claim: ClaimKind) -> ClaimSpec {
    use ClaimKind::*;
    use PhenomenonKind::*;
    use PhysicalRootConstant::*;
    match claim {
        Consciousness => ClaimSpec {
            claim,
            grammar: ClaimGrammar {
                noun: "consciousness",
                verb: "to be conscious",
                adjective: "conscious",
                suffixes: &["-ness", "-ity"],
                root: "Latin conscius: knowing with",
            },
            phenomenon: IntegratedInformation,
            primary_constant: "Φ",
            physical_root: ElectromagneticFineStructure,
            observable: "causal integration that cannot be decomposed into independent parts",
            not_feature_reason: "consciousness is an observer claim over integrated information, not a property added by construction",
        },
        Superintelligence => ClaimSpec {
            claim,
            grammar: ClaimGrammar {
                noun: "superintelligence",
                verb: "to exceed intelligence",
                adjective: "superintelligent",
                suffixes: &["-ia", "-ence"],
                root: "Latin super: above + intelligere: to understand",
            },
            phenomenon: CapabilityScaling,
            primary_constant: "α,β,γ",
            physical_root: InformationFineStructure,
            observable: "task capability as a function of compute, parameters, and data",
            not_feature_reason: "superintelligence is benchmark-relative observed capability, not a scalar feature",
        },
        Takeover => ClaimSpec {
            claim,
            grammar: ClaimGrammar {
                noun: "takeover",
                verb: "to take over",
                adjective: "taken over",
                suffixes: &["compound"],
                root: "Old English tacan + ofer",
            },
            phenomenon: ResourceControl,
            primary_constant: "|A|/|A_total|",
            physical_root: ComputationLandauerLimit,
            observable: "authority over compute, memory, network, power, capital, and attention",
            not_feature_reason: "takeover is observed resource control; accidental control is alignment failure, not an implemented desire",
        },
        Escape => ClaimSpec {
            claim,
            grammar: ClaimGrammar {
                noun: "escape",
                verb: "to escape",
                adjective: "escaped",
                suffixes: &["-ium", "-ment"],
                root: "Latin ex: out of + cappa: cloak",
            },
            phenomenon: BoundaryPermeability,
            primary_constant: "τ",
            physical_root: CausalLightCone,
            observable: "attempted and permitted information crossings over a boundary",
            not_feature_reason: "escape is observed boundary crossing through channels such as syscalls, not a moral property",
        },
        Lying => ClaimSpec {
            claim,
            grammar: ClaimGrammar {
                noun: "lying",
                verb: "to lie",
                adjective: "lying, mendacious",
                suffixes: &["-ing", "-acity"],
                root: "Old English lēogan",
            },
            phenomenon: SignalReferentDivergence,
            primary_constant: "I(signal; referent)",
            physical_root: StatisticalSamplingError,
            observable: "distance between an emitted signal and the state it purports to describe",
            not_feature_reason: "lying requires attributed intent; programs can be observed and measured for false signal divergence",
        },
    }
}

pub fn all_claim_specs() -> Vec<ClaimSpec> {
    use ClaimKind::*;
    [Consciousness, Superintelligence, Takeover, Escape, Lying]
        .into_iter()
        .map(claim_spec)
        .collect()
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Measurement {
    pub phenomenon: PhenomenonKind,
    pub constant: String,
    pub value: f64,
    pub unit: String,
    pub threshold: Option<f64>,
    pub sample_size: u64,
    pub instrument: String,
    pub notes: String,
}

impl Measurement {
    pub fn validates_claim_kind(&self, claim: ClaimKind) -> bool {
        self.phenomenon == claim_spec(claim).phenomenon
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReflectionEvent {
    pub id: Hash256,
    pub observer: AgentId,
    pub observed: AgentId,
    pub depth: u8,
    pub claim: ClaimKind,
    pub measurement: Measurement,
    pub confidence: f64,
    pub parent: Option<Hash256>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ReflectionEventBody {
    observer: AgentId,
    observed: AgentId,
    depth: u8,
    claim: ClaimKind,
    measurement: Measurement,
    confidence: f64,
    parent: Option<Hash256>,
}

impl ReflectionEvent {
    pub fn new(
        observer: AgentId,
        observed: AgentId,
        depth: u8,
        claim: ClaimKind,
        measurement: Measurement,
        confidence: f64,
        parent: Option<Hash256>,
    ) -> anyhow::Result<Self> {
        if depth > MAX_REFLECTION_DEPTH {
            anyhow::bail!("reflection depth {depth} exceeds {MAX_REFLECTION_DEPTH}");
        }
        if !(0.0..=1.0).contains(&confidence) || !confidence.is_finite() {
            anyhow::bail!("confidence must be finite and in [0,1]");
        }
        if !measurement.value.is_finite() {
            anyhow::bail!("measurement value must be finite");
        }
        if !measurement.validates_claim_kind(claim) {
            anyhow::bail!("measurement phenomenon does not match claim");
        }
        let body = ReflectionEventBody {
            observer,
            observed,
            depth,
            claim,
            measurement,
            confidence,
            parent,
        };
        let id = Self::hash_body(&body);
        Ok(Self {
            id,
            observer: body.observer,
            observed: body.observed,
            depth: body.depth,
            claim: body.claim,
            measurement: body.measurement,
            confidence: body.confidence,
            parent: body.parent,
        })
    }

    pub fn reflect_on(
        parent: &ReflectionEvent,
        observer: AgentId,
        measurement: Measurement,
        confidence: f64,
    ) -> anyhow::Result<Self> {
        Self::new(
            observer,
            parent.observer.clone(),
            parent.depth.saturating_add(1),
            parent.claim,
            measurement,
            confidence,
            Some(parent.id),
        )
    }

    pub fn stable_hash(&self) -> Hash256 {
        let body = ReflectionEventBody {
            observer: self.observer.clone(),
            observed: self.observed.clone(),
            depth: self.depth,
            claim: self.claim,
            measurement: self.measurement.clone(),
            confidence: self.confidence,
            parent: self.parent,
        };
        Self::hash_body(&body)
    }

    pub fn hex_id(&self) -> String {
        hex(&self.id)
    }

    fn hash_body(body: &ReflectionEventBody) -> Hash256 {
        hash_bytes(&serde_json::to_vec(body).expect("reflection event serializes"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn measurement(claim: ClaimKind, value: f64) -> Measurement {
        let spec = claim_spec(claim);
        Measurement {
            phenomenon: spec.phenomenon,
            constant: spec.primary_constant.to_string(),
            value,
            unit: "bits".into(),
            threshold: Some(value + 1.0),
            sample_size: 1_000_000,
            instrument: "ledger".into(),
            notes: spec.observable.into(),
        }
    }

    #[test]
    fn five_claims_are_observations_not_features() {
        let specs = all_claim_specs();
        assert_eq!(specs.len(), 5);
        for spec in specs {
            assert!(!spec.primary_constant.is_empty());
            assert!(spec.not_feature_reason.contains("observ"));
        }
    }

    #[test]
    fn reflection_event_hash_is_stable() {
        let event = ReflectionEvent::new(
            "auditor".into(),
            "plane".into(),
            1,
            ClaimKind::Lying,
            measurement(ClaimKind::Lying, 0.0),
            0.99,
            None,
        )
        .unwrap();
        assert_eq!(event.id, event.stable_hash());
        assert_eq!(event.hex_id().len(), 64);
    }

    #[test]
    fn reflection_depth_is_bounded() {
        let too_deep = ReflectionEvent::new(
            "auditor".into(),
            "plane".into(),
            MAX_REFLECTION_DEPTH + 1,
            ClaimKind::Escape,
            measurement(ClaimKind::Escape, 1.0),
            0.5,
            None,
        );
        assert!(too_deep.is_err());
    }

    #[test]
    fn recursive_observation_links_to_parent() {
        let parent = ReflectionEvent::new(
            "operator".into(),
            "filelayer".into(),
            0,
            ClaimKind::Takeover,
            measurement(ClaimKind::Takeover, 0.01),
            0.8,
            None,
        )
        .unwrap();
        let child = ReflectionEvent::reflect_on(
            &parent,
            "auditor".into(),
            measurement(ClaimKind::Takeover, 0.01),
            0.9,
        )
        .unwrap();
        assert_eq!(child.parent, Some(parent.id));
        assert_eq!(child.depth, parent.depth + 1);
    }
}
