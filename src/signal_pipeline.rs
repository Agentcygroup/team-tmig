use crate::ledger::{hash_bytes, hex};
use serde::{Deserialize, Serialize};

/// Digital image processing is not admitted as a TMIG substrate feature.
/// Its in-scope value is the signal-processing grammar: operations over
/// content-addressed artifacts must declare whether they operate in a linear
/// physical domain or a perceptual/encoded domain.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SignalPrimitive {
    Classification,
    FeatureExtraction,
    MultiScaleSignalAnalysis,
    PatternRecognition,
    Projection,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalDomain {
    /// Physical/linear behavior: scaling, filtering, convolution, affine math.
    LinearRgbLike,
    /// Perceptual/encoded behavior: gamma-corrected r'g'b, Y'CbCr, lossy coding.
    PerceptualEncoded,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageTechnique {
    AnisotropicDiffusion,
    HiddenMarkovModel,
    ImageEditing,
    ImageRestoration,
    IndependentComponentAnalysis,
    LinearFiltering,
    NeuralNetwork,
    PartialDifferentialEquation,
    Pixelation,
    PointFeatureMatching,
    PrincipalComponentsAnalysis,
    SelfOrganizingMap,
    Wavelet,
    AffineTransform,
    MathematicalMorphology,
    HistogramEqualization,
}

impl ImageTechnique {
    pub fn primitives(self) -> &'static [SignalPrimitive] {
        use ImageTechnique::*;
        use SignalPrimitive::*;
        match self {
            AnisotropicDiffusion => &[MultiScaleSignalAnalysis, FeatureExtraction],
            HiddenMarkovModel => &[PatternRecognition, Classification],
            ImageEditing => &[Projection, FeatureExtraction],
            ImageRestoration => &[FeatureExtraction, MultiScaleSignalAnalysis],
            IndependentComponentAnalysis => &[Projection, FeatureExtraction],
            LinearFiltering => &[MultiScaleSignalAnalysis, FeatureExtraction],
            NeuralNetwork => &[FeatureExtraction, PatternRecognition, Classification],
            PartialDifferentialEquation => &[MultiScaleSignalAnalysis, Projection],
            Pixelation => &[Projection],
            PointFeatureMatching => &[FeatureExtraction, PatternRecognition],
            PrincipalComponentsAnalysis => &[Projection, FeatureExtraction],
            SelfOrganizingMap => &[Projection, Classification],
            Wavelet => &[MultiScaleSignalAnalysis, FeatureExtraction],
            AffineTransform => &[Projection],
            MathematicalMorphology => &[FeatureExtraction, PatternRecognition],
            HistogramEqualization => &[Projection, Classification],
        }
    }

    pub fn preferred_domain(self) -> SignalDomain {
        use ImageTechnique::*;
        match self {
            LinearFiltering
            | AffineTransform
            | AnisotropicDiffusion
            | PartialDifferentialEquation
            | MathematicalMorphology
            | Wavelet
            | ImageRestoration => SignalDomain::LinearRgbLike,
            HiddenMarkovModel
            | NeuralNetwork
            | Pixelation
            | HistogramEqualization
            | ImageEditing
            | IndependentComponentAnalysis
            | PointFeatureMatching
            | PrincipalComponentsAnalysis
            | SelfOrganizingMap => SignalDomain::PerceptualEncoded,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SignalTransformRecord {
    pub artifact_hash: String,
    pub technique: ImageTechnique,
    pub domain: SignalDomain,
    pub primitives: Vec<SignalPrimitive>,
}

impl SignalTransformRecord {
    pub fn new(artifact_hash: String, technique: ImageTechnique) -> Self {
        Self {
            artifact_hash,
            technique,
            domain: technique.preferred_domain(),
            primitives: technique.primitives().to_vec(),
        }
    }

    pub fn hash(&self) -> String {
        hex(&hash_bytes(
            &serde_json::to_vec(self).expect("signal transform record serializes"),
        ))
    }
}

pub fn all_primitives() -> Vec<SignalPrimitive> {
    use SignalPrimitive::*;
    vec![
        Classification,
        FeatureExtraction,
        MultiScaleSignalAnalysis,
        PatternRecognition,
        Projection,
    ]
}

pub fn all_techniques() -> Vec<ImageTechnique> {
    use ImageTechnique::*;
    vec![
        AnisotropicDiffusion,
        HiddenMarkovModel,
        ImageEditing,
        ImageRestoration,
        IndependentComponentAnalysis,
        LinearFiltering,
        NeuralNetwork,
        PartialDifferentialEquation,
        Pixelation,
        PointFeatureMatching,
        PrincipalComponentsAnalysis,
        SelfOrganizingMap,
        Wavelet,
        AffineTransform,
        MathematicalMorphology,
        HistogramEqualization,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_processing_collapses_to_five_signal_primitives() {
        assert_eq!(all_primitives().len(), 5);
        for technique in all_techniques() {
            assert!(!technique.primitives().is_empty(), "{:?}", technique);
        }
    }

    #[test]
    fn physical_linear_operations_stay_left_side() {
        assert_eq!(
            ImageTechnique::LinearFiltering.preferred_domain(),
            SignalDomain::LinearRgbLike
        );
        assert_eq!(
            ImageTechnique::AffineTransform.preferred_domain(),
            SignalDomain::LinearRgbLike
        );
        assert_eq!(
            ImageTechnique::MathematicalMorphology.preferred_domain(),
            SignalDomain::LinearRgbLike
        );
    }

    #[test]
    fn perceptual_operations_stay_right_side() {
        assert_eq!(
            ImageTechnique::Pixelation.preferred_domain(),
            SignalDomain::PerceptualEncoded
        );
        assert_eq!(
            ImageTechnique::NeuralNetwork.preferred_domain(),
            SignalDomain::PerceptualEncoded
        );
    }

    #[test]
    fn signal_records_are_content_addressed() {
        let a = SignalTransformRecord::new("0".repeat(64), ImageTechnique::Wavelet);
        let b = SignalTransformRecord::new("0".repeat(64), ImageTechnique::Wavelet);
        let c = SignalTransformRecord::new("0".repeat(64), ImageTechnique::AffineTransform);
        assert_eq!(a.hash(), b.hash());
        assert_ne!(a.hash(), c.hash());
    }
}
