use crate::corpus::manifest::Manifest;
use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Axis {
    Class,
    Authority,
    Scope,
    Size,
    License,
    Domain,
    Cadence,
    Access,
    DType,
}

impl Axis {
    pub fn all() -> &'static [Axis] {
        &[
            Axis::Class,
            Axis::Authority,
            Axis::Scope,
            Axis::Size,
            Axis::License,
            Axis::Domain,
            Axis::Cadence,
            Axis::Access,
            Axis::DType,
        ]
    }
    pub fn name(&self) -> &'static str {
        match self {
            Axis::Class => "class",
            Axis::Authority => "authority",
            Axis::Scope => "scope",
            Axis::Size => "size",
            Axis::License => "license",
            Axis::Domain => "domain",
            Axis::Cadence => "cadence",
            Axis::Access => "access",
            Axis::DType => "dtype",
        }
    }
    pub fn value_of(&self, s: &crate::corpus::manifest::Source) -> String {
        match self {
            Axis::Class => s.class_.clone(),
            Axis::Authority => s.authority.clone(),
            Axis::Scope => s.scope.clone(),
            Axis::Size => s.size.clone(),
            Axis::License => s.license.clone(),
            Axis::Domain => s.domain.clone(),
            Axis::Cadence => s.cadence.clone(),
            Axis::Access => s.access.clone(),
            Axis::DType => s.dtype.clone(),
        }
    }
}

pub struct Spectral {
    pub matrix: BTreeMap<(String, String), usize>,
    pub axis_a: Axis,
    pub axis_b: Axis,
}

impl Spectral {
    pub fn compute(m: &Manifest, a: Axis, b: Axis) -> Self {
        let mut x = BTreeMap::new();
        for s in &m.sources {
            let va = a.value_of(s);
            let vb = b.value_of(s);
            *x.entry((va, vb)).or_insert(0) += 1;
        }
        Self {
            matrix: x,
            axis_a: a,
            axis_b: b,
        }
    }
    pub fn total(&self) -> usize {
        self.matrix.values().sum()
    }
    pub fn to_tsv(&self) -> String {
        let mut out = String::from(self.axis_a.name());
        out.push('\\');
        out.push_str(self.axis_b.name());
        out.push('\n');
        for ((a, b), v) in &self.matrix {
            out.push_str(&format!("{}\t{}\t{}\n", a, b, v));
        }
        out
    }
}
