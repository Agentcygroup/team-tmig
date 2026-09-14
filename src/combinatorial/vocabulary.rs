//! The four fixed vocabularies whose cross-product is the combinatorial
//! expansion: tags, primitives, metrics, layers. Each is closed: no term in
//! the composite space requires a fifth vocabulary.

use serde::{Deserialize, Serialize};

// ------------------------------------------------------------------ tags

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Concern {
    Structure,
    Behavior,
    State,
    Communication,
    Correctness,
    Performance,
    Security,
    Evolution,
    Meaning,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Mode {
    Type,
    Operation,
    Invariant,
    Property,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Tag {
    pub concern: Concern,
    pub mode: Mode,
}

impl Tag {
    pub const fn new(concern: Concern, mode: Mode) -> Self {
        Self { concern, mode }
    }
    pub fn short(&self) -> String {
        format!("{}:{}", short_concern(self.concern), short_mode(self.mode))
    }
}

pub const fn short_concern(c: Concern) -> &'static str {
    match c {
        Concern::Structure => "struct",
        Concern::Behavior => "behav",
        Concern::State => "state",
        Concern::Communication => "comm",
        Concern::Correctness => "correct",
        Concern::Performance => "perf",
        Concern::Security => "sec",
        Concern::Evolution => "evolve",
        Concern::Meaning => "meaning",
    }
}
pub const fn short_mode(m: Mode) -> &'static str {
    match m {
        Mode::Type => "type",
        Mode::Operation => "op",
        Mode::Invariant => "inv",
        Mode::Property => "prop",
    }
}

pub const CONCERNS: [Concern; 9] = [
    Concern::Structure,
    Concern::Behavior,
    Concern::State,
    Concern::Communication,
    Concern::Correctness,
    Concern::Performance,
    Concern::Security,
    Concern::Evolution,
    Concern::Meaning,
];
pub const MODES: [Mode; 4] = [Mode::Type, Mode::Operation, Mode::Invariant, Mode::Property];

pub fn all_tags() -> Vec<Tag> {
    let mut v = Vec::with_capacity(36);
    for c in CONCERNS {
        for m in MODES {
            v.push(Tag::new(c, m));
        }
    }
    v
}

// ------------------------------------------------------------------ primitives

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Primitive {
    Declaration,
    Consent,
    Retention,
    Disclosure,
    Provenance,
    Revocation,
}

impl Primitive {
    pub fn name(self) -> &'static str {
        match self {
            Primitive::Declaration => "Declaration",
            Primitive::Consent => "Consent",
            Primitive::Retention => "Retention",
            Primitive::Disclosure => "Disclosure",
            Primitive::Provenance => "Provenance",
            Primitive::Revocation => "Revocation",
        }
    }
}

pub const PRIMITIVES: [Primitive; 6] = [
    Primitive::Declaration,
    Primitive::Consent,
    Primitive::Retention,
    Primitive::Disclosure,
    Primitive::Provenance,
    Primitive::Revocation,
];

// ------------------------------------------------------------------ metrics

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MetricAspect {
    Count,
    Rate,
    Ratio,
    Distribution,
    Duration,
    Cost,
}

pub const METRIC_ASPECTS: [MetricAspect; 6] = [
    MetricAspect::Count,
    MetricAspect::Rate,
    MetricAspect::Ratio,
    MetricAspect::Distribution,
    MetricAspect::Duration,
    MetricAspect::Cost,
];

impl MetricAspect {
    pub fn name(self) -> &'static str {
        match self {
            MetricAspect::Count => "Count",
            MetricAspect::Rate => "Rate",
            MetricAspect::Ratio => "Ratio",
            MetricAspect::Distribution => "Distribution",
            MetricAspect::Duration => "Duration",
            MetricAspect::Cost => "Cost",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Metric {
    pub domain: Concern,
    pub aspect: MetricAspect,
}

impl Metric {
    pub const fn new(domain: Concern, aspect: MetricAspect) -> Self {
        Self { domain, aspect }
    }
    pub fn short(&self) -> String {
        format!("{}:{}", short_concern(self.domain), aspect_str(self.aspect))
    }
}

pub fn aspect_str(a: MetricAspect) -> &'static str {
    match a {
        MetricAspect::Count => "count",
        MetricAspect::Rate => "rate",
        MetricAspect::Ratio => "ratio",
        MetricAspect::Distribution => "dist",
        MetricAspect::Duration => "duration",
        MetricAspect::Cost => "cost",
    }
}

pub fn all_metrics() -> Vec<Metric> {
    let mut v = Vec::with_capacity(54);
    for d in CONCERNS {
        for a in METRIC_ASPECTS {
            v.push(Metric::new(d, a));
        }
    }
    v
}

// ------------------------------------------------------------------ layers

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Layer {
    Substrate,
    Ledger,
    Primitives,
    Plane,
    Skills,
    Catalog,
    Doctrine,
}

pub const LAYERS: [Layer; 7] = [
    Layer::Substrate,
    Layer::Ledger,
    Layer::Primitives,
    Layer::Plane,
    Layer::Skills,
    Layer::Catalog,
    Layer::Doctrine,
];

impl Layer {
    pub fn name(self) -> &'static str {
        match self {
            Layer::Substrate => "Substrate",
            Layer::Ledger => "Ledger",
            Layer::Primitives => "Primitives",
            Layer::Plane => "Plane",
            Layer::Skills => "Skills",
            Layer::Catalog => "Catalog",
            Layer::Doctrine => "Doctrine",
        }
    }
}

// ------------------------------------------------------------------ tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tag_count_is_36() {
        assert_eq!(all_tags().len(), 36);
    }
    #[test]
    fn primitive_count_is_6() {
        assert_eq!(PRIMITIVES.len(), 6);
    }
    #[test]
    fn metric_count_is_54() {
        assert_eq!(all_metrics().len(), 54);
    }
    #[test]
    fn layer_count_is_7() {
        assert_eq!(LAYERS.len(), 7);
    }
    #[test]
    fn syntactic_product_is_81648() {
        assert_eq!(
            all_tags().len() * PRIMITIVES.len() * all_metrics().len() * LAYERS.len(),
            81_648
        );
    }
}
