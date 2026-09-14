#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QualityAttr {
    FunctionalCorrectness,
    PerformanceEfficiency,
    Compatibility,
    Usability,
    Reliability,
    Security,
    Maintainability,
    Portability,
}

pub fn all_quality_attrs() -> &'static [QualityAttr] {
    &[
        QualityAttr::FunctionalCorrectness,
        QualityAttr::PerformanceEfficiency,
        QualityAttr::Compatibility,
        QualityAttr::Reliability,
        QualityAttr::Security,
        QualityAttr::Maintainability,
        QualityAttr::Portability,
    ]
}
