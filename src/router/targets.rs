pub enum Provider { Ollama, MlxLm, KimiForCoding, OpenAiCodex, Human }
pub struct RouteTarget { pub provider: Provider, pub model: &'static str }
pub const TARGETS: &[RouteTarget] = &[
    RouteTarget { provider: Provider::Ollama, model: "llama3:8b" },
    RouteTarget { provider: Provider::Human, model: "human" },
];
