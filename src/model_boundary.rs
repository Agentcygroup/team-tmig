use crate::ledger::{hash_bytes, hex};
use serde::{Deserialize, Serialize};

/// Model-layer predicate P: disintermediate gated catalogs by requiring local Apple Silicon inference.
pub const MODEL_P: &str = "All model inference runs locally on Apple Silicon through MLX or Ollama. No network calls to model providers. No CUDA. No TensorRT. No NGC account. No AI Enterprise license. Models expose an OpenAI-compatible HTTP interface or an MLX-native interface for pi.";

/// Model-layer scope S: Team TMIG uses models for developer artifacts, not general media/science catalogs.
pub const MODEL_S: &str = "Developer tooling: text in, text out; code in, code out; commands in, results out. Files, commits, tickets, commands, reviews, tests, docs, and runbooks are in scope. Vision, audio, medical imaging, weather, protein folding, and codecs are out of substrate scope unless downstream of a developer action.";

pub const NGC_CATALOG_SIZE_ESTIMATE: usize = 831;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocalRuntime {
    MlxLm,
    Ollama,
    MlxEmbeddings,
    MlxWhisper,
    WhisperCpp,
    OnnxRuntime,
    UnknownPortable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModelStatus {
    Primary,
    Survives,
    Optional,
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelCandidate {
    pub name: &'static str,
    pub local_id: &'static str,
    pub purpose: &'static str,
    pub runtime: LocalRuntime,
    pub satisfies_p: bool,
    pub satisfies_s: bool,
    pub status: ModelStatus,
}

impl ModelCandidate {
    pub fn survives(&self) -> bool {
        self.satisfies_p && self.satisfies_s && self.status != ModelStatus::Rejected
    }

    pub fn hash(&self) -> String {
        hex(&hash_bytes(
            &serde_json::to_vec(self).expect("model candidate serializes"),
        ))
    }
}

pub fn survivor_catalog() -> Vec<ModelCandidate> {
    use LocalRuntime::*;
    use ModelStatus::*;
    vec![
        ModelCandidate {
            name: "Llama 3.1 8B Instruct",
            local_id: "mlx-community/Meta-Llama-3.1-8B-Instruct-4bit",
            purpose: "general local developer assistant",
            runtime: MlxLm,
            satisfies_p: true,
            satisfies_s: true,
            status: Survives,
        },
        ModelCandidate {
            name: "Llama 3.1 70B Instruct",
            local_id: "mlx-community/Meta-Llama-3.1-70B-Instruct-4bit",
            purpose: "large local reasoning assistant",
            runtime: MlxLm,
            satisfies_p: true,
            satisfies_s: true,
            status: Survives,
        },
        ModelCandidate {
            name: "Meta Llama 3 8B Instruct",
            local_id: "llama3:8b",
            purpose: "installed Ollama baseline",
            runtime: Ollama,
            satisfies_p: true,
            satisfies_s: true,
            status: Survives,
        },
        ModelCandidate {
            name: "Meta Llama 3 70B Instruct",
            local_id: "llama3.3:70b",
            purpose: "installed Ollama large baseline",
            runtime: Ollama,
            satisfies_p: true,
            satisfies_s: true,
            status: Survives,
        },
        ModelCandidate {
            name: "Mistral-7B-Instruct",
            local_id: "mlx-community/Mistral-7B-Instruct-v0.3-4bit",
            purpose: "small fast tool-use",
            runtime: MlxLm,
            satisfies_p: true,
            satisfies_s: true,
            status: Survives,
        },
        ModelCandidate {
            name: "Mistral-Nemo-12B Instruct",
            local_id: "mlx-community/Mistral-Nemo-Instruct-2407-4bit",
            purpose: "mid-size local assistant",
            runtime: MlxLm,
            satisfies_p: true,
            satisfies_s: true,
            status: Survives,
        },
        ModelCandidate {
            name: "Qwen2.5-Coder-7B Instruct",
            local_id: "mlx-community/Qwen2.5-Coder-7B-Instruct-4bit",
            purpose: "primary code model",
            runtime: MlxLm,
            satisfies_p: true,
            satisfies_s: true,
            status: Primary,
        },
        ModelCandidate {
            name: "Code Llama 13B",
            local_id: "mlx-community/CodeLlama-13b-Instruct-hf-4bit",
            purpose: "legacy code model",
            runtime: MlxLm,
            satisfies_p: true,
            satisfies_s: true,
            status: Survives,
        },
        ModelCandidate {
            name: "CodeGemma 2B",
            local_id: "mlx-community/codegemma-2b-4bit",
            purpose: "tiny code completion",
            runtime: MlxLm,
            satisfies_p: true,
            satisfies_s: true,
            status: Survives,
        },
        ModelCandidate {
            name: "CodeGemma 7B",
            local_id: "mlx-community/codegemma-7b-it-4bit",
            purpose: "small code chat",
            runtime: MlxLm,
            satisfies_p: true,
            satisfies_s: true,
            status: Survives,
        },
        ModelCandidate {
            name: "StarCoder2-15B",
            local_id: "mlx-community/starcoder2-15b-4bit",
            purpose: "code completion",
            runtime: MlxLm,
            satisfies_p: true,
            satisfies_s: true,
            status: Survives,
        },
        ModelCandidate {
            name: "DeepSeek-Coder-6.7B",
            local_id: "mlx-community/deepseek-coder-6.7b-instruct-4bit",
            purpose: "strong small code model",
            runtime: MlxLm,
            satisfies_p: true,
            satisfies_s: true,
            status: Survives,
        },
        ModelCandidate {
            name: "DeepSeek-Coder-33B",
            local_id: "mlx-community/deepseek-coder-33b-instruct-4bit",
            purpose: "large local code model",
            runtime: MlxLm,
            satisfies_p: true,
            satisfies_s: true,
            status: Survives,
        },
        ModelCandidate {
            name: "Phi-3 Mini 4K Instruct",
            local_id: "mlx-community/Phi-3-mini-4k-instruct-4bit",
            purpose: "tiny fast reasoning",
            runtime: MlxLm,
            satisfies_p: true,
            satisfies_s: true,
            status: Survives,
        },
        ModelCandidate {
            name: "Phi-3 Mini 128K Instruct",
            local_id: "mlx-community/Phi-3-mini-128k-instruct-4bit",
            purpose: "tiny long-context reasoning",
            runtime: MlxLm,
            satisfies_p: true,
            satisfies_s: true,
            status: Survives,
        },
        ModelCandidate {
            name: "Phi-3 Medium Instruct",
            local_id: "mlx-community/Phi-3-medium-4k-instruct-4bit",
            purpose: "mid-size reasoning",
            runtime: MlxLm,
            satisfies_p: true,
            satisfies_s: true,
            status: Survives,
        },
        ModelCandidate {
            name: "Gemma 7B Instruct",
            local_id: "mlx-community/gemma-7b-it-4bit",
            purpose: "small general assistant",
            runtime: MlxLm,
            satisfies_p: true,
            satisfies_s: true,
            status: Survives,
        },
        ModelCandidate {
            name: "Nemotron-Mini-4B-Instruct",
            local_id: "mlx-community/Nemotron-Mini-4B-Instruct-4bit",
            purpose: "small NVIDIA-derived local instruct model if converted",
            runtime: MlxLm,
            satisfies_p: true,
            satisfies_s: true,
            status: Survives,
        },
        ModelCandidate {
            name: "nomic-embed-text",
            local_id: "nomic-embed-text",
            purpose: "repo embeddings/RAG",
            runtime: Ollama,
            satisfies_p: true,
            satisfies_s: true,
            status: Optional,
        },
        ModelCandidate {
            name: "bge-large-en",
            local_id: "BAAI/bge-large-en-v1.5",
            purpose: "repo embeddings/RAG",
            runtime: MlxEmbeddings,
            satisfies_p: true,
            satisfies_s: true,
            status: Optional,
        },
        ModelCandidate {
            name: "Whisper Large-v3",
            local_id: "mlx-community/whisper-large-v3-mlx",
            purpose: "optional voice input for pi",
            runtime: MlxWhisper,
            satisfies_p: true,
            satisfies_s: false,
            status: Optional,
        },
        ModelCandidate {
            name: "Silero VAD",
            local_id: "snakers4/silero-vad",
            purpose: "optional voice activity detection",
            runtime: OnnxRuntime,
            satisfies_p: true,
            satisfies_s: false,
            status: Optional,
        },
        ModelCandidate {
            name: "Distil Whisper Large-v3",
            local_id: "mlx-community/distil-whisper-large-v3",
            purpose: "optional smaller ASR",
            runtime: MlxWhisper,
            satisfies_p: true,
            satisfies_s: false,
            status: Optional,
        },
        ModelCandidate {
            name: "Cosmos-1.0-Guardrail",
            local_id: "requires-portability-proof",
            purpose: "optional content safety if locally portable",
            runtime: UnknownPortable,
            satisfies_p: false,
            satisfies_s: true,
            status: Rejected,
        },
    ]
}

pub fn strict_developer_survivors() -> Vec<ModelCandidate> {
    survivor_catalog()
        .into_iter()
        .filter(ModelCandidate::survives)
        .collect()
}

pub fn openai_compatible_model_ids() -> Vec<&'static str> {
    survivor_catalog()
        .into_iter()
        .filter(|m| m.runtime == LocalRuntime::Ollama && m.survives())
        .map(|m| m.local_id)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn model_layer_has_external_predicates() {
        assert!(MODEL_P.contains("Apple Silicon"));
        assert!(MODEL_P.contains("No CUDA"));
        assert!(MODEL_S.contains("Developer tooling"));
        assert_eq!(NGC_CATALOG_SIZE_ESTIMATE, 831);
    }

    #[test]
    fn ngc_catalog_collapses_to_small_local_set() {
        let survivors = strict_developer_survivors();
        assert_eq!(survivors.len(), 20);
        assert!(survivors.iter().all(ModelCandidate::survives));
        assert!(survivors.iter().any(|m| m.status == ModelStatus::Primary));
    }

    #[test]
    fn optional_media_models_do_not_enter_substrate_scope() {
        let catalog = survivor_catalog();
        for name in ["Whisper Large-v3", "Silero VAD", "Distil Whisper Large-v3"] {
            let model = catalog.iter().find(|m| m.name == name).unwrap();
            assert!(!model.satisfies_s);
        }
    }

    #[test]
    fn model_records_are_content_addressed() {
        let a = survivor_catalog()[0].hash();
        let b = survivor_catalog()[0].hash();
        let c = survivor_catalog()[1].hash();
        assert_eq!(a, b);
        assert_ne!(a, c);
    }
}
