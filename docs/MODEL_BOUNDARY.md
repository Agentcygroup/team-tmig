# Model Boundary: Catalog Distillation, Not ML Distillation

The NVIDIA NGC catalog behaves like the feature wishlist: it is a dense graph. ASR, TTS, NMT, vision, medical imaging, weather, codecs, guardrails, and LLMs are all a few hops apart through shared architectures and runtimes. Combining them does not converge.

Team TMIG disintermediates the catalog by applying predicates from outside the graph.

## P: local model invariant

All model inference runs locally on Apple Silicon through MLX or Ollama. No network calls to model providers. No CUDA. No TensorRT. No NGC account. No AI Enterprise license. Models expose an OpenAI-compatible HTTP interface or an MLX-native interface for `pi`.

## S: model scope

Developer tooling: text in, text out; code in, code out; commands in, results out. Files, commits, tickets, commands, reviews, tests, docs, and runbooks are in scope. Vision, audio, medical imaging, weather, protein folding, and codecs are out of substrate scope unless downstream of a developer action.

## Result

The catalog filter is:

```text
M_local = { m in NGC/HF/Ollama : m satisfies MODEL_P ∩ MODEL_S }
```

For an 831-entry NGC-style catalog, the strict developer subset is approximately 20 local models. The executable manifest lives in `src/model_boundary.rs`; `bin/model-boundary.sh` emits a content-addressed JSON manifest. The current generated manifest is `docs/model-catalog.local.json`.

## Primary proof path

```bash
python3 -m pip install --user mlx-lm
mlx_lm.generate \
  --model mlx-community/Qwen2.5-Coder-7B-Instruct-4bit \
  --prompt "write a bash function that detects piped stdin"
```

If that prints code, the local MLX runtime works.

## pi local provider shape

`pi` reads custom local providers from `~/.pi/agent/models.json`.

Minimal Ollama provider:

```json
{
  "providers": {
    "ollama": {
      "baseUrl": "http://localhost:11434/v1",
      "api": "openai-completions",
      "apiKey": "ollama",
      "compat": {
        "supportsDeveloperRole": false,
        "supportsReasoningEffort": false
      },
      "models": [
        { "id": "llama3:8b" },
        { "id": "llama3.3:70b" }
      ]
    }
  }
}
```

This keeps the model layer inside the same boundary as the rest of Team TMIG: local, auditable, reversible, content-addressed, and developer-scoped.
