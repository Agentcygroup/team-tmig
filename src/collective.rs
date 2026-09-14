use crate::router::Provider;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Vote {
    pub provider: Provider,
    pub answer: String,
    pub weight: f32,
}

pub const PRIOR_WEIGHT: f32 = 0.5;

pub fn accuracy_weight(o: &[bool]) -> f32 {
    if o.is_empty() {
        return PRIOR_WEIGHT;
    }
    let n = o.len() as f32;
    let k = o.iter().filter(|&&b| b).count() as f32;
    (k + 1.0) / (n + 2.0)
}

pub fn weighted_majority(vs: &[(Vote, f32)]) -> Option<String> {
    let total: f32 = vs.iter().map(|(_, w)| *w).sum();
    if total <= 0.0 {
        return None;
    }
    let mut m: BTreeMap<String, f32> = BTreeMap::new();
    for (v, w) in vs {
        *m.entry(v.answer.clone()).or_insert(0.0) += *w;
    }
    let (a, w) = m
        .into_iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())?;
    if w / total >= PRIOR_WEIGHT {
        Some(a)
    } else {
        None
    }
}
