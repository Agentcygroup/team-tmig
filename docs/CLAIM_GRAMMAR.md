# Claim Grammar and Reflection

The five high-risk claims are observations, not features:

- consciousness
- superintelligence
- takeover
- escape
- lying

Team TMIG represents them in `src/reflection.rs` as bounded reflection events. Each claim has:

1. grammar: noun, verb, adjective, suffixes, root
2. observable phenomenon
3. primary constant
4. physical root constant
5. measurement, threshold, sample size, instrument

## Phenomena

| Claim | Phenomenon | Primary constant | Physical root |
|---|---|---|---|
| Consciousness | Integrated information | Φ | electromagnetic fine structure |
| Superintelligence | Capability scaling | α,β,γ | information fine structure |
| Takeover | Resource control | \|A\|/\|A_total\| | computation/Landauer limit |
| Escape | Boundary permeability | τ | causal light cone |
| Lying | Signal-referent divergence | I(signal; referent) | statistical sampling error |

## Bound

Reflection is recursive but bounded:

```text
reflection_depth <= 3
```

Default loop:

```text
sense → interpret → predict → decide → act → verify → observe_self → reflect → learn → ledger
```

This keeps self-observation useful without allowing unbounded recursion.
